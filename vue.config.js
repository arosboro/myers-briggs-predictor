const { defineConfig } = require("@vue/cli-service");
const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");
module.exports = defineConfig({
  transpileDependencies: true,
  configureWebpack: {
    mode: "production",
    output: {
      path: dist,
      filename: "[name].js"
    },
    devServer: {
      static: {
        directory: dist
      }
    },
    plugins: [
      new CopyPlugin({
          patterns: [
              path.resolve(__dirname, "www")
          ]
      }),
  
      new WasmPackPlugin({
        crateDirectory: __dirname,
        extraArgs: '--target no-modules', // our wasm is used in a Web Worker not as an ES6 module
        outName: 'mbti_wasm',
        outDir: "www/pkg",
      }),
    ]
  }
});
