// Minimal test runner — no external deps, exercises Node.js core
"use strict";

const { fibonacci, parseCSV, sortRecords, aggregate } = require("./index");

let passed = 0;
let failed = 0;

function assert(condition, message) {
    if (condition) {
        passed++;
    } else {
        failed++;
        console.error(`  FAIL: ${message}`);
    }
}

function assertEqual(actual, expected, message) {
    assert(
        JSON.stringify(actual) === JSON.stringify(expected),
        `${message}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`
    );
}

// --- fibonacci ---
console.log("Testing fibonacci...");
assertEqual(fibonacci(0), 0, "fib(0)");
assertEqual(fibonacci(1), 1, "fib(1)");
assertEqual(fibonacci(10), 55, "fib(10)");
assertEqual(fibonacci(20), 6765, "fib(20)");
assertEqual(fibonacci(30), 832040, "fib(30)");

// --- parseCSV ---
console.log("Testing parseCSV...");
assertEqual(
    parseCSV("a,b,c\n1,2,3\n4,5,6"),
    [["a","b","c"],["1","2","3"],["4","5","6"]],
    "basic CSV"
);
assertEqual(parseCSV(""), [], "empty CSV");
assertEqual(
    parseCSV("x, y , z"),
    [["x","y","z"]],
    "CSV with whitespace"
);

// --- sortRecords ---
console.log("Testing sortRecords...");
const records = [["b","2"],["a","3"],["c","1"]];
assertEqual(
    sortRecords(records, 0),
    [["a","3"],["b","2"],["c","1"]],
    "sort by col 0 (alpha)"
);
assertEqual(
    sortRecords(records, 1),
    [["c","1"],["b","2"],["a","3"]],
    "sort by col 1 (numeric)"
);

// --- aggregate ---
console.log("Testing aggregate...");
const data = [["us","10"],["uk","20"],["us","30"],["uk","40"]];
const groups = aggregate(data, 0);
assertEqual(Object.keys(groups).sort(), ["uk","us"], "group keys");
assertEqual(groups["us"].length, 2, "us count");
assertEqual(groups["uk"].length, 2, "uk count");

// --- stress: compute-bound loop ---
console.log("Testing compute stress...");
const t0 = Date.now();
let sum = 0;
for (let i = 0; i < 1000; i++) {
    sum += fibonacci(40);
}
const elapsed = Date.now() - t0;
assert(sum === 102334155000, "stress sum");
console.log(`  compute: ${elapsed}ms for 1000x fib(40)`);

// --- Summary ---
console.log(`\n${passed} passed, ${failed} failed`);
if (failed > 0) {
    process.exit(1);
} else {
    console.log("NODE_BENCH_OK");
}
