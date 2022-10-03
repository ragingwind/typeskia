import { gn, ninjaBuild } from './helper.js';

const outDir = `out/${process.argv[2] ?? 'Debug'}`;

gn({outDir, args: 'skia module'});

ninjaBuild({outDir});
