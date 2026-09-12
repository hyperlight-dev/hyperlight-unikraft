/* No-op lsb_release stub for Unikraft guests.
 *
 * Note: pip's distro detection execs lsb_release to identify the OS.
 * Unikraft's execve crashes on exec failure (no graceful recovery),
 * so we provide a valid ELF binary that silently exits 0.
 */
int main(void) { return 0; }
