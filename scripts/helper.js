import { execSync } from 'child_process';
import { join } from 'path';

import { stringifyArgs } from './gn-args.js';

const skiaDir = `${process.cwd()}/third-party/skia`;
const ninja = `${process.cwd()}/third-party/depot_tools/ninja`;

function exec(command, cwd) {
  execSync(command, { stdio: 'inherit', cwd, env: process.env });
}

export function gn({ outDir }) {
  exec(`bin/gn gen ${outDir} --args="${stringifyArgs()}"`, skiaDir);
}

export function ninjaBuild({ outDir, args }) {
  exec(`${ninja} -C ${outDir} ${args ?? ''}`, skiaDir);
}

export function ninjaClean({ outDir }) {
  exec(`${ninja} -t clean`, join(skiaDir, outDir));
}
