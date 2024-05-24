# Requirement
### execute the flowing command to install dependencies
* Install [rust](https://www.rust-lang.org/)
* [ wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) `cargo install wasm-pack`
* [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli) `cargo install dioxus-cli`
* [webpack](https://www.webpackjs.com/guides/installation/) `npm install --save-dev webpack`


# Development WebUI

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

# Development Chrome Extension
Run the following command in the root of the project to build the chrome extension:

`npm install && npm run build2`


### Package the extension(todo)



## Project Structure

```
├── Cargo.toml
├── Dioxus.toml
├── index.html // Custom HTML is needed for this, to load the SW and manifest.
├── LICENSE
├── public
│   ├── favicon.ico
│   ├── logo_192.png
│   ├── logo_512.png
│   ├── manifest.json // The manifest file - edit this as you need to.
│   └── sw.js // The service worker - you must edit this for actual projects.
├── README.md
└── src
    └── main.rs
```
