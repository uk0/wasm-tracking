#!/bin/bash


# Build the project

wasm-pack build --target web


sed -i '' 's/WatchDom_bg.wasm/\/static\/wasm\/wasm_monitor_bg.wasm/g' pkg/WatchDom.js

cp pkg/WatchDom.js /Users/firshme/Desktop/work/WatchDom_Server/static/js/wasm_monitor.js

cp pkg/WatchDom_bg.wasm /Users/firshme/Desktop/work/WatchDom_Server/static/wasm/wasm_monitor_bg.wasm


echo "build success!"