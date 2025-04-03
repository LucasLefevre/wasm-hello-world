import init, { wasm_batch_tokenize, wasm_parallel_tokenize, wasm_tokenize } from "wasm-hello-world/wasm_hello_world";

await init();

wasm_parallel_tokenize
wasm_batch_tokenize
wasm_tokenize
// console.log(wasm_parallel_tokenize(["=1+1", '="coucou" + 10']));

let formulas: string[] = [];
for (let i = 0; i < 100000; i++) {
    formulas.push("=1+1");
}
console.time("wasm_tokenize")
const tokens = wasm_batch_tokenize(formulas)
console.timeEnd("wasm_tokenize")
console.log(tokens)
// console.log(tokens)
