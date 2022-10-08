<template>
  <section class="progress-bar">
    <label for="trainingProgress">Training Progress</label>
    <progress id="trainingProgress" :value="percent" max="1">
      {{ content }}
    </progress>
  </section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    percent: Number,
    content: String,
  },
})
export default class ProgressBar extends Vue {
  percent!: number;
  content!: string;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
section {
  margin-top: 1em;
}
label {
  margin: 1em;
}
</style>
