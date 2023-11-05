// node --input-type=module --eval "$(cat ./app.js)"

import * as M from './mod.mjs'

console.log(M);
// =>
// [Module: null prototype] {
//     default: [Function: Array],
//     randomItem: [Function: randomItem]
// }

for (let i = 0; i < 10; i++) {
    console.log([42, 'sample'].sample());
}
