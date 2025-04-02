import nodeResolve from "@rollup/plugin-node-resolve";
import wasm from "@rollup/plugin-wasm";

const config = [
{
    input: 'build/js/index.js',
    output: {
        file: 'build/bundle/bundle.js',
        format: 'es',
        sourcemap: true,
    },
    plugins: [ wasm(), nodeResolve()],
}
];
export default config;
