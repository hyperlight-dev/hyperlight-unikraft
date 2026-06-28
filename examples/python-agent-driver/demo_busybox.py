import subprocess

cmds = [
    (["echo", "hello from hyperlight guest"], None),
    (["uname", "-a"], None),
    (["ls", "/bin"], None),
    (["grep", "nameserver", "/etc/resolv.conf"], None),
    (["find", "/etc", "-name", "*.conf"], None),
    (["wc", "-l", "/etc/resolv.conf"], None),
    (["sh", "-c", "echo hello from sh"], None),
]

for cmd, stdin in cmds:
    label = " ".join(cmd)
    print(f"\n$ {label}")
    r = subprocess.run(cmd, capture_output=True, text=True, input=stdin)
    if r.stdout:
        print(r.stdout.rstrip())
    if r.stderr:
        print(f"stderr: {r.stderr.rstrip()}")
    if r.returncode != 0:
        print(f"exit code: {r.returncode}")
