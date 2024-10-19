use wasm_bindgen::prelude::*;
use web_sys::{console, MutationObserver, MutationObserverInit, Element, EventTarget, Document, Window, Event, Location};
use wasm_bindgen::JsCast;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::rc::Rc;
use reqwest::Client;
use uuid::Uuid;
// 添加全局 debug 常量
static DEBUG: bool = false;
static DELETE_SCRIPTS: bool = true;  // 判断是否去除html内的script标签
static DELETE_BODY: bool = false;  // 判断是否包含body进行推送

#[wasm_bindgen]
pub struct WasmObserver {
    mutation_observer: MutationObserver,
    recent_changes: Rc<RefCell<Vec<Value>>>,
}

fn get_css_selector(element: &Element) -> String {
    let mut path = Vec::new();
    let mut current = Some(element.clone());

    while let Some(elem) = current {
        let mut selector = elem.tag_name().to_lowercase();

        let id = elem.id();
        if !id.is_empty() {
            selector.push_str(&format!("#{}", id));
            path.push(selector);
            break;
        }

        if let Some(parent) = elem.parent_element() {
            let siblings = parent.children();
            let mut nth_child = 1;
            for i in 0..siblings.length() {
                if siblings.item(i) == Some(elem.clone()) {
                    break;
                }
                if siblings.item(i).map(|s| s.tag_name()) == Some(elem.tag_name()) {
                    nth_child += 1;
                }
            }
            if nth_child > 1 {
                selector.push_str(&format!(":nth-of-type({})", nth_child));
            }
        }

        path.push(selector);
        current = elem.parent_element();
    }

    path.reverse();
    path.join(" > ")
}

fn get_xpath(element: &Element) -> String {
    let mut path = Vec::new();
    let mut current = Some(element.clone());

    while let Some(elem) = current {
        let id = elem.id();
        if !id.is_empty() {
            path.push(format!("*[@id='{}']", id));
            break;
        }

        if let Some(parent) = elem.parent_element() {
            let siblings = parent.children();
            let mut nth_child = 1;
            for i in 0..siblings.length() {
                if siblings.item(i) == Some(elem.clone()) {
                    break;
                }
                if siblings.item(i).map(|s| s.tag_name()) == Some(elem.tag_name()) {
                    nth_child += 1;
                }
            }
            path.push(format!("{}[{}]", elem.tag_name().to_lowercase(), nth_child));
        } else {
            path.push(elem.tag_name().to_lowercase());
        }

        current = elem.parent_element();
    }

    path.reverse();
    format!("//{}", path.join("/"))
}

fn create_target_data(element: &Element) -> Value {
    json!({
        "tagName": element.tag_name().to_lowercase(),
        "id": element.id(),
        "className": element.class_name(),
        "xpath": get_xpath(element),
        "cssSelector": get_css_selector(element),
        "textContent": element.text_content().unwrap_or_default().trim(),
        "htmlContent": element.outer_html(),
    })
}

fn get_mouse_position(event: &Event) -> Option<(i32, i32)> {
    let event_obj: &JsValue = event.as_ref();
    if js_sys::Reflect::has(event_obj, &JsValue::from_str("clientX")).unwrap_or(false) &&
        js_sys::Reflect::has(event_obj, &JsValue::from_str("clientY")).unwrap_or(false) {
        let client_x = js_sys::Reflect::get(event_obj, &JsValue::from_str("clientX"))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|x| x as i32);
        let client_y = js_sys::Reflect::get(event_obj, &JsValue::from_str("clientY"))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|y| y as i32);

        if let (Some(x), Some(y)) = (client_x, client_y) {
            Some((x, y))
        } else {
            None
        }
    } else {
        None
    }
}
async fn send_event_data(data: Value,location: Location) {
    let client = Client::new();
    let protocol = location.protocol().unwrap_or_else(|_| "http".to_string());
    let host = location.host().unwrap_or_else(|_| "localhost".to_string());
    let url = format!("{}//{}/event_process", protocol, host);
    console::log_1(&JsValue::from_str(&format!("event_process url {:?}", url)));

    match client.post(url)
        .json(&data)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                console::log_1(&JsValue::from_str("Event data sent successfully"));
            } else {
                console::log_1(&JsValue::from_str(&format!("Failed to send event data: {:?}", response.status())));
            }
        },
        Err(e) => {
            console::log_1(&JsValue::from_str(&format!("Error sending event data: {:?}", e)));
        }
    }
}


#[wasm_bindgen]
impl WasmObserver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmObserver, JsValue> {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let recent_changes = Rc::new(RefCell::new(Vec::new()));
        let recent_changes_clone = Rc::clone(&recent_changes);

