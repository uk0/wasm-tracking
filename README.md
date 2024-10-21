##  wasm-tracking

* 埋点数据


### quick 


```js
// step 1
import init, {WasmObserver} from '/static/js/wasm_monitor.js';

async function initWasm() {
    await init();
    const observer = new WasmObserver();
    const target = document.body;
    try {
        observer.observe(target);
        console.log("WASM Observer started");
    } catch (error) {
        console.error("Error starting WASM Observer:", error);
    }
}
// step 2
document.addEventListener('DOMContentLoaded', async function () {
    await initWasm();
});

```


### data example

```json

{
  "body": "<body>\n\n<div id=\"container\">container</div>\n<div id=\"base\">base</div>\n<div id=\"base_64\">base_64</div>\n<div id=\"app\" class=\"nested-div\">\n    <h1>DOM\u5d4c\u5957\u6d4b\u8bd5 with WASM</h1>\n    <div id=\"v1\" class=\"nested-div\">\n        <h2>\u5d4c\u5957\u5c42\u7ea71</h2>\n        <div id=\"v2\" class=\"nested-div\">\n            <h3>\u5d4c\u5957\u5c42\u7ea72</h3>\n            <div id=\"monitor-results\"><div class=\"event\">\n            <div class=\"event-type\">Click</div>\n            <div class=\"event-details\">Clicked element: BUTTON#addButton</div>\n            <div class=\"event-time\">Time: 1:45:53 PM</div>\n        </div><div class=\"event\">\n            <div class=\"event-type\">DOM Change</div>\n            <div class=\"event-details\">Added new nested element</div>\n            <div class=\"event-time\">Time: 1:45:53 PM</div>\n        </div></div>\n            <div id=\"v3\" class=\"nested-div\">\n                <h4>\u5d4c\u5957\u5c42\u7ea73</h4>\n                <div id=\"v4\" class=\"nested-div\">\n                    <h5>\u5d4c\u5957\u5c42\u7ea74</h5>\n                    <div id=\"v5\" class=\"nested-div\">\n                        <h5>\u5d4c\u5957\u5c42\u7ea75</h5>\n                        <button id=\"addButton\">\u6dfb\u52a0\u65b0\u7684\u5d4c\u5957\u5143\u7d20</button>\n                        <p id=\"result\"></p>\n                    </div>\n                </div>\n            </div>\n        </div>\n    </div>\n    <button id=\"testButton\">\u6d4b\u8bd5\u63a5\u53e3</button>\n\n<div class=\"nested-div\"><h5>\u52a8\u6001\u6dfb\u52a0\u7684\u5d4c\u5957\u5143\u7d20</h5></div></div>\n\n<button id=\"clear-results\">\u6e05\u9664\u7ed3\u679c</button>\n\n\n<input id=\"test_input\" name=\"test input \">\n<button id=\"testb1\">\u6d4b\u8bd5</button>\n<button id=\"testb2\">\u6d4b\u8bd5</button>\n<button id=\"testb3\">\u6d4b\u8bd5</button>\n<button id=\"testb4\">\u6d4b\u8bd5</button>\n\n</body>",
  "id": "e011f41d-a9e4-471c-a770-0211a8712595",
  "pageUrl": "http://127.0.0.1:8858/",
  "position": {
    "x": 727,
    "y": 676
  },
  "target": {
    "absoluteXPath": "/html[1]/body[1]/div[4]/div[1]/div[1]/div[2]/div[1]/div[1]/button[1]",
    "className": "",
    "cssSelector": "button#addButton",
    "htmlContent": "<button id=\"addButton\">\u6dfb\u52a0\u65b0\u7684\u5d4c\u5957\u5143\u7d20</button>",
    "id": "addButton",
    "relativeXPath": "//button[@id='addButton']",
    "tagName": "button",
    "textContent": "\u6dfb\u52a0\u65b0\u7684\u5d4c\u5957\u5143\u7d20"
  },
  "timestamp": 1729489553907,
  "type": "click",
  "viewportSize": {
    "height": 1094,
    "width": 1912
  }
}


```


### If it helps you, please don’t be stingy with your star ✨