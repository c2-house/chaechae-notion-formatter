#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

function getBinaryPath() {
    const platform = process.platform; 
    const arch = process.arch;

    let target;
    if (platform === 'darwin' && arch === 'arm64') {
        target = 'aarch64-apple-darwin';
    } else if (platform === 'darwin' && arch === 'x64') {
        target = 'x86_64-apple-darwin';
    } else if (platform === 'linux' && arch === 'x64') {
        target = 'x86_64-unknown-linux-gnu';
    } else if (platform === 'win32' && arch === 'x64') {
        target = 'x86_64-pc-windows-msvc';
    } else {
        console.error(`Unsupported platform: ${platform}-${arch}`);
        process.exit(1);
    }
    
    const binaryName = platform === 'win32' ? 'notion-formatter.exe' : 'notion-formatter';
    const binaryPath = path.join(__dirname, 'bin', target, binaryName);

    if (!fs.existsSync(binaryPath)) {
        console.error(`Could not find binary for your platform at: ${binaryPath}`);
        console.error('Please report this issue on GitHub.');
        process.exit(1);
    }

    return binaryPath;
}

const binary = getBinaryPath();
const args = process.argv.slice(2);

const child = spawn(binary, args, { stdio: 'inherit' });

child.on('close', (code) => {
    if (code !== 0) {
        process.exit(code);
    }
});

child.on('error', (err) => {
    console.error('Failed to start the notion-formatter binary:', err);
    process.exit(1);
});