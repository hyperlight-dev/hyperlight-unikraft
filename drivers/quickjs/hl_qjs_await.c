/*
 * hl_qjs_await.c -- quickjs-libc, and what the driver's await needs of it.
 *
 * Compiled in place of quickjs-ng's quickjs-libc.c, which it includes
 * unmodified, with warnings off as the engine is; the await itself is in
 * hl_quickjsdriver.c.  js_os_poll() and the thread state are static in
 * quickjs-libc.c, and the await needs both: to wait for timers and I/O,
 * and to tell when nothing is left that could ever settle a promise.
 */

#include "quickjs-libc.c"

/* Wait for and run one timer or I/O event: js_os_poll().  Only called
 * once hl_os_idle() has said there is something to wait for. */
int hl_os_poll(JSContext *ctx)
{
	return js_os_poll(ctx);
}

/* Whether nothing is left to wait for: js_os_poll's own "no more
 * events" test (no timer, I/O handler or worker port; a signal handler
 * alone does not keep it waiting). */
int hl_os_idle(JSContext *ctx)
{
	JSThreadState *ts = js_get_thread_state(JS_GetRuntime(ctx));

	return !ts->can_js_os_poll ||
	       (list_empty(&ts->os_timers) && list_empty(&ts->os_rw_handlers) &&
		list_empty(&ts->port_list));
}

/* Cancel every timer, I/O handler and worker port: what a call that ended
 * on an uncaught error had started. */
void hl_os_cancel(JSContext *ctx)
{
	JSRuntime *rt = JS_GetRuntime(ctx);
	JSThreadState *ts = js_get_thread_state(rt);
	struct list_head *el, *el1;

	list_for_each_safe(el, el1, &ts->os_timers)
		free_timer(rt, list_entry(el, JSOSTimer, link));
	list_for_each_safe(el, el1, &ts->os_rw_handlers)
		free_rw_handler(rt, list_entry(el, JSOSRWHandler, link));
#ifdef USE_WORKER
	list_for_each_safe(el, el1, &ts->port_list)
		js_free_port(rt, list_entry(el, JSWorkerMessageHandler, link));
#endif
}
