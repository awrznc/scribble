const pug = require('pug');

const compiledFunction = pug.compileFile('assets/template.pug');
const html = compiledFunction( { value: 'Hello' } );

console.log( html );
