import init, { say_something } from './pkg/killingdihspree.js';

async function run() {
    // Initialize WASM
    await init();

    // Call the function after initialization
    say_something();
}

run();
