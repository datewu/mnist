import {
  $,
  cropScaleGetImageData,
  //  toFixed,
  chartConfigBuilder,
} from "./index.js";

const hiddenInputEl = $("hidden-input");
const hiddenBtnEl = $("hidden-btn");
const the_click_event = new Event("click");

const chart = chartConfigBuilder($("chart"));
// input.oninput not trigger for programmatic changes
// hiddenInputEl.oninput = function () {
//   let data = hiddenInputEl.value;
//   console.log("hidden value:", data);
//   chart.data.datasets[0].data = JSON.parse(data);
// };
// Handle programmatic changes via MutationObserver
const observer = new MutationObserver(function (mutations) {
  mutations.forEach(function (mutation) {
    if (
      mutation.type === "attributes" &&
      mutation.attributeName === "data-value"
    ) {
      // Get the current value of the custom attribute 'data-value'
      let currentDataValue = hiddenInputEl.getAttribute("data-value");
      chart.data.datasets[0].data = JSON.parse(currentDataValue || "[]");
      chart.update();
    }
  });
});

// Observe changes to the 'data-value' attribute
observer.observe(hiddenInputEl, {
  attributes: true,
  attributeFilter: ["data-value"],
});

const mainCanvasEl = $("main-canvas");
const scaledCanvasEl = $("scaled-canvas");
const cropEl = $("crop-canvas");
const mainContext = mainCanvasEl.getContext("2d", { willReadFrequently: true });
const cropContext = cropEl.getContext("2d", { willReadFrequently: true });
const scaledContext = scaledCanvasEl.getContext("2d", {
  willReadFrequently: true,
});

const fabricCanvas = new fabric.Canvas(mainCanvasEl, {
  isDrawingMode: true,
});

const backgroundColor = "rgba(255, 255, 255, 255)"; // White with solid alpha
fabricCanvas.freeDrawingBrush.width = 25;
fabricCanvas.backgroundColor = backgroundColor;

$("clear").onclick = function () {
  fabricCanvas.clear();
  fabricCanvas.backgroundColor = backgroundColor;
  fabricCanvas.renderAll();
  mainContext.clearRect(0, 0, mainCanvasEl.width, mainCanvasEl.height);
  scaledContext.clearRect(0, 0, scaledCanvasEl.width, scaledCanvasEl.height);

  chart.data.datasets[0].data = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
  chart.update();
};

let timeoutId;
let isDrawing = false;
let isTimeOutSet = false;

//wasm().then((module) => {
// const mnist = new Mnist();

async function fireOffInference() {
  clearTimeout(timeoutId);
  timeoutId = setTimeout(async () => {
    isTimeOutSet = true;
    fabricCanvas.freeDrawingBrush._finalizeAndAddPath();
    cropScaleGetImageData(mainContext, cropContext, scaledContext);
    hiddenBtnEl.dispatchEvent(the_click_event);
    isTimeOutSet = false;
  }, 50);
  isTimeOutSet = true;
}

fabricCanvas.on("mouse:down", function (event) {
  isDrawing = true;
});
fabricCanvas.on("mouse:up", async function (event) {
  isDrawing = false;
  await fireOffInference();
  //hiddenBtnEl.dispatchEvent(the_click_event);
});

fabricCanvas.on("mouse:move", async function (event) {
  if (isDrawing && isTimeOutSet == false) {
    await fireOffInference();
  }
});
//});
