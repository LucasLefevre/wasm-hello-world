import init, { WasmUniverse as Universe } from "wasm-hello-world/wasm_hello_world";

await init();

const universe = Universe.new(800, 600);

console.log(universe.get_area())
console.log(universe.get_area_object())
