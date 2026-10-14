"""Process demo: what a supervisor (an agent's code runner) asks of processes.

Children started after other threads have run and from those threads, waits
that block until a child exits, timeouts that kill, sessions and process
groups for killing everything a command started, and the errors a missing
program or process gives.  Every child is Python, so the plain python image
runs it.
"""
import os
import signal
import subprocess
import sys
import threading
import time

PY = sys.executable


def child(code, **kwargs):
    return subprocess.Popen([PY, "-c", code], **kwargs)


# 1. A child after a thread has run, and one from a thread: libc takes its
#    locks once a second thread has existed, so the child must run on its
#    parent's thread pointer, and a thread's stack is not the kernel's to
#    free.
t = threading.Thread(target=lambda: None)
t.start()
t.join()
r = subprocess.run([PY, "-c", "print('after')"], capture_output=True, text=True)
assert r.stdout == "after\n", r
out = []
t = threading.Thread(target=lambda: out.append(
    subprocess.run([PY, "-c", "print('from')"], capture_output=True, text=True).stdout))
t.start()
t.join()
assert out == ["from\n"], out
print("1. children after and from a thread")

# 2. wait() on a child that is still running returns when it exits.
t0 = time.monotonic()
p = child("import time; time.sleep(0.5)")
assert p.wait() == 0
assert time.monotonic() - t0 >= 0.5
print("2. wait() on a running child")

# 3. kill() and a timeout end a sleeping child at once, not when its sleep
#    would have.
p = child("import time; time.sleep(30)")
time.sleep(0.2)
t0 = time.monotonic()
p.kill()
assert p.wait() == -signal.SIGKILL
assert time.monotonic() - t0 < 5, time.monotonic() - t0
t0 = time.monotonic()
try:
    subprocess.run([PY, "-c", "import time; time.sleep(30)"], timeout=0.5)
    raise AssertionError("no timeout")
except subprocess.TimeoutExpired:
    pass
assert time.monotonic() - t0 < 5, time.monotonic() - t0
p = child("import time; time.sleep(30)")
time.sleep(0.2)
p.terminate()
assert p.wait() == -signal.SIGTERM
print("3. kill, terminate and timeouts are prompt")

# 4. A handled signal runs its handler and does not end the sleep.
p = child("import signal, time\n"
          "signal.signal(signal.SIGUSR1, lambda *a: print('usr1', flush=True))\n"
          "time.sleep(1)\n"
          "print('slept', flush=True)", stdout=subprocess.PIPE, text=True)
time.sleep(0.5)
p.send_signal(signal.SIGUSR1)
assert p.communicate()[0] == "usr1\nslept\n"
assert p.returncode == 0
print("4. a handled signal")

# 5. A new session and group for a command, and killpg() ending everything
#    in it: the command, and a child it left running in the background.
p = child("import subprocess, sys, time\n"
          "subprocess.Popen([sys.executable, '-c',\n"
          "                  'import time; time.sleep(1); print(\"alive\", flush=True)'])\n"
          "print('started', flush=True)\n"
          "time.sleep(30)", start_new_session=True, stdout=subprocess.PIPE, text=True)
assert p.stdout.readline() == "started\n"
assert os.getsid(p.pid) == p.pid and os.getpgid(p.pid) == p.pid
assert os.getsid(0) != p.pid and os.getpgrp() != p.pid
t0 = time.monotonic()
os.killpg(p.pid, signal.SIGKILL)
assert p.wait() == -signal.SIGKILL
# The background child held the pipe too: end of file means it died with
# the group, and before it could print.
assert p.stdout.read() == ""
assert time.monotonic() - t0 < 5, time.monotonic() - t0
print("5. a session's group killed with killpg")

# 6. setpgid() and waiting on a group.
p = child("import time; time.sleep(0.2)", process_group=0)
assert os.getpgid(p.pid) == p.pid
pid, status = os.waitpid(-p.pid, 0)
assert pid == p.pid and os.waitstatus_to_exitcode(status) == 0
print("6. waitpid on a process group")

# 7. The errors of a missing program and a missing process.
try:
    subprocess.run(["/no/such/program"])
    raise AssertionError("ran a missing program")
except FileNotFoundError:
    pass
for pid in (424242, 2**22, 2**31 - 1):
    try:
        os.kill(pid, 0)
        raise AssertionError(f"pid {pid} exists")
    except ProcessLookupError:
        pass
