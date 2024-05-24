const path = require('path');
const fs = require('fs');
const pkg = "./manifest-v3/src"
// const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
// const mode = 'development';
const mode = 'production';
const devMode = mode == "production";
const WebpackShellPluginNext = require('webpack-shell-plugin-next');
const wasmModulePat = /fetch\((.*)\s*\+\s*\"\"\s*\+\s*(.*)\s*\+\s*\"\.module\.wasm\"\)/;

function delDir(path) {
    let files = [];
    if (fs.existsSync(path)) {
        files = fs.readdirSync(path);
        files.forEach((file, index) => {
            let curPath = path + "/" + file;
            if (fs.statSync(curPath).isDirectory()) {
                delDir(curPath); //递归删除文件夹
            } else {
                fs.unlinkSync(curPath); //删除文件
            }
        });
        fs.rmdirSync(path);
    }
}
module.exports = {
    entry: pkg + '/content.js',
    output: {
        path: path.resolve(pkg, 'dist'),
        filename: 'better_spider_content.bundle.js',
        clean: true
    },
    mode: mode,
    // mode: "production",
    experiments: {
        asyncWebAssembly: true
    },
    plugins: [
        new WebpackShellPluginNext({
            dev: devMode,
            onBuildStart: {
                scripts: [
                    () => {
                        const paths = [pkg + '/wasm', pkg + '/dist']
                        console.log("removing old files: " + paths.join(','))
                        for (let _path of paths) {
                            delDir(path.resolve(_path))
                        }
                        console.log("removing old files: " + paths.join(',') + ',done!')
                    },
                    'wasm-pack build --release --no-typescript --out-dir "./manifest-v3/src/wasm" --out-name "better_spider" --target bundler'
                ],
                blocking: true,
                parallel: false
            },
            onBuildEnd: {
                scripts: [
                    () => {
                        console.log('replacing wasm fetch')
                        const bundle_js_file_path = path.resolve(pkg, 'dist', 'better_spider_content.bundle.js');
                        const content = fs.readFileSync(bundle_js_file_path, 'utf8');
                        let rep = content.replace(wasmModulePat, 'fetch(chrome.runtime.getURL("./src/dist/" + $2 + ".module.wasm"))')
                        fs.writeFileSync(bundle_js_file_path, rep);
                        console.log('replacing wasm fetch, done！')
                    },
                    () => {
                        console.log('copying css files')
                        const cssfile = 'better-spider.css';
                        const from = path.resolve(__dirname, 'assets', cssfile);
                        const to = path.resolve(pkg, cssfile);
                        fs.copyFileSync(from, to);
                        console.log('copying css files, done!')
                    }
                ],
                blocking: true,
                parallel: false
            },
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        }),
    ],

    devtool: 'source-map'
};