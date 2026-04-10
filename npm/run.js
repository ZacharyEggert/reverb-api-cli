#!/usr/bin/env node
'use strict';

const { join } = require('path');
const { spawnSync } = require('child_process');
const { getPlatformKey } = require('./platform');

const pkg = require('./package.json');
const platformKey = getPlatformKey();
const binaryName = pkg.supportedPlatforms[platformKey]?.binary ?? 'revcli';
const binaryPath = join(__dirname, 'bin', binaryName);

const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

if (result.error) {
  if (result.error.code === 'ENOENT') {
    console.error(`revcli binary not found at ${binaryPath}`);
    console.error('Try reinstalling: npm install -g @reverbdotcom/cli');
  } else {
    console.error(result.error.message);
  }
  process.exit(1);
}

process.exit(result.status ?? 0);
