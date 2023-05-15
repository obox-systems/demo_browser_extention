# Google Chrome extension

## Abstract
In this project, I developed a simple Google Chrome extension that prints the URLs of all opened tabs of the browser in the extension HTML window, when the button of the extension is pressed. This extension was written in Rust and compiled to WASM to use the Rust functions in the Javascript module of the extension later. 

In this project all Rust functions are in the `src` folder, they perform the main logic of a plugin. `manifest.json` is the file with all settings of the plugin and is the first file that our browser sees after installing the plugin. `popup.html` is a file with HTML markup of a plugin window. And the `script.js` file contains Javascript functions that are exported into Rust.

I used a `wasm-bindgen` crate to compile functions from Rust module into a wasm file. I also used a `web-sys` crate to get the information about the URLs of all opened tabs of the browser.

It can be used to get a list of all open tabs in your browser and will be useful if you have a lot of tabs open and want to find a particular one.

## How to run

1. Install [Rust](https://rustup.rs/)
2. Install Wasm-pack
```bash
cargo install wasm-pack
```
3. Run the app
```bash
wasm-pack build --target web
```
4. Open Google Chrome browser and [install the extension](https://support.google.com/chrome_webstore/answer/2664769?hl=en)
5. Press the button to see the URLs in the window

## Example

Here is an example of extension work:

![res](Image/tabs_gif.gif)