print("7. ENOENT and ESRCH")

# 8. A child that signals its own group, which is ours and init's: init is
#    spared (its death would end the guest), this process takes the signal
#    in its handler, and the child dies of it.
got = []
old_handler = signal.signal(signal.SIGTERM, lambda *a: got.append(1))
r = subprocess.run([PY, "-c", "import os, signal; os.kill(0, signal.SIGTERM)"])
signal.signal(signal.SIGTERM, old_handler)
assert os.getpgid(1) == os.getpgrp()
assert r.returncode == -signal.SIGTERM and got == [1], (r.returncode, got)
os.kill(1, 0)
print("8. a group signal spares init")

# 9. setsid() once, then never again as the leader it made us.
os.setsid()
assert os.getsid(0) == os.getpid() == os.getpgrp()
try:
    os.setsid()
    raise AssertionError("a group leader started a session")
except PermissionError:
    pass
print("9. setsid")

# 10. kill() ends a child blocked with no deadline, here in a read from a
#     pipe nothing will write to, and the read lets go of the pipe: its
#     writer sees the reader gone.  The same with the read in a second
#     thread of the child.
for code in ("import sys; print('reading', flush=True); sys.stdin.read()",
             "import sys, threading, time\n"
             "threading.Thread(target=sys.stdin.read).start()\n"
             "print('reading', flush=True); time.sleep(30)"):
    p = subprocess.Popen([PY, "-c", code], stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE, text=True)
    assert p.stdout.readline() == "reading\n"
    t0 = time.monotonic()
    p.kill()
    assert p.wait() == -signal.SIGKILL
    assert time.monotonic() - t0 < 5, time.monotonic() - t0
    try:
        p.stdin.write("x")
        p.stdin.flush()
        raise AssertionError("the killed reader still holds the pipe")
    except BrokenPipeError:
        pass
    try:
        p.stdin.close()
    except BrokenPipeError:
        pass
# SIGTERM to a child with two threads blocked reading two pipes: both
# reads let go, though the first thread to take the signal dequeues it;
# a child in select() with no timeout; and a SIGTERM a child sends one of
# its own threads with pthread_kill().
r, w = os.pipe()
p = subprocess.Popen([PY, "-c",
                      "import os, sys, threading\n"
                      "threading.Thread(target=sys.stdin.read).start()\n"
                      "print('reading', flush=True); os.read(%d, 1)" % r],
                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
                     pass_fds=(r,))
os.close(r)
assert p.stdout.readline() == "reading\n"
p.terminate()
assert p.wait() == -signal.SIGTERM
for write in (lambda: (p.stdin.write("x"), p.stdin.flush()),
              lambda: os.write(w, b"x")):
    try:
        write()
        raise AssertionError("a reader of the terminated child holds its pipe")
    except BrokenPipeError:
        pass
os.close(w)
for code in ("import select, sys; print('reading', flush=True); "
             "select.select([sys.stdin], [], [])",
             "import signal, sys, threading, time\n"
             "t = threading.Thread(target=sys.stdin.read); t.start()\n"
             "print('reading', flush=True); time.sleep(0.2)\n"
             "signal.pthread_kill(t.ident, signal.SIGTERM); time.sleep(30)"):
    p = subprocess.Popen([PY, "-c", code], stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE, text=True)
    assert p.stdout.readline() == "reading\n"
    if "pthread_kill" not in code:
        p.kill()
    t0 = time.monotonic()
    assert p.wait(timeout=10) in (-signal.SIGKILL, -signal.SIGTERM)
    assert time.monotonic() - t0 < 5, time.monotonic() - t0
    p.stdin.close()
print("10. kill a child blocked in read")

# 11. Two threads waiting for two children: each wakes for its own.
a = child("import time; time.sleep(0.5)")
b = child("import time; time.sleep(0.1)")
codes = {}
ta = threading.Thread(target=lambda: codes.__setitem__("a", a.wait()))
tb = threading.Thread(target=lambda: codes.__setitem__("b", b.wait()))
ta.start()
time.sleep(0.05)
tb.start()
ta.join(10)
tb.join(10)
assert codes == {"a": 0, "b": 0}, codes
print("11. two threads, two children")

