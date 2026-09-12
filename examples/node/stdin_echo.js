/** Read from stdin and echo each line. */
const readline = require('readline');
const rl = readline.createInterface({ input: process.stdin });
const lines = [];
rl.on('line', (line) => { lines.push(line); });
rl.on('close', () => {
    console.log(`lines=${lines.length}`);
    for (const line of lines) { console.log(`echo: ${line}`); }
    console.log('stdin-done');
});
