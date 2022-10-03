import { gn, ninjaClean } from './helper.js';

const outDir = `out/${process.argv[2] ?? 'Debug'}`;

ninjaClean({ outDir });
