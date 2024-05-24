
// const WASM_MOD_URL = chrome.runtime.getURL('dist/better_spider_bg.wasm');


// // // Import Wasm module binding using dynamic import
// // // "init" may fail if the current site CSP restricts the use of Wasm (e.g. any github.com page)
// // // In this case instantiate module in the background worker (see background.js) and use message passing
// const loadWasmModule = async () => {
//     const { default: init } = await import(WASM_MOD_URL);

//     return init().catch(() => null);
// };


// (async () => {
//     const mod = await loadWasmModule();

//     // If the module is successfully initialized,
//     // import entities from the module
//     if (mod) {
//         const { hello_content, hello_background } = mod;

//         hello_content();
//         hello_background();
//     }
// })();
// fixme: 怎么自动替换这里 ？
// const uri= chrome.runtime.getURL("./js/dist/" + wasmModuleHash + ".module.wasm")

const better_spider_id = 'better-spider-box';
const div = div = document.createElement('div')
div.setAttribute('id', better_spider_id);
document.body.appendChild(div)

import * as css from "./better_spider.css";
const style = document.createElement('style');
console.log('spider css',css)
style.innerHTML = css;
document.body.appendChild(style);

import { run } from './wasm/better_spider';
run();