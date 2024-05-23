const path = require('path');
const pkg = "./manifest-v3/js"
// const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const mode = 'development'
// const mode = 'production'
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
module.exports = {
    entry: pkg + '/content.js',
    output: {
        path: path.resolve(pkg, 'dist'),
        filename: 'better_spider_content.bundle.js',
    },
    mode: mode,
    // mode: "production",
    experiments: {
        asyncWebAssembly: true
    },
    plugins: [
        // new HtmlWebpackPlugin(),
        // new WasmPackPlugin({
        //     crateDirectory: path.resolve(pkg, "wasm"),
        //     // The same as the `--out-name` option for `wasm-pack`
        //     outName: "better_spider",
        //     forceMode: mode,
        //     args: "",
        //     extraArgs: "--no-typescript --target bundler",
        //     outDir: path.resolve(pkg, 'wasm')

        // }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    devtool: 'source-map'
};