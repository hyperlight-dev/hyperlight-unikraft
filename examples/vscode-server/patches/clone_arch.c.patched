/* SPDX-License-Identifier: BSD-3-Clause */
/* Copyright (c) 2023, Unikraft GmbH and The Unikraft Authors.
 * Licensed under the BSD-3-Clause License (the "License").
 * You may not use this file except in compliance with the License.
 */

#include <string.h>
#include <uk/process.h>
#include <uk/arch/ctx.h>
#include <uk/lcpu.h>
#include <uk/print.h>

static void child_execenv_load_traced(long auxsp_pos)
{
	struct ukarch_execenv *cexecenv = (struct ukarch_execenv *)auxsp_pos;

	uk_pr_err("VFORK_DIAG: child scheduled! execenv=%p "
		  "RIP=0x%lx RSP=0x%lx RAX=0x%lx R9=0x%lx\n",
		  cexecenv,
		  uk_lcpu_regs_get(cexecenv->regs, RIP),
		  uk_lcpu_regs_get(cexecenv->regs, RSP),
		  uk_lcpu_regs_get(cexecenv->regs, RAX),
		  uk_lcpu_regs_get(cexecenv->regs, R9));

	ukarch_execenv_load(auxsp_pos);
}

void clone_setup_child_ctx(struct ukarch_execenv *pexecenv,
			   struct uk_thread *child, __uptr sp,
			   int is_vfork)
{
	struct ukarch_execenv *cexecenv;
	struct ukarch_auxspcb *auxspcb;
	__u64 gsbase, fsbase;
	__uptr auxsp_pos;

	UK_ASSERT(pexecenv);
	UK_ASSERT(child);
	UK_ASSERT(sp);

	auxspcb = ukarch_auxsp_get_cb(child->auxsp);
	UK_ASSERT(auxspcb);

	auxsp_pos = ukarch_auxspcb_get_curr_fp(auxspcb);
	UK_ASSERT(auxsp_pos);

	/* Create a child context whose stack pointer is that of the auxiliary
	 * stack, minus the parent's `struct ukarch_execenv` saved on the
	 * auxiliary stack that we will have to first patch now and then pop off
	 */

	/* Make room for child's copy of `struct ukarch_execenv` */
	auxsp_pos = ALIGN_DOWN(auxsp_pos, UKARCH_EXECENV_END_ALIGN);
	auxsp_pos -= UKARCH_EXECENV_SIZE;

	/* Now patch the child's return registers */
	cexecenv = (struct ukarch_execenv *)auxsp_pos;
	*cexecenv = *pexecenv;

	/* Child must see %rax as 0 */
	uk_lcpu_regs_set(cexecenv->regs, RAX, 0x0);

	/* Use new stack pointer */
	uk_lcpu_regs_set(cexecenv->regs, RSP, sp);

	/* Use parent's userland gsbase */
	gsbase = uk_lcpu_sysctx_get(pexecenv->sysctx, GSBASE);
	uk_lcpu_sysctx_set(cexecenv->sysctx, GSBASE, gsbase);

	/* For vfork the child shares the parent's stack whose frames
	 * carry stack-canary values derived from the parent's user-TLS
	 * (%fs:0x28 on x86-64).  The child has its own kernel TLS
	 * (child->uktlsp) for __thread variables, but the execenv
	 * FSBASE — which is restored on return to userspace — must be
	 * the parent's so the canary check passes.
	 */
	if (is_vfork || !child->tlsp)
		fsbase = uk_lcpu_sysctx_get(pexecenv->sysctx, FSBASE);
	else
		fsbase = child->tlsp;

	uk_lcpu_sysctx_set(cexecenv->sysctx, FSBASE, fsbase);

	ukarch_ctx_init_entry1(&child->ctx,
			       auxsp_pos,
			       1,
			       (ukarch_ctx_entry1)&child_execenv_load_traced,
			       auxsp_pos);
}