# 12. Waiting on a group whose only child leaves it: ECHILD, not a hang.
#     A second thread waiting for another child takes no wake-up from it.
p = child("import os, time; time.sleep(0.3); os.setpgid(0, 0); time.sleep(0.5)")
other = child("import time; time.sleep(1)", process_group=0)
codes = {}
t = threading.Thread(target=lambda: codes.__setitem__("other", other.wait()))
t.start()
time.sleep(0.05)
try:
    os.waitpid(-os.getpgrp(), 0)
    raise AssertionError("waited on a group with no child in it")
except ChildProcessError:
    pass
assert p.wait() == 0
t.join(10)
assert codes == {"other": 0}, codes
print("12. a group's last child leaves it")

# 13. Two threads after one child, one for any child and one for its pid,
#     each starting first: one reaps it, the other is told there is nothing
#     left (and neither reads a freed process nor sleeps for good).
for first in ("any", "pid"):
    p = child("import time; time.sleep(0.3)")
    results = []
    def wait_any():
        try:
            results.append(os.wait()[0])
        except ChildProcessError:
            results.append("ECHILD")
    def wait_pid():
        try:
            results.append(os.waitpid(p.pid, 0)[0])
        except ChildProcessError:
            results.append("ECHILD")
    waits = {"any": wait_any, "pid": wait_pid}
    second = "pid" if first == "any" else "any"
    ta = threading.Thread(target=waits[first])
    tb = threading.Thread(target=waits[second])
    ta.start()
    time.sleep(0.05)
    tb.start()
    ta.join(10)
    tb.join(10)
    assert sorted(map(str, results)) == sorted([str(p.pid), "ECHILD"]), results
    p.returncode = 0  # reaped behind Popen's back
print("13. two waits for one child")

# 14. SIGTERM to a child whose reading thread blocks it: the process still
#     dies of it, the blocking thread included.
p = subprocess.Popen([PY, "-c",
                      "import signal, sys, threading, time\n"
                      "def read():\n"
                      "    signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGTERM})\n"
                      "    sys.stdin.read()\n"
                      "    sys.exit(1)\n"
                      "threading.Thread(target=read).start()\n"
                      "print('reading', flush=True); time.sleep(30)"],
                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
assert p.stdout.readline() == "reading\n"
time.sleep(0.2)
p.terminate()
assert p.wait(timeout=10) == -signal.SIGTERM, p.returncode
p.stdin.close()
print("14. a signal one thread blocks")

# 15. With SIGCHLD ignored, an exiting child goes at once (nothing to
#     reap), and a wait() for it learns so: ECHILD, not a hang.
old_chld = signal.signal(signal.SIGCHLD, signal.SIG_IGN)
pid = os.posix_spawn(PY, [PY, "-c", "import time; time.sleep(0.2)"], os.environ)
try:
    os.waitpid(pid, 0)
    raise AssertionError("reaped a child the kernel was to discard")
except ChildProcessError:
    pass
signal.signal(signal.SIGCHLD, old_chld)
print("15. SIGCHLD ignored")

# 16. A process paused for a signal wakes for the SIGCHLD of a child that
#     exits, as a shell's `cmd & wait` does: a signal the kernel sends, not
#     kill(), must wake it too.
#     The check and pause() are not atomic: a SIGCHLD that lands between
#     them leaves pause() nothing to wake for, so an alarm bounds it.  The
#     alarm's handler then runs first, as when pause() missed the SIGCHLD,
#     so that outcome is tried once more: missing it twice is the bug.
got = []
old_chld = signal.signal(signal.SIGCHLD, lambda *a: got.append("chld"))
old_alrm = signal.signal(signal.SIGALRM, lambda *a: got.append("alrm"))
for attempt in (1, 2):
    got.clear()
    p = child("import time; time.sleep(0.5)")
    signal.alarm(5)
    if not got:  # a child that beat us to it leaves nothing to wait for
        signal.pause()
    signal.alarm(0)
    assert p.wait() == 0
    if got[:1] == ["chld"]:
        break
else:
    raise AssertionError(got)
signal.signal(signal.SIGCHLD, old_chld)
signal.signal(signal.SIGALRM, old_alrm)
print("16. pause() wakes for a child's SIGCHLD")

# 17. pause() sleeps through a signal it does not take, here the SIGCHLD of
#     a child at its default action (ignored), and a later pause() sleeps
#     again rather than return for signals taken since.
#     As in 16, an alarm bounds a pause() that b's signal beat; a wrong
#     wake-up returns early, with b's signal yet to come.
got = []
old_usr1 = signal.signal(signal.SIGUSR1, lambda *a: got.append("usr1"))
old_alrm = signal.signal(signal.SIGALRM, lambda *a: None)
for n in (1, 2):
    a = child("import time; time.sleep(0.2)")
    b = child("import os, signal, time; time.sleep(0.8); os.kill(os.getppid(), signal.SIGUSR1)")
    signal.alarm(5)
    if len(got) < n:  # b's signal beat us to it: nothing left to wait for
        signal.pause()
    signal.alarm(0)
    assert got == ["usr1"] * n, (n, got)
    assert a.wait() == 0 and b.wait() == 0
signal.signal(signal.SIGUSR1, old_usr1)
signal.signal(signal.SIGALRM, old_alrm)
print("17. pause() sleeps through an ignored SIGCHLD")

# 18. Signals to a thread: signal 0 asks whether it exists and queues
#     nothing; one taken from the thread's queue leaves nothing pending, so
#     the same signal sent to the process next is taken too.
signal.pthread_kill(threading.get_ident(), 0)
got = []
old_usr1 = signal.signal(signal.SIGUSR1, lambda *a: got.append("usr1"))
for send in (lambda: signal.pthread_kill(threading.get_ident(), signal.SIGUSR1),
             lambda: os.kill(os.getpid(), signal.SIGUSR1)):
    signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGUSR1})
    send()
    signal.pthread_sigmask(signal.SIG_UNBLOCK, {signal.SIGUSR1})
