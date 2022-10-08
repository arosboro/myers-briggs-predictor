<template>
  <section id="imageViewer" class="flex column">
    <div class="flex">
      <div class="center">
        <table>
          <tr>
            <th></th>
            <th></th>
            <th>E</th>
            <th></th>
            <th>S</th>
            <th></th>
          </tr>
          <tr>
            <th></th>
            <td colspan="4" rowspan="4">
              <canvas id="image"></canvas>
            </td>
            <th></th>
          </tr>
          <tr>
            <th>I</th>
            <th>N</th>
          </tr>
          <tr>
            <th></th>
            <th></th>
          </tr>
          <tr>
            <th>T</th>
            <th>P</th>
          </tr>
          <tr>
            <th></th>
            <th></th>
            <th>F</th>
            <th></th>
            <th>J</th>
            <th></th>
          </tr>
        </table>
        <div class="controls">
          <label for="viewMode">View Negative</label>
          <input
            type="checkbox"
            :checked="drawNegative"
            :disabled="isDrawModeDisabled"
            @click="viewMode"
            id="viewMode"
          />
        </div>
      </div>
    </div>
    <div class="flex column">
      <span>
        Sample #{{ sample_index }}:
        {{ sample_label !== "" ? `(${sample_label})` : `sample_label` }}
        Predicted: {{ classification }}
      </span>
      <div class="flex row space">
        <button
          aria-label="Previous Image"
          :disabled="isPreviousButtonDisabled"
          @click="previousImage"
          id="previous"
        >
          ←
        </button>
        <button
          aria-label="Next Image"
          :disabled="isNextButtonDisabled"
          @click="nextImage"
          id="next"
        >
          →
        </button>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  emits: ["draw-next-image", "draw-previous-image", "draw-negative"],
  props: {
    drawModeDisabled: Boolean,
    nextButtonDisabled: Boolean,
    previousButtonDisabled: Boolean,
    drawNegative: Boolean,
    sample_index: Number,
    sample_label: String,
    classification: String,
  },
  computed: {
    isDrawModeDisabled(): boolean {
      return this.drawModeDisabled;
    },
    isNextButtonDisabled(): boolean {
      return this.nextButtonDisabled;
    },
    isPreviousButtonDisabled(): boolean {
      return this.previousButtonDisabled;
    },
    isDrawNegative(): boolean {
      return this.drawNegative;
    },
  },
  methods: {
    previousImage: function () {
      this.$emit("draw-previous-image");
    },
    nextImage: function () {
      this.$emit("draw-next-image");
    },
    viewMode: function () {
      this.$emit("draw-negative");
    },
  },
})
export default class ImageViewer extends Vue {
  drawModeDisabled!: boolean;
  nextButtonDisabled!: boolean;
  previousButtonDisabled!: boolean;
  drawNegative!: boolean;
  sample_index!: number;
  sample_label!: string;
  classification!: string;
  nextImage!: () => void;
  previousImage!: () => void;
  viewMode!: () => void;
  isDrawModeDisabled!: () => boolean;
  isNextButtonDisabled!: () => boolean;
  isPreviousButtonDisabled!: () => boolean;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
#image {
  margin: 1em;
  transform: scale(100%);
  image-rendering: pixelated;
  width: 128px;
  z-index: 1;
}
#imageViewer {
  width: 200px;
  z-index: 0;
}
#currentImage {
  text-align: center;
}
.controls {
  margin-top: 1em;
}
.flex {
  display: flex;
}
.column {
  flex-direction: column;
}
.row {
  flex-direction: row;
}
.center {
  margin: auto;
}
.space {
  justify-content: space-around;
}
section {
  margin-top: 1em;
}
span {
  margin: 1em;
}
</style>
