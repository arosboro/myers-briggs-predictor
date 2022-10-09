<template>
  <section class="scrollable-chart-container">
    <h1>Average loss per 100 images during training</h1>
    <h2 id="verticalAxisLabel">1</h2>
    <h2 id="origin">0</h2>
    <h2 id="horizontalAxisLabel">Batch</h2>
    <figure class="chart">
      <ul>
        <li
          v-for="(el, index) in pointsPlotted"
          :style="{ left: el.style.left, bottom: el.style.bottom }"
          :key="index"
        >
          &nbsp;
        </li>
      </ul>
    </figure>
  </section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    pointsPlotted: Array,
  },
})
export default class ProgressBar extends Vue {
  pointsPlotted!: HTMLLIElement[];
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="less">
label {
  margin: 1em;
}
figure.chart {
  border-left: 1px solid black;
  border-bottom: 1px solid black;
  height: 300px;
  width: auto;
  margin: 1em;
  padding: 0;
  overflow-x: scroll;
}
figure.chart ul {
  list-style: none;
  margin-block-end: 0;
  margin-block-start: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  position: relative;
}
figure.chart ul li {
  /* Create a circle via the border */
  border: 2px solid blue;
  border-radius: 50%;
  /* The chart is 400px wide, and at most we need to plot 80 points */
  height: 5px;
  width: 5px;
  position: absolute;
}
#verticalAxisLabel {
  position: absolute;
  left: 5px;
  margin: 0;
}
#origin {
  margin: 0;
  position: absolute;
  margin-top: 267px;
  left: 5px;
}
#horizontalAxisLabel {
  position: absolute;
  margin-top: 302px;
  margin-left: 20px;
}
.scrollable-chart-container {
  padding-bottom: 1em;
}
</style>