assert got == ["usr1", "usr1"], got
# One sent to the thread while ignored is dropped, not kept to run the
# handler once the signal is handled.
signal.signal(signal.SIGUSR1, signal.SIG_IGN)
signal.pthread_kill(threading.get_ident(), signal.SIGUSR1)
signal.signal(signal.SIGUSR1, lambda *a: got.append("usr1"))
signal.pthread_kill(threading.get_ident(), signal.SIGUSR1)
assert got == ["usr1"] * 3, got
signal.signal(signal.SIGUSR1, old_usr1)
print("18. signals to a thread")

# 19. SIGTERM ends a process whose signal queue is full, here of realtime
#     signals it blocks (above 34: the kernel's realtime signals start at
#     musl's SIGRTMIN, 35, and glibc's 34 would be folded into one).  The
#     limit is on realtime signals; a standard one is never refused.
p = child("import os, signal, time\n"
          "rt = signal.SIGRTMIN + 2\n"
          "signal.pthread_sigmask(signal.SIG_BLOCK, {rt})\n"
          "for n in range(64):\n"
          "    try:\n"
          "        os.kill(os.getpid(), rt)\n"
          "    except OSError:\n"
          "        break\n"
          "print('full' if n < 63 else 'never full', flush=True); time.sleep(30)",
          stdout=subprocess.PIPE, text=True)
assert p.stdout.readline() == "full\n"
t0 = time.monotonic()
p.terminate()
assert p.wait(timeout=10) == -signal.SIGTERM
assert time.monotonic() - t0 < 5, time.monotonic() - t0
print("19. SIGTERM through a full signal queue")

# 20. A signal to a group reaches every member, though one refuses it (its
#     queue of realtime signals is full): the first refusal used to end the
#     send, and the members after it got nothing.
rt = signal.SIGRTMIN + 2
full = child("import os, signal, time\n"
             f"signal.pthread_sigmask(signal.SIG_BLOCK, {{{int(rt)}}})\n"
             "for n in range(64):\n"
             "    try:\n"
             f"        os.kill(os.getpid(), {int(rt)})\n"
             "    except OSError:\n"
             "        break\n"
             "print('full' if n < 63 else 'never full', flush=True); time.sleep(30)",
             stdout=subprocess.PIPE, text=True, process_group=0)
assert full.stdout.readline() == "full\n"
other = child("import time; time.sleep(30)", process_group=full.pid)
time.sleep(0.2)
os.killpg(full.pid, rt)
assert other.wait(timeout=10) == -rt, other.returncode
full.kill()
assert full.wait(timeout=10) == -signal.SIGKILL
print("20. a group signal past a member that refuses it")

print("Process demo passed")
