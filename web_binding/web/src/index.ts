import init, { wasm_buffer_batch_tokenize, wasm_batch_tokenize, wasm_parallel_tokenize, wasm_tokenize } from "wasm-hello-world/wasm_hello_world";

await init();

// enum TokenType {
//     Operator=1,
//     Number=2,
//     String=3,
//     Symbol=4,
//     Space=5,
//     Debugger=6,
//     ArgSeparator=7,
//     LeftParen=8,
//     RightParen=9,
//     Reference=10,
//     InvalidReference=11,
//     Unknown=12,
// }

wasm_parallel_tokenize
wasm_buffer_batch_tokenize
wasm_tokenize
// console.log(wasm_parallel_tokenize(["=1+1", '="coucou" + 10']));

let formulas: string[] = [];
for (let i = 0; i < 100000; i++) {
    formulas.push('=10+"abc"');
}

console.time("wasm_batch_tokenize")
const tokens = wasm_batch_tokenize(formulas);
console.timeEnd("wasm_batch_tokenize")
console.log(tokens);


console.time("wasm_buffer_batch_tokenize")
const buffer = wasm_buffer_batch_tokenize(formulas)
const all_tokens: object[] = [];

let formula_index = 0;
for (let i = 0; i < buffer.length; i++) {
    let offset_in_formula = 0;
    const tokens: object[] = [];
    while (buffer[i] !== 0) {
        const formula = formulas[formula_index];
        const token_length = buffer[i+1];
        tokens.push({ type: buffer[i], value: formula.substring(offset_in_formula, offset_in_formula + token_length) });
        offset_in_formula += token_length;
        i+=2;
    }
    formula_index++;
    all_tokens.push(tokens);
}
console.timeEnd("wasm_buffer_batch_tokenize")
console.log(all_tokens);
