// Simple utility module — exercises require, module.exports, basic JS
"use strict";

function fibonacci(n) {
    if (n <= 1) return n;
    let a = 0, b = 1;
    for (let i = 2; i <= n; i++) {
        [a, b] = [b, a + b];
    }
    return b;
}

function parseCSV(text) {
    return text.split("\n").filter(Boolean).map(line =>
        line.split(",").map(cell => cell.trim())
    );
}

function sortRecords(records, colIndex) {
    return [...records].sort((a, b) => {
        const va = a[colIndex], vb = b[colIndex];
        const na = Number(va), nb = Number(vb);
        if (!isNaN(na) && !isNaN(nb)) return na - nb;
        return String(va).localeCompare(String(vb));
    });
}

function aggregate(records, colIndex) {
    const groups = {};
    for (const row of records) {
        const key = row[colIndex] || "(empty)";
        if (!groups[key]) groups[key] = [];
        groups[key].push(row);
    }
    return groups;
}

module.exports = { fibonacci, parseCSV, sortRecords, aggregate };
