// Filesystem operations — demonstrates hostfs mount.

const fs = require('fs');
const path = require('path');

const mountDir = '/mnt/host';
const filePath = path.join(mountDir, 'hello.txt');

// Write and read back.
fs.writeFileSync(filePath, 'Hello from Node.js guest!\n');
const content = fs.readFileSync(filePath, 'utf-8');
console.log(`Read back: ${content.trim()}`);

// Stat.
const stat = fs.statSync(filePath);
console.log(`Size: ${stat.size} bytes`);

// List directory.
const entries = fs.readdirSync(mountDir);
console.log(`Files in ${mountDir}: ${entries.join(', ')}`);

// Create subdirectory.
const subdir = path.join(mountDir, 'subdir');
fs.mkdirSync(subdir, { recursive: true });
fs.writeFileSync(path.join(subdir, 'nested.txt'), 'nested content\n');

// Walk and print.
function walk(dir) {
    for (const entry of fs.readdirSync(dir)) {
        const full = path.join(dir, entry);
        const s = fs.statSync(full);
        if (s.isDirectory()) {
            walk(full);
        } else {
            console.log(`  ${full} (${s.size} bytes)`);
        }
    }
}
walk(mountDir);

// Clean up.
fs.unlinkSync(path.join(subdir, 'nested.txt'));
fs.rmdirSync(subdir);
fs.unlinkSync(filePath);
console.log('Cleanup done.');
