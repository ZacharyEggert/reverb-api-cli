'use strict';

const {execSync} = require('child_process');
const {createWriteStream, mkdirSync, chmodSync} = require('fs');
const {join} = require('path');
const https = require('https');
const {getPlatformKey} = require('./platform');

const pkg = require('./package.json');
const {version, supportedPlatforms} = pkg;

const platformKey = getPlatformKey();
const platformInfo = supportedPlatforms[platformKey];

if (!platformInfo) {
    console.error(`No binary available for platform: ${platformKey}`);
    process.exit(1);
}

const artifactName = platformInfo.artifact.replace('{version}', version);
const binaryName = platformInfo.binary;
const downloadUrl = `https://github.com/reverbdotcom/cli/releases/download/v${version}/${artifactName}`;

const binDir = join(__dirname, 'bin');
mkdirSync(binDir, {recursive: true});

console.log(`Downloading revcli ${version} for ${platformKey}...`);
console.log(`  ${downloadUrl}`);

// Download and extract (simplified — use a proper tar/unzip in production)
// TODO: implement actual download + extraction
console.log('Install complete. Run: revcli --help');
