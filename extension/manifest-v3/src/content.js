// 创建挂载点存放wasm
const better_spider_id = 'better-spider-box';
const div = document.createElement('div')
div.setAttribute('id', better_spider_id);
document.body.appendChild(div);
// 创建style，链接better-spider样式
const link = document.createElement("link");
link.href = chrome.runtime.getURL("src/better-spider.css");
link.type = "text/css";
link.rel = "stylesheet";
document.getElementsByTagName("head")[0].appendChild(link);
document.oncontextmenu = () => false; //右键菜单屏蔽
import { run } from './wasm/better_spider';
run();