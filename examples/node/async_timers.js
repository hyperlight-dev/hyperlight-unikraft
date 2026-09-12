/** Async operations — demonstrates that setTimeout and Promises work. */
const results = [];

setTimeout(() => {
    results.push('timer-1');
    setTimeout(() => {
        results.push('timer-2');
        console.log(`results: ${results.join(', ')}`);
        console.log('async-done');
    }, 50);
}, 50);
