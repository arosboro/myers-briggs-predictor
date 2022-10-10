<template>
  <div class="training-grounds">
    <h1>MBTI Training Grounds</h1>
    <TrainingActions
      @train-epoch="blockImageViewerUI"
      @load-weights="loadWeights"
      :worker="worker"
      :prepareButtonDisabled="prepareButtonDisabled"
      :trainButtonDisabled="trainButtonDisabled"
      :loadWeightsFromJson="loadWeightsFromJson"
      :loadWeightsDisabled="loadWeightsDisabled"
    />
    <ImageViewer
      @draw-previous-image="drawPreviousImage"
      @draw-next-image="drawNextImage"
      @draw-negative="drawNegativeImage"
      :nextButtonDisabled="nextButtonDisabled"
      :previousButtonDisabled="previousButtonDisabled"
      :drawModeDisabled="drawModeDisabled"
      :drawNegative="drawNegative"
      :sample_index="sample_index"
      :sample_label="sample_label"
      :classification="classification"
    />
  </div>
  <ProgressBar
    :percent="trainingPercent"
    :content="trainingPercentString + '%'"
  />
  <ScrollableChart :pointsPlotted="pointsPlotted" />
  <p>
    <span>{{ trainingAccuracy }}</span>
    <span>{{ testingAccuracy }}</span>
  </p>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import TrainingActions from "@/components/TrainingActions.vue";
import ImageViewer from "@/components/ImageViewer.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import ScrollableChart from "@/components/ScrollableChart.vue";

export enum MBType {
  I = 0b10000000,
  E = 0b01000000,
  S = 0b00100000,
  N = 0b00010000,
  T = 0b00001000,
  F = 0b00000100,
  J = 0b00000010,
  P = 0b00000001,
}

export enum MBTILabels {
  "ENFP" = 0b01010101, // "ENFP",
  "ENFJ" = 0b01010110, // "ENFJ",
  "ENTP" = 0b01011001, // "ENTP",
  "ENTJ" = 0b01011010, // "ENTJ",
  "ESFP" = 0b01100101, // "ESFP",
  "ESFJ" = 0b01100110, // "ESFJ",
  "ESTP" = 0b01101001, // "ESTP",
  "ESTJ" = 0b01101010, // "ESTJ",
  "INFP" = 0b10010101, // "INFP",
  "INFJ" = 0b10010110, // "INFJ",
  "INTP" = 0b10011001, // "INTP",
  "INTJ" = 0b10011010, // "INTJ",
  "ISFP" = 0b10100101, // "ISFP",
  "ISFJ" = 0b10100110, // "ISFJ",
  "ISTP" = 0b10101001, // "ISTP",
  "ISTJ" = 0b10101010, // "ISTJ",
}

const MAX_EPOCHS = 10;

