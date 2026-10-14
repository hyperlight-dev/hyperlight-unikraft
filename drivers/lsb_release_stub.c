/* No-op lsb_release stub for Unikraft guests.
 *
 * Note: pip's distro detection execs lsb_release to identify the OS.
 * Without it the exec fails, which pip copes with, but every failed
 * exec prints kernel log lines on standard error; a valid ELF binary
 * that silently exits 0 keeps them away.
 */
int main(void) { return 0; }
