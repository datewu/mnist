use crate::m::M;
use leptos::html::Canvas;
use leptos::prelude::*;
use leptos::task::spawn_local;

use leptos_meta::{provide_meta_context, HashedStylesheet, Link, MetaTags, Script, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="A MNIST demo" />

        <Script type_="module" src="index.js" />
        // content for this welcome page

        <Script
            src="https://cdn.jsdelivr.net/npm/fabric@5.3.0/dist/fabric.min.js"
            integrity="sha256-SPjwkVvrUS/H/htIwO6wdd0IA8eQ79/XXNAH+cPuoso="
            crossorigin="anonymous"
        />

        <Script
            src="https://cdn.jsdelivr.net/npm/chart.js@4.2.1/dist/chart.umd.min.js"
            integrity="sha256-tgiW1vJqfIKxE0F2uVvsXbgUlTyrhPMY/sm30hh/Sxc="
            crossorigin="anonymous"
        />

        <Script
            src="https://cdn.jsdelivr.net/npm/chartjs-plugin-datalabels@2.2.0/dist/chartjs-plugin-datalabels.min.js"
            integrity="sha256-IMCPPZxtLvdt9tam8RJ8ABMzn+Mq3SQiInbDmMYwjDg="
            crossorigin="anonymous"
        />

        <Link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/npm/normalize.min.css@8.0.1/normalize.min.css"
            integrity="sha256-oeib74n7OcB5VoyaI+aGxJKkNEdyxYjd2m3fi/3gKls="
            crossorigin="anonymous"
        />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

// export function rgba2gray(data) {
//   let converted = new Float32Array(data.length / 4);
//
//   // Data is stored as [r0,g0,b0,a0, ... r[n],g[n],b[n],a[n]] where n is number of pixels.
//   for (let i = 0; i < data.length; i += 4) {
//     let r = 255 - data[i]; // red
//     let g = 255 - data[i + 1]; // green
//     let b = 255 - data[i + 2]; // blue
//     let a = 255 - data[i + 3]; // alpha
//
//     // Use RGB grayscale coefficients (https://imagej.nih.gov/ij/docs/menus/image.html)
//     let y = 0.299 * r + 0.587 * g + 0.114 * b;
//     converted[i / 4] = y; // 4 times fewer data points but the same number of pixels.
//   }
//   return converted;
// }
//

fn rgba2gray(data: &[u8]) -> Vec<f64> {
    let mut converted = vec![0.0; data.len() / 4];
    let mut i = 0;
    let l = data.len();
    while i + 2 < l {
        let r = 255 - data[i]; // red
        let g = 255 - data[i + 1]; // green
        let b = 255 - data[i + 2]; // blue
                                   //   let a = 255 - data[i + 3]; // alpha
                                   // Use RGB grayscale coefficients (https://imagej.nih.gov/ij/docs/menus/image.html)
        let y = 0.299 * (r as f64) + 0.587 * (g as f64) + 0.114 * (b as f64);
        converted[i / 4] = y; // 4 times fewer data points but the same number of pixels.

        i += 4;
    }
    converted
}

#[component]
fn TheCanvas() -> impl IntoView {
    let m = M::default();
    let (inference, set_inference) = signal(String::new());
    let can = NodeRef::<Canvas>::new();
    let handle_inference = move |_| {
        if let Some(canvas) = can.get() {
            let context: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .expect("should have 2d context")
                .expect("should not be null")
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            let image_data = context.get_image_data(0.0, 0.0, 28.0, 28.0).unwrap().data();
            //context.restore();
            let grayscale = rgba2gray(&image_data);
            // leptos::logging::log!("grayscale: len: {}, data: {:?}", grayscale.len(), grayscale);
            let mut m = m.clone();
            spawn_local(async move {
                let result = m.inference(grayscale.as_slice()).await.unwrap();
                set_inference(format!("{:?}", result));
            });
        };
    };
    view! {
        // <input id="hidden-input" prop:value=inference />
        <input id="hidden-input" hidden data-value=inference />
        <button id="hidden-btn" on:click=handle_inference>
            Click me
        </button>
        <canvas
            node_ref=can
            id="scaled-canvas"
            width="28"
            height="28"
            style="border: 1px solid #aaa; width: 100px; height: 100px"
        />
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>Burn MNIST Inference Demo</h1>
        <div class="table">
            <div class="row">
                <div class="cell">Draw a digit here</div>
                <div class="cell">Cropped and scaled</div>
                <div class="cell">Probability result</div>
            </div>
            <div class="row">
                <div class="cell">
                    <canvas
                        id="main-canvas"
                        width="300"
                        height="300"
                        style="border: 1px solid #aaa"
                    ></canvas>
                </div>
                <div class="cell">
                    <TheCanvas />
                    <canvas id="crop-canvas" width="28" height="28" style="display: none"></canvas>
                </div>
                <div class="cell">
                    <canvas
                        id="chart"
                        style="border: 1px solid #aaa; width: 600px; height: 300px"
                    ></canvas>
                </div>
            </div>
            <div class="row">
                <div>
                    <button id="clear">Clear</button>
                </div>
            </div>
        </div>

        <div></div>
        <script type="module" src="run.js"></script>
    }
}
