const path = require('path');
const fs = require('fs');
const pkg = "./manifest-v3/js"
// const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const mode = 'development';
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const devMode = mode == "production";
// const mode = 'production'
const WebpackShellPluginNext = require('webpack-shell-plugin-next');

function delDir(path){
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
            dev:devMode,
            onBuildStart: {
                scripts: [
                    () => {
                        const paths = [pkg + '/wasm', pkg + '/dist']
                        console.log("removing old files: "+ paths.join(','))
                        for (let _path of paths) {
                            delDir(path.resolve(_path))
                        }
                        console.log("removing old files: "+ paths.join(',')+',done!')
                    },
                    'wasm-pack build --release --no-typescript --out-dir "./manifest-v3/js/wasm" --out-name "better_spider" --target bundler'
                ],
                blocking: true,
                parallel: false
            },
            onBuildEnd: {
                scripts: ['echo "Webpack onBuildEnd End"'],
                blocking: true,
                parallel: false
            },
            onBuildExit: {
                scripts: ['echo "Webpack onBuildExit End"'],
                blocking: true,
                parallel: false
            },
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ].concat(devMode ? [] : [new MiniCssExtractPlugin()]),

    module: {
        rules: [
            {
                // If you enable `experiments.css` or `experiments.futureDefaults`, please uncomment line below
                // type: "javascript/auto",
                test: /\.(le|c)ss$/i,
                use: [
                    devMode ? "style-loader" : MiniCssExtractPlugin.loader,
                    "css-loader",
                    "less-loader",
                ],
            },
        ],
    },
    devtool: 'source-map'
};