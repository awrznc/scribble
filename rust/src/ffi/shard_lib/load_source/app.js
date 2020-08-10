// WebAssembly.instantiateStreaming(fetch('./target/wasm32-unknown-unknown/foo.wasm'), importObject)
// .then(results => {
//   // Do something with the results!
// });

const wasmCode = await Deno.readFile("./target/wasm32-unknown-unknown/release/foo.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const {
    hello,
} = wasmInstance.exports;

console.log(hello());
