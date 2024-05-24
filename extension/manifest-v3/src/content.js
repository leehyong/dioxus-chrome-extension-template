const better_spider_id = 'better-spider-box';
const div = document.createElement('div')
div.setAttribute('id', better_spider_id);
document.body.appendChild(div);

const link = document.createElement("link");
link.href = chrome.runtime.getURL("src/better-spider.css");
link.type = "text/css";
link.rel = "stylesheet";
document.getElementsByTagName("head")[0].appendChild(link);

import { run } from './wasm/better_spider';
run();