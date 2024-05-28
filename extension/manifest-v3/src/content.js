// 创建挂载点存放wasm
function create_spider_box() {
    const better_spider_id = 'better-spider-box';
    const div = document.createElement('div')
    div.setAttribute('id', better_spider_id);
    document.body.appendChild(div);
}
function create_css_link(path) {
    const link = document.createElement("link");
    link.href = chrome.runtime.getURL(path);
    link.type = "text/css";
    link.rel = "stylesheet";
    link.media = "all"
    document.getElementsByTagName("head")[0].appendChild(link);
}
create_spider_box();
create_css_link("src/hover-min.css");
create_css_link("src/animate.min.css");
create_css_link("src/better-spider.css");
// 创建style，链接better-spider样式

document.oncontextmenu = () => false; //右键菜单屏蔽
import { run } from './wasm/better_spider';
run();