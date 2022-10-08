const { defineConfig } = require("@vue/cli-service");
const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");
function resolve(dir) {
  return path.join(__dirname, dir);
}
module.exports = defineConfig({
  transpileDependencies: ["mbti-worker", "mbti-data"],
  configureWebpack: {
    entry: {
      app: "./src/main.ts",
      "mbti.worker": "./worker/mbti.worker.ts",
      "data/mbti.data": "./worker/data/mbti.data.ts",
    },
    output: {
      path: dist,
      filename: "[name].js",
    },
    resolve: {
      alias: {
        "@": resolve("src"),
        "@worker": resolve("worker"),
        "@data": resolve("worker/data"),
      },
    },
    optimization: {
      runtimeChunk: "single",

      splitChunks: {
        chunks: "async",
        minSize: 30000,
        minChunks: 1,
        maxAsyncRequests: 5,
        maxInitialRequests: 3,
        name: (chunk) =>
          chunk.name in ["app", "mbti.worker", "data/mbti.data"]
            ? chunk.name
            : "chunk",

        cacheGroups: {
          default: {
            minChunks: 2,
            priority: -20,
            reuseExistingChunk: true,
          },

          vendors: {
            test: /[\\/]node_modules[\\/]/,
            priority: -10,
          },
        },
      },
    },
    plugins: [
      new CopyPlugin({
        patterns: [
          {
            from: "worker/data/mbti_samples.json",
            to: "data/mbti_samples.json",
          },
          { from: "worker/data/network.json", to: "data/network.json" },
          { from: "worker/pkg", to: "js/pkg" },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: __dirname,
        extraArgs: "--target no-modules", // our wasm is used in a Web Worker not as an ES6 module
        outName: "mbti_wasm",
        outDir: "worker/pkg",
      }),
    ],
  },
  chainWebpack: (config) => {
    config.plugin("html").tap((args) => {
      args[0].inject = false;

      return args;
    });
  },
});
