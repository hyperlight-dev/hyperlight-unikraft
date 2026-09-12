/*
 * hl_driver.h — shared boilerplate for every Hyperlight runtime driver.
 *
 * A driver's job is the same regardless of runtime: read the kernel dispatch
 * addresses the platform injected as env vars, register a dispatch callback,
 * and halt so the host can drive it.  Only the callback and any runtime setup
 * in between differ, so a driver's main() reduces to:
 *
 *     int main(int argc, char **argv, char **envp) {
 *         if (hl_driver_init(envp, "hl_foodriver"))
 *             return 1;
 *         ... runtime-specific init ...
 *         hl_driver_run(foo_dispatch);   // registers + halts; never returns
 *     }
 *
 * Include hl_fc.h (for hl_dispatch_fn_t) transitively via this header.
 */

#ifndef HL_DRIVER_H
#define HL_DRIVER_H

#include <stdint.h>
#include <stdio.h>

#include "hl_fc.h"  /* hl_dispatch_fn_t */
#include "hl_env.h" /* hl_env_init, hl_env_clean_reserved */

/*
 * Kernel dispatch wiring, injected as env vars by plat/hyperlight/dispatch.c:
 *   HL_DISPATCH_CALLBACK_PTR — where to store our callback pointer
 *   HL_DISPATCH_ENTRY        — address to halt into for each host call()
 *
 * TODO: these are raw kernel addresses passed as env vars.  Replace with a
 * cleaner interface (vDSO export, syscall, or device ioctl).
 */
static hl_dispatch_fn_t *g_callback_slot;
static uint64_t g_dispatch_entry;

/*
 * Parse the kernel dispatch addresses from the environment and validate them.
 * Returns 0 on success (g_callback_slot / g_dispatch_entry set), 1 on error.
 */
static inline int hl_driver_init(char **envp, const char *name)
{
	hl_env_init(envp, &g_callback_slot, &g_dispatch_entry);
	hl_env_clean_reserved();

	if (!g_callback_slot || !g_dispatch_entry) {
		fprintf(stderr,
			"%s: missing HL_DISPATCH_CALLBACK_PTR or HL_DISPATCH_ENTRY\n",
			name);
		return 1;
	}
	return 0;
}

/*
 * Register the dispatch callback and halt the VM — never returns.
 *
 * Why halt instead of exit?  exit_group would tear down the VFS fd table
 * (closing stdout/stderr) and run atexit handlers; halting directly leaves
 * fds 0/1/2 open and the runtime's heap/TLS intact for the dispatch calls.
 * The elfloader thread is frozen mid-wait — it never runs again, which is
 * fine; snapshot/restore captures and restores that frozen state.
 *
 * RAX = the dispatch entry so the host knows where to set RIP for each
 * call().  Port 108 = the halt VmAction.  See plat/hyperlight/dispatch.c.
 */
static inline __attribute__((noreturn)) void hl_driver_run(hl_dispatch_fn_t cb)
{
	*g_callback_slot = cb;
	__asm__ volatile(
		"andq $~0xf, %%rsp\n\t"
		"movq %0, %%rax\n\t"
		"movw $108, %%dx\n\t"
		"outl %%eax, %%dx\n\t"
		"cli\n\t"
		"hlt\n\t"
		: : "r"(g_dispatch_entry) : "rax", "rdx", "memory"
	);
	__builtin_unreachable();
}

#endif /* HL_DRIVER_H */
