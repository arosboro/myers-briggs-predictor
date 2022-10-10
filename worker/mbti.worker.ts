console.log("Initializing Worker...");
import MBTI from "./data/mbti.data";
import {
  fetchNetworkWeights,
  getNetworkWeights,
  clearWeights,
} from "./data/mbti.network";
import { Raw, Sample, MBTIDataset } from "./data/mbti.types";
if (typeof importScripts === "function") {
  console.log("Worker: Starting, importScripts available.");
}

importScripts("/js/pkg/mbti_wasm.js");
console.log("Worker: importScripts done.");

type TrainingData = {
  labels: number[];
  images: Raw[];
};

const window = self;
const WIDTH = 16;
const HEIGHT = 16;
// const SAMPLES = 6483;
const TRAINING_SIZE = 5186;
const TESTING_SIZE = 1297;

let training: TrainingData;
let testing: TrainingData;

let memory: WebAssembly.Memory;

// Add functions to DedicatedWorkerGlobalScope
window.logProgress = (percent: number) => {
  postMessage({
    progress: true,
    percent: percent,
  });
};
window.logBatchLoss = (percent: number) => {
  postMessage({
    batchLoss: true,
    percent: percent,
  });
};
window.getNetworkWeights = () => {
  return getNetworkWeights();
};

(async () => {
  wasm_bindgen("pkg/mbti_wasm_bg.wasm").then(async (mbtiWasmModule) => {
    memory = mbtiWasmModule.memory;
    const { Dataset, Image, NeuralNetwork } = wasm_bindgen;
    const trainingDataset = Dataset.new_training();
    const testingDataset = Dataset.new_testing();
    let network = NeuralNetwork.new();

    const intoImage = (image: Raw) => {
      const imageWasm = Image.new();
      const pixels = new Float64Array(
        memory.buffer,
        imageWasm.buffer(),
        WIDTH * HEIGHT
      );
      // copy each pixel into the buffer exposed over Wasm to give it to
      // the Rust code
      for (let j = 0; j < WIDTH * HEIGHT; j++) {
        pixels[j] = image[j];
      }
      imageWasm.set_length();
      return imageWasm;
    };

    onmessage = async (event) => {
      const data = event.data;
      if (data.checkWeights) {
        const loadWeightsFromJson = data.loadWeightsFromJson;
        if (loadWeightsFromJson) {
          await fetchNetworkWeights();
        } else {
          clearWeights();
        }
        network = NeuralNetwork.new();
        // Signal that the UI can unblock;
        postMessage({
          loadedWeights: true,
        });
      }
      if (data.prepareDataset) {
        console.log("Worker: Preparing dataset...");
        const dataset: MBTIDataset | undefined = MBTI.set?.(
          TRAINING_SIZE,
          TESTING_SIZE
        );
        training = splitData(dataset?.training);
        testing = splitData(dataset?.test);

        for (let i = 0; i < training.images.length; i++) {
          const image = training.images[i];
          const label = training.labels[i];
          trainingDataset.add(intoImage(image), label);
        }

        for (let i = 0; i < testing.images.length; i++) {
          const image = testing.images[i];
          const label = testing.labels[i];
          testingDataset.add(intoImage(image), label);
        }

        postMessage({ datasetPrepared: true });
        postAccuracy();
      }
      if (data.trainEpoch) {
        console.log("Worker: Training...");
        window.logProgress(0);
        const loss = network.train(trainingDataset);
        postMessage({ trainedEpoch: true, loss: loss });
        postAccuracy();
      }
      if (data.requestCurrentImage) {
        console.log("Worker: Fetching current image...");
        const image: number = Math.min(
          Math.max(0, data.currentImage),
          TRAINING_SIZE - 1
        );
        const classification = network.classify(
          intoImage(training.images[image])
        );
        postMessage({
          currentImage: true,
          imageData: training.images[image],
          label: training.labels[image],
          index: image,
          classification: classification,
        });
      }
    };

    postMessage({
      loadedWorker: true,
    });

    const postAccuracy = () => {
      console.log("postAccuracy");
      const trainingAccuracy = network.accuracy(trainingDataset);
      const testingAccuracy = network.accuracy(testingDataset);
      postMessage({
        accuracy: true,
        trainingAccuracy: trainingAccuracy,
        testingAccuracy: testingAccuracy,
      });
    };
  });
})();

/**
 * Converts a dataset provided by the mbti package into two seperate
 * arrays, the first, an array of images, and the second an array of labels.
 */
const splitData = (dataset: Sample[] | undefined) => {
  const labels: number[] = [];
  const images: Raw[] = [];
  for (const entry of dataset || []) {
    images.push(entry.input);
    // dataset is encoded as 1-hot, ie an image of the first MBTI is represented as
    // [1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0], convert this to a single digit as data
    // transfer over wasm is slow
    labels.push(entry.output.indexOf(1));
  }
  return {
    labels: labels,
    images: images,
  };
};
