#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

function getBinaryPath() {
    const platform = process.platform;
    const arch = process.arch;

    let binaryName;

    if (platform === 'win32') {
        binaryName = 'notion-formatter.exe';
    } else {
        binaryName = 'notion-formatter';
    }

    // In a real package, you'd have logic to select the correct binary
    // based on platform and architecture. For local testing, we'll
    // just point to the target/release directory.
    const binaryPath = path.join(__dirname, 'target', 'release', binaryName);
    
    // For a real package, you might use a structure like this:
    // const binaryPath = path.join(__dirname, 'bin', `${platform}-${arch}`, binaryName);
    
    return binaryPath;
}

const binary = getBinaryPath();
const args = process.argv.slice(2);

const child = spawn(binary, args, { stdio: 'inherit' });

child.on('close', (code) => {
    if (code !== 0) {
        console.error(`notion-formatter exited with code ${code}`);
        process.exit(1);
    }
});

child.on('error', (err) => {
    console.error('Failed to start notion-formatter:', err);
    process.exit(1);
});
