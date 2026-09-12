# Coreutils demo — real cat, grep, sed, sort, awk, ls, etc.
#
# With NOMMU mode, BusyBox hush uses vfork()+execve() to run all
# applets as separate processes.  Pipes and output redirects from
# external commands crash the kernel (vfork shares the parent's
# stack with the child), so each command reads from a file and
# writes to stdout.

mkdir -p /tmp/demo

# ── Create test data (printf is a shell builtin, redirects work) ─
printf "alice:admin:login\n"   > /tmp/demo/users.txt
printf "bob:viewer:login\n"   >> /tmp/demo/users.txt
printf "carol:admin:logout\n" >> /tmp/demo/users.txt
printf "dave:viewer:logout\n" >> /tmp/demo/users.txt
printf "eve:editor:login\n"   >> /tmp/demo/users.txt

# ── cat ──────────────────────────────────────────────────────────
echo "=== cat ==="
cat /tmp/demo/users.txt
echo ""

# ── grep ─────────────────────────────────────────────────────────
echo "=== grep admin ==="
grep admin /tmp/demo/users.txt
echo ""

echo "=== grep -v admin ==="
grep -v admin /tmp/demo/users.txt
echo ""

echo "=== grep -c login ==="
grep -c login /tmp/demo/users.txt
echo ""

# ── head / tail ──────────────────────────────────────────────────
echo "=== head -2 ==="
head -2 /tmp/demo/users.txt
echo ""

echo "=== tail -2 ==="
tail -2 /tmp/demo/users.txt
echo ""

# ── wc ───────────────────────────────────────────────────────────
echo "=== wc -l ==="
wc -l /tmp/demo/users.txt
echo ""

# ── cut ──────────────────────────────────────────────────────────
echo "=== cut -d: -f2 ==="
cut -d: -f2 /tmp/demo/users.txt
echo ""

# ── sort ─────────────────────────────────────────────────────────
echo "=== sort ==="
sort /tmp/demo/users.txt
echo ""

echo "=== sort -r ==="
sort -r /tmp/demo/users.txt
echo ""

# ── uniq -c (awk does it in one pass — no pipes needed) ──────────
echo "=== uniq -c by role ==="
awk -F: '{ c[$2]++ } END { for (r in c) printf "  %d %s\n", c[r], r }' /tmp/demo/users.txt
echo ""

# ── sed ──────────────────────────────────────────────────────────
echo "=== sed s/viewer/readonly/ ==="
sed 's/viewer/readonly/' /tmp/demo/users.txt
echo ""

# ── awk ──────────────────────────────────────────────────────────
echo "=== awk table ==="
awk -F: '{ printf "%-8s %-10s %s\n", $1, $2, $3 }' /tmp/demo/users.txt
echo ""

echo "=== awk filter + transform ==="
awk -F: '$3 == "login" { print toupper($1), $2 }' /tmp/demo/users.txt
echo ""

# ── ls ───────────────────────────────────────────────────────────
echo "=== ls ==="
touch /tmp/demo/a.log /tmp/demo/b.log /tmp/demo/notes.md
ls --color=never /tmp/demo/
echo ""

echo "=== ls -la ==="
ls -la --color=never /tmp/demo/users.txt
echo ""

# ── find ─────────────────────────────────────────────────────────
echo "=== find *.txt ==="
find /tmp/demo -name "*.txt"
echo ""

echo "=== find *.log ==="
find /tmp/demo -name "*.log"
echo ""

# ── stat ─────────────────────────────────────────────────────────
echo "=== stat ==="
stat /tmp/demo/users.txt
echo ""

# ── base64 ───────────────────────────────────────────────────────
echo "=== base64 encode ==="
echo "Hello, Hyperlight!" > /tmp/demo/msg.txt
base64 /tmp/demo/msg.txt
echo ""

# ── md5sum / sha256sum ───────────────────────────────────────────
echo "=== checksums ==="
md5sum /tmp/demo/users.txt
sha256sum /tmp/demo/users.txt
echo ""

# ── string ops (shell builtins) ──────────────────────────────────
echo "=== string ops ==="
path="/usr/local/bin/hello.tar.gz"
echo "full:      $path"
echo "basename:  ${path##*/}"
echo "dirname:   ${path%/*}"
echo "extension: ${path##*.}"
echo "strip ext: ${path%.*}"
echo ""

# ── arithmetic (shell builtins) ──────────────────────────────────
echo "=== arithmetic ==="
echo "42 * 2 + 1 = $((42 * 2 + 1))"
echo "2^10 = $((1 << 10))"
echo "255 & 0xF0 = $((255 & 0xF0))"
echo "17 %% 5 = $((17 % 5))"
echo ""

# ── system info ──────────────────────────────────────────────────
echo "=== system info ==="
uname -a
whoami
pwd
which sh
echo ""

# ── seq ──────────────────────────────────────────────────────────
echo "=== seq ==="
seq 1 5
echo ""

# ── hexdump ──────────────────────────────────────────────────────
echo "=== hexdump ==="
hexdump -C /tmp/demo/msg.txt
echo ""

# ── cleanup ──────────────────────────────────────────────────────
rm -f /tmp/demo/users.txt /tmp/demo/a.log /tmp/demo/b.log /tmp/demo/notes.md /tmp/demo/msg.txt
rmdir /tmp/demo

echo "Done"
