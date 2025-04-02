import init, { parallel_tokenize } from "wasm-hello-world/wasm_hello_world";

await init();

const tokens = parallel_tokenize(["=1+1", '="coucou" + 10']);

console.log(tokens)
