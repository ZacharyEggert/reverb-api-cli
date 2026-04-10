'use strict';

const {arch, platform} = process;

function getPlatformKey() {
    const a = arch === 'arm64' ? 'aarch64' : 'x86_64';

    if (platform === 'darwin') return `${a}-apple-darwin`;
    if (platform === 'win32') return `${a}-pc-windows-msvc`;
    if (platform === 'linux') return `${a}-unknown-linux-gnu`;

    throw new Error(`Unsupported platform: ${platform} ${arch}`);
}

module.exports = {getPlatformKey};
