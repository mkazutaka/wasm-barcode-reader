const CopyWebpackPlugin = require("copy-webpack-plugin");
const WriteFilePlugin = require('write-file-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = {
    entry: {
        "bootstrap": "./bootstrap.js",
        "demo": "./src/demo/index.js",
        "bench": "./src/bench/index.js",
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name].js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin([
            'index.html',
            'src/demo/demo.html',
            'src/bench/bench.html'
        ]),
    ],
};
