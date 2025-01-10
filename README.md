# MNIST Inference Web App Using burn and leptos

A simple web app for inference with a trained MNIST model using Burn and Leptos.

Port [burn mnist inference](https://github.com/tracel-ai/burn/tree/main/examples/mnist-inference-web)
which was a simple web page built by wasm-pack to a Leptos app.

I can use trunk as well (ie the `csr` feature of Leptos),
but using Leptos `ssr` feature bring me a lot of fun.

The origin Burn demo Readme document is [here](https://github.com/tracel-ai/burn/tree/main/examples/mnist-inference-web#readme).
Defintily check it out :)

## Run

```shell
# --release mode
make serve

# develop mode
make watch

```

## What did you do actually?

1. I move the origin lib crate to a `m` module,
and lose the origin `web` module.

2. Kind of "replace" the `web` module and `index.html` with `leptos` components.
3. Tweak index.js and index.html(mainly extract
the `<script type="module">....</script>` to a `run.js` file) to make it work.
4. Using `leptos` to build the web app.

[视频](https://www.bilibili.com/video/BV1dnrkY9En9/)

## wasm-pack

You can checkout [branch](https://github.com/datewu/mnist/tree/wasm-pack)
`wasm-pack` which is almost idential to the origin `burn` example
to see how to build the wasm-pack example.

Note: there is a Makefile to simpfy the build and serve process.

```shell
git checkout wasm-pack
make run
```
