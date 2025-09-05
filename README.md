# rolldown-wasm-issue

A minimal reproduction case for a bug in [rolldown](https://github.com/rolldown/rolldown/) where JavaScript functions are not correctly passed to WASM functions.

## Issue Description

When using [rolldown](https://github.com/rolldown/rolldown/) to bundle a web application that calls WASM functions with JavaScript function parameters, the JS functions are stripped from the parameters during the build process.


## Reproduction
1. Install [Rust](https://www.rust-lang.org/), [wasm-pack](https://github.com/drager/wasm-pack), and [yarn](https://classic.yarnpkg.com/en/docs/install#windows-stable).
2. Setup this minimal reproduction case
    ```sh
    git clone https://github.com/ParaN3xus/rolldown-wasm-issue
    cd rolldown-wasm-issue
    wasm-pack build
    yarn install
    ```
3. See the result
    ```sh
    yarn build && yarn preview
    ```
    - Expected Behavior (can be verified by running `yarn dev` or using rollup)
        ```
        get_v returned: 1
        ```
    - Actual Behavior
        ```
        failed: Failed to deserialize init opts: Error: invalid type: JsValue(Object({"v":1})), expected struct WasmFuncCallParameters
        ```
      The error indicates that the `f` function parameter is missing from the object passed to WASM.
