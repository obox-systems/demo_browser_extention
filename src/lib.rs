#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! In this project, I developed a simple Google Chrome extension that prints the URLs of all opened tabs of the browser in the extension HTML window, when the button of the extension is pressed. This extension was written in Rust and compiled to WASM to use the Rust functions in the Javascript module of the extension later.
//! 
//! In this project all Rust functions are in the src folder, they perform the main logic of a plugin. manifest.json is the file with all settings of the plugin and is the first file that our browser sees after installing the plugin. popup.html is a file with HTML markup of a plugin window. And the script.js file contains Javascript functions that are exported into Rust.
//! 
//! I used a wasm-bindgen crate to compile functions from Rust module into a wasm file. I also used a web-sys crate to get the information about the URLs of all opened tabs of the browser.
//! 
//! It can be used to get a list of all open tabs in your browser and will be useful if you have a lot of tabs open and want to find a particular one.

use wasm_bindgen::prelude::*;

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/script.js")]
extern "C" {
    fn getActiveTabUrl(callback: &Closure<dyn FnMut(Vec<JsValue>)>);
    fn changeContent(content: JsValue);
}

/// this function takes callback from js function and transforms it to pretty view
#[wasm_bindgen]
pub fn my_plugin_function() {
    // let closure = Closure::wrap(Box::new(|url: JsValue| {
    //   console_log!("qwe");
    //   let url_str = url.as_string().unwrap();
    //   console_log!("{}", url_str);
    //}) as Box<dyn Fn(JsValue)>);
    let closure = Closure::new(move |url: Vec<JsValue>| {
        let url_str = url
            .into_iter()
            .filter_map(|url| url.as_string())
            .collect::<Vec<_>>();
        console_log!("{:?}", url_str);
        let pretty_html_list = format!(
            r#"<ul class="styled-list">
      {}
      </ul>"#,
            url_str
                .into_iter()
                .map(|url| format!("<li>{url}</li>"))
                .collect::<Vec<_>>()
                .join("")
        );
        changeContent(pretty_html_list.into());
    });
    getActiveTabUrl(&closure);

    closure.forget();
}