@Options({
  components: {
    TrainingActions,
    ImageViewer,
    ProgressBar,
    ScrollableChart,
  },
  data: () => ({
    worker: null,
    epochs: 0,
    prepareButtonDisabled: true,
    trainButtonDisabled: true,
    drawModeDisabled: true,
    nextButtonDisabled: true,
    previousButtonDisabled: true,
    sample_index: 0,
    sample_label: "None",
    sample_image: 0,
    classification: "None",
    drawNegative: false,
    trainingAccuracy: "",
    testingAccuracy: "",
    trainingPercent: 0,
    trainingPercentString: "0%",
    pointsPlotted: [],
    loadWeightsFromJson: false,
    loadWeightsDisabled: false,
  }),
  methods: {
    init: function () {
      // don't reset the worker incase it has already fetched assets.
      // this.worker = null;
      this.epochs = 0;
      this.prepareButtonDisabled = true;
      this.trainButtonDisabled = true;
      this.drawModeDisabled = true;
      this.nextButtonDisabled = true;
      this.previousButtonDisabled = true;
      this.sample_image = [];
      this.sample_label = "None";
      this.sample_index = 0;
      this.classification = "None";
      this.drawNegative = false;
      this.trainingAccuracy = "";
      this.testingAccuracy = "";
      this.trainingPercent = 0;
      this.trainingPercentString = "0%";
      this.pointsPlotted = [];
      this.loadWeightsFromJson = false;
      this.loadWeightsDisabled = false;
    },
    drawCurrentImage: function () {
      console.log("Drawing current image");
      this.worker.postMessage({
        requestCurrentImage: true,
        currentImage: this.sample_index,
      });
    },
    drawPreviousImage: function () {
      console.log("Drawing previous image");
      this.sample_index -= 1;
      this.worker.postMessage({
        requestCurrentImage: true,
        currentImage: this.sample_index,
      });
    },
    drawNextImage: function () {
      console.log("Drawing next image");
      this.sample_index += 1;

      this.worker.postMessage({
        requestCurrentImage: true,
        currentImage: this.sample_index,
      });
    },
    drawNegativeImage: function () {
      console.log("Drawing negative image");
      this.drawNegative = !this.drawNegative;
      this.worker.postMessage({
        requestCurrentImage: true,
        currentImage: this.sample_index,
      });
    },
    loadWeights: async function () {
      console.log("Loading weights");
      const loadWeightsFromJson = !this.loadWeightsFromJson;
      this.init();
      this.resetCanvas();
      this.loadWeightsFromJson = loadWeightsFromJson;
      this.worker.postMessage({
        checkWeights: true,
        loadWeightsFromJson: loadWeightsFromJson,
      });
    },
    getColor: function (color: number) {
      if (this.drawNegative) {
        return `rgb(${color * 255}, ${color * 255}, ${color * 255})`;
      } else {
        return `rgb(${255 - color * 255}, ${255 - color * 255}, ${
          255 - color * 255
        })`;
      }
    },
    blockImageViewerUI: function () {
      this.loadWeightsDisabled = true;
      this.trainButtonDisabled = true;
      this.nextButtonDisabled = true;
      this.previousButtonDisabled = true;
      this.drawModeDisabled = true;
    },
    unBlockImageViewerUI: function () {
      this.loadWeightsDisabled = false;
      this.trainButtonDisabled = false;
      this.nextButtonDisabled = false;
      this.previousButtonDisabled = false;
      this.drawModeDisabled = false;
    },
    resetCanvas: function () {
      if (typeof this.canvasContext !== "undefined") {
        const canvas = document.getElementById("image") as HTMLCanvasElement;
        this.canvasContext.clearRect(0, 0, canvas.width, canvas.height);
      }
    },
    initWorker: function () {
      this.worker = null;
      this.worker = new Worker(
        new URL("./../../worker/mbti.worker.ts", import.meta.url),
        { type: "classic" }
      );
      this.worker.onerror = (ev: ErrorEvent) => {
        console.log("Error in this.worker");
        console.error(ev);
      };
      this.worker.onmessage = (event: MessageEvent) => {
        let data = event.data;
        if (data.loadedWeights) {
          console.log("loadedWeights");
          this.prepareButtonDisabled = false;
        }
        if (data.loadedWorker) {
          console.log("loadedWorker");
          this.prepareButtonDisabled = false;
        }
        if (data.datasetPrepared) {
          console.log("datasetPrepared");
          this.drawCurrentImage();
          this.prepareButtonDisabled = true;
        }
        if (data.currentImage) {
          console.log("currentImage");
          this.sample_image = data.imageData;
          this.nextButtonDisabled = false;
          this.previousButtonDisabled = false;
          this.drawModeDisabled = false;
          // Draw image data to canvas
          let color = this.sample_image[0];
          const WIDTH = 16;
          const HEIGHT = 16;
          const canvas: HTMLElement | null = document.getElementById("image");
          if (canvas instanceof HTMLCanvasElement) {
            canvas.width = WIDTH;
            canvas.height = HEIGHT;
            this.canvasContext = canvas.getContext("2d");
            this.canvasContext.fillStyle = this.getColor(color);
            for (let y = 0; y < HEIGHT; y++) {
              for (let x = 0; x < WIDTH; x++) {
                let delta = x + y * HEIGHT;
                if (this.sample_image[delta] !== color) {
                  color = this.sample_image[delta];
                  this.canvasContext.fillStyle = this.getColor(color);
                }
                this.canvasContext.fillRect(x, y, 1, 1);
              }
            }
          }
          const labels_u8 = Object.keys(MBTILabels).map((key) =>
            parseInt(key, 10)
          );
          const labels = Object.values(MBTILabels).map((value) =>
            value.toString()
          );
          this.sample_index = data.index;
          this.sample_label = labels[data.label];
          this.classification = labels[labels_u8.indexOf(data.classification)];
          console.log(this.classification);
          // Prevent training before the first image loads.
          this.trainButtonDisabled = false;
        }
        if (data.trainedEpoch) {
          console.log("trainedEpoch");
          console.log(`Epoch: ${this.epochs}, Loss: ${data.loss}`);
          if (this.epochs < MAX_EPOCHS && !this.loadWeightsFromJson) {
            this.worker.postMessage({ trainEpoch: true });
            this.epochs += 1;
          } else if (this.loadWeightsFromJson) {
            this.epochs = 0;
            this.unBlockImageViewerUI();
          } else {
            this.epochs = 0;
            this.unBlockImageViewerUI();
          }
        }
        if (data.progress) {
          this.trainingPercentString = `${
            Math.round(data.percent * 1000) / 10
          }%`;
          this.trainingPercent = data.percent;
        }
        if (data.batchLoss) {
          let li = document.createElement("li");
          li.style.left = `${this.pointsPlotted.length * 5}px`;
          li.style.bottom = `${data.percent * 300}px`;
          this.pointsPlotted.push(li);
        }
        if (data.accuracy) {
          console.log("accuracy");
          this.trainingAccuracy = `Accuracy on Training Data: ${Math.floor(
            data.trainingAccuracy * 100
          )}%`;
          this.testingAccuracy = `Accuracy on Testing Data: ${Math.floor(
            data.testingAccuracy * 100
          )}%`;
        }
      };
      console.log(this.worker);
      this.worker.postMessage({ loadWorker: true });
    },
  },
  mounted() {
    console.log("Mounted");
    this.initWorker();
    // assert(Object.keys(MBTILabels).length === 16);
    // assert(MBTILabels["ENFP"] === (MBType.E ^ MBType.N ^ MBType.F ^ MBType.P));
    // assert(MBTILabels["ENFJ"] === (MBType.E ^ MBType.N ^ MBType.F ^ MBType.J));
    // assert(MBTILabels["ENTP"] === (MBType.E ^ MBType.N ^ MBType.T ^ MBType.P));
    // assert(MBTILabels["ENTJ"] === (MBType.E ^ MBType.N ^ MBType.T ^ MBType.J));
    // assert(MBTILabels["ESFP"] === (MBType.E ^ MBType.S ^ MBType.F ^ MBType.P));
    // assert(MBTILabels["ESFJ"] === (MBType.E ^ MBType.S ^ MBType.F ^ MBType.J));
    // assert(MBTILabels["ESTP"] === (MBType.E ^ MBType.S ^ MBType.T ^ MBType.P));
    // assert(MBTILabels["ESTJ"] === (MBType.E ^ MBType.S ^ MBType.T ^ MBType.J));
    // assert(MBTILabels["INFP"] === (MBType.I ^ MBType.N ^ MBType.F ^ MBType.P));
    // assert(MBTILabels["INFJ"] === (MBType.I ^ MBType.N ^ MBType.F ^ MBType.J));
    // assert(MBTILabels["INTP"] === (MBType.I ^ MBType.N ^ MBType.T ^ MBType.P));
    // assert(MBTILabels["INTJ"] === (MBType.I ^ MBType.N ^ MBType.T ^ MBType.J));
    // assert(MBTILabels["ISFP"] === (MBType.I ^ MBType.S ^ MBType.F ^ MBType.P));
    // assert(MBTILabels["ISFJ"] === (MBType.I ^ MBType.S ^ MBType.F ^ MBType.J));
    // assert(MBTILabels["ISTP"] === (MBType.I ^ MBType.S ^ MBType.T ^ MBType.P));
    // assert(MBTILabels["ISTJ"] === (MBType.I ^ MBType.S ^ MBType.T ^ MBType.J));
  },
})
export default class TrainingGroundsView extends Vue {
  worker: Worker | null = null;
  epochs = 0;
  prepareButtonDisabled = true;
  trainButtonDisabled = true;
  drawModeDisabled = true;
  nextButtonDisabled = true;
  previousButtonDisabled = true;
  sample_image: number[] = [];
  sample_label = "None";
  sample_index = 0;
  classification = "None";
  drawNegative = false;
  trainingAccuracy = "";
  testingAccuracy = "";
  trainingPercent = 0;
  trainingPercentString = "0%";
  pointsPlotted: HTMLLIElement[] = [];
  loadWeightsFromJson = false;
  loadWeightsDisabled = false;
  init!: () => void;
  drawCurrentImage!: () => void;
  drawPreviousImage!: () => void;
  drawNextImage!: () => void;
  drawNegativeImage!: () => void;
  getColor!: (color: number) => string;
  blockImageViewerUI!: () => void;
  unBlockImageViewerUI!: () => void;
  resetCanvas!: () => void;
  initWorker!: () => void;
  loadWeights!: () => void;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
#imageViewer {
  margin: auto auto;
}
</style>
