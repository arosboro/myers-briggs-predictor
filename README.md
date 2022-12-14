# Myers Briggs Predictor

### This could be a great skipping stone to your next stateful front-end project.

Please :star: (star) or :fork_and_knife: (fork) to show attribution or gratitude,
as always.

---

## Origins

Heavily inspired by https://github.com/Skeletonxf/easy-ml-mnist-wasm-example.

Credit Skeletonxf for the original wasm example and classic WebWorker
threading implementation.

From the above project's README.md:

    # Easy ML MNIST Web Assembly Example

    Simple MNIST Neural Network scaffold for demonstrating Rust code in the browser.

    Uses `wasm-pack` to build the web assembly. The webpage can be accessed by
    running the included Node.js server.

    ## About

    This project is a template for doing machine learning in the browser via Rust
    code loaded as WebAssembly. The code trains a simple feedforward neural network
    on a subset of the MNIST data using [Easy ML](https://crates.io/crates/easy-ml)
    with mini batching and automatic differentiation.

# Myers Briggs Predictor Examples

This repository utilizes the above scaffold with a revamped frontend implemented in
Vue.js. As the name suggests, instead of MNIST Number identification, Myers Briggs
Type Indicators (MBTIs) are focused on.

## Screenshot

<img src="../main/screenshots/webpage.png?raw=true" height="250px"></img>

## Current State of Affairs

1. Procedurally generated 16x16 pixel heatmaps with validated labels have been provided
2. Network weights for a network trained to 7 epochs have been provided, 0% loss.
3. Vue Components replecate the scaffold's User Interface functionality with TypeScript
4. WebWorker and data fetching ported from JavaScript to TypeScript
5. Vue.js niceties such as prettier ensure that the code smell is reduced

## Roadmap

1. Generate images procedurally in frontend from user text input
   - @see https://github.com/arosboro/mbti-random-forest/tree/arosboro-sandbox
2. Imutable Storage with Aleo Blockchain based contract deployed development node
3. Demonstrate ability to share Machine Learning statistics without exposing Text Input
4. Unit Tests
5. E2E Tests
6. Progressive Web Application (PWA) full support for offline-mode
7. Final implementation deployed to TestNet V3

## Project setup

_Recommended: Set prettier as the default formatter for saving the files in this project._

```
npm install
```

### Compiles and hot-reloads for development

```
npm run serve
```

### Compiles and minifies for production

```
npm run build
```

### Run your unit tests

```
npm run test:unit
```

### Run your end-to-end tests

```
npm run test:e2e
```

### Lints and fixes files

```
npm run lint
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).
