<template>
  <div class="controls">
    <label for="loadWeights">Use pre-trained weights</label>
    <input
      type="checkbox"
      :checked="loadWeightsFromJson"
      :disabled="isLoadWeightsDisabled"
      @click="loadWeights"
      id="loadWeights"
    />
    <br />
    <span>
      <button id="prepare" :disabled="isPrepareDisabled" @click="prepareData()">
        Prepare Data
      </button>
      <button id="train" :disabled="isTrainDisabled" @click="trainEpoch()">
        Train Epoch
      </button>
    </span>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  name: "TrainingActions",
  emits: ["train-epoch", "load-weights"],
  props: {
    worker: Worker,
    prepareButtonDisabled: Boolean,
    trainButtonDisabled: Boolean,
    loadWeightsFromJson: Boolean,
    loadWeightsDisabled: Boolean,
  },
  computed: {
    isPrepareDisabled(): boolean {
      return this.prepareButtonDisabled;
    },
    isTrainDisabled(): boolean {
      return this.trainButtonDisabled;
    },
    isLoadWeightsDisabled(): boolean {
      return this.loadWeightsDisabled;
    },
  },
  methods: {
    prepareData() {
      console.log("Preparing data");
      this.worker.postMessage({ prepareDataset: true });
    },
    trainEpoch() {
      console.log("Training epoch");
      this.$emit("train-epoch");
      this.worker.postMessage({ trainEpoch: true });
    },
    loadWeights() {
      this.$emit("load-weights");
    },
  },
})
export default class TrainingActions extends Vue {
  worker!: Worker;
  prepareButtonDisabled!: boolean;
  trainButtonDisabled!: boolean;
  loadWeightsDisabled!: boolean;
  loadWeightsFromJson!: boolean;
  prepareData!: () => void;
  trainEpoch!: () => void;
  loadWeights!: () => void;
  isPrepareDisabled!: boolean;
  isTrainDisabled!: boolean;
  isLoadWeightsDisabled!: boolean;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
button {
  margin: 1em 0;
}
#prepare {
  margin-right: 1em;
}
</style>