        let mutation_callback = Closure::wrap(Box::new(move |mutations: Vec<web_sys::MutationRecord>, _observer: web_sys::MutationObserver| {
            for mutation in mutations.iter() {
                let target = mutation.target().unwrap();
                if let Some(element) = target.dyn_ref::<Element>() {
                    let change = create_change_data("mutation", element, &window, &document, None);
                    recent_changes_clone.borrow_mut().push(change.clone());
                    // console::log_1(&JsValue::from_str(&format!("DOM Change: {}", serde_json::to_string_pretty(&change).unwrap())));
                }
            }
        }) as Box<dyn FnMut(Vec<web_sys::MutationRecord>, web_sys::MutationObserver)>);

        let mutation_observer = MutationObserver::new(mutation_callback.as_ref().unchecked_ref())?;
        mutation_callback.forget();

        let observer = WasmObserver {
            mutation_observer,
            recent_changes,
        };

        if DELETE_SCRIPTS {
            observer.remove_body_scripts();
        }


        observer.setup_event_listeners()?;

        Ok(observer)
    }

    // 新添加的方法，用于删除 body 内的 script 标签
    fn remove_body_scripts(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let scripts = body.get_elements_by_tag_name("script");
        let scripts_length = scripts.length();

        for i in (0..scripts_length).rev() {
            if let Some(script) = scripts.item(i) {
                if let Some(parent) = script.parent_node() {
                    let _ = parent.remove_child(&script);
                    console::log_1(&JsValue::from_str("Removed a script tag from body"));
                }
            }
        }
    }

    fn setup_event_listeners(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let recent_changes = Rc::clone(&self.recent_changes);
        let location = window.location();

        let event_types = vec!["click", "input", "scroll", "mousemove", "keypress"];

        for event_type in event_types {
            let recent_changes_clone = Rc::clone(&recent_changes);
            let window_clone = window.clone();
            let document_clone = document.clone();
            let event_type_clone = event_type.to_string();
            let location_clone = location.clone();


            let callback = Closure::wrap(Box::new(move |event: Event| {
                if let Some(target) = event.target() {
                    if let Some(element) = target.dyn_ref::<Element>() {
                        // 检查是否为 mousemove 事件且 DEBUG 为 false
                        if event_type_clone == "mousemove" && !DEBUG {
                            return;
                        }
                        let mouse_pos = get_mouse_position(&event);
                        let change = create_change_data(&event_type_clone, element, &window_clone, &document_clone, mouse_pos);
                        recent_changes_clone.borrow_mut().push(change.clone());
                        // console::log_1(&JsValue::from_str(&format!("Event: {}", serde_json::to_string_pretty(&change).unwrap())));

                        // 发送数据到 API
                        wasm_bindgen_futures::spawn_local(send_event_data(change, location_clone.clone()));                    }
                }
            }) as Box<dyn FnMut(Event)>);

            document.add_event_listener_with_callback(event_type, callback.as_ref().unchecked_ref())?;
            callback.forget();
        }

        Ok(())
    }


    pub fn observe(&self, target: &web_sys::Node) -> Result<(), JsValue> {
        let mut mutation_options = MutationObserverInit::new();
        mutation_options.child_list(true)
            .attributes(true)
            .character_data(true)
            .subtree(true);

        self.mutation_observer.observe_with_options(target, &mutation_options)?;

        Ok(())
    }

    pub fn disconnect(&self) {
        self.mutation_observer.disconnect();
    }

    pub fn get_recent_changes(&self) -> String {
        let changes = self.recent_changes.borrow();
        serde_json::to_string_pretty(&*changes).unwrap()
    }

    pub fn clear_recent_changes(&self) {
        self.recent_changes.borrow_mut().clear();
    }
}

fn create_change_data(event_type: &str, element: &Element, window: &Window, document: &Document, mouse_pos: Option<(i32, i32)>) -> Value {
   let mut change =  json!({
        // "body": document.body().unwrap().outer_html(),
        "id": Uuid::new_v4().to_string(),
        "timestamp": js_sys::Date::now() as i64,
        "type": event_type,
        "target": create_target_data(element),
        "position": {
            "x": mouse_pos.map(|(x, _)| x).unwrap_or_else(|| window.scroll_x().unwrap_or(0.0) as i32),
            "y": mouse_pos.map(|(_, y)| y).unwrap_or_else(|| window.scroll_y().unwrap_or(0.0) as i32),
        },
        "viewportSize": {
            "width": window.inner_width().unwrap().as_f64().unwrap() as i32,
            "height": window.inner_height().unwrap().as_f64().unwrap() as i32,
        },
        "pageUrl": window.location().href().unwrap_or_default(),
    });

    // 只有在 DELETE_BODY 为 false 时才添加 body 字段
    if !DELETE_BODY {
        if let Some(body) = document.body() {
            change["body"] = Value::String(body.outer_html());
        }
    }
    change
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console::log_1(&"WASM Observer module initialized".into());
    Ok(())
}