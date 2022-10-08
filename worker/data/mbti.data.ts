import { MBTIData, MBTypeSet, Raw, Output, Sample } from "./mbti.types";

const MBTI: MBTIData = [];

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
  "ENFJ" = 0b01010110, // "ENFJ",
  "ENFP" = 0b01010101, // "ENFP",
  "ENTJ" = 0b01011010, // "ENTJ",
  "ENTP" = 0b01011001, // "ENTP",
  "ESFJ" = 0b01100110, // "ESFJ",
  "ESFP" = 0b01100101, // "ESFP",
  "ESTJ" = 0b01101010, // "ESTJ",
  "ESTP" = 0b01101001, // "ESTP",
  "INFJ" = 0b10010110, // "INFJ",
  "INFP" = 0b10010101, // "INFP",
  "INTJ" = 0b10011010, // "INTJ",
  "INTP" = 0b10011001, // "INTP",
  "ISFJ" = 0b10100110, // "ISFJ",
  "ISFP" = 0b10100101, // "ISFP",
  "ISTJ" = 0b10101010, // "ISTJ",
  "ISTP" = 0b10101001, // "ISTP",
}

const labels = Object.values(MBTILabels).slice(0, 16);
const labels_u8 = Object.keys(MBTILabels)
  .slice(0, 16)
  .map((label) => parseInt(label, 10));

fetch("../data/mbti_samples.json")
  .then((response) => response.json())
  .then((raw) => {
    const WIDTH = 16;
    const HEIGHT = 16;

    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].forEach(function (
      id
    ) {
      // myers briggs type indicator
      const label = labels[id];
      const mbti: MBTypeSet = {
        id: id,
        epoch: 0,
        raw: raw[label], // raw data
        length: raw[label].length | 0, // number of samples
        // get one sample
        get: function (_which: number) {
          let which: number = _which;
          // if not specified, or if invalid, pick a random sample
          if ("undefined" == typeof which || which > this.length || which < 0) {
            which = (Math.random() * mbti.length) | 0;
          }

          // generate sample
          const sample = this.raw[which];
          return sample;
        },
        // get a range of samples
        range: function (start: number, end: number) {
          if (start < 0) start = 0;
          if (end >= this.length) end = this.length - 1;
          if (start > end) {
            const tmp = start;
            start = end;
            end = tmp;
          }
          const range: Raw[] = [];
          for (let i = start; i <= end; range.push(this.get(i++)));
          return range;
        },
        // get set of indicators, ready to be used for training or testing
        set: function (start: number, end: number) {
          const set: Sample[] = [];
          const output: Output = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          ];
          output[id] = 1;
          const range: Raw[] = this.range(start, end);
          for (
            let i = 0;
            i < range.length;
            set.push({
              input: range[i++],
              output: output,
            })
          );
          return set;
        },
      };

      // add Myers Briggs Type Indicator
      MBTI.push(mbti);
      console.log(
        `MBTI<${id}, usize> | MBTI<${labels_u8[id]}, u8> | MBTI<${labels[id]}, &str>: ${mbti.length} Samples`
      );
    });

    // Generates non-overlaping training and a test sets, with the desired ammount of samples
    MBTI.get = function (count: number) {
      let range: Sample[] = [];
      for (const i in [
        "ENFJ",
        "ENFP",
        "ENTJ",
        "ENTP",
        "ESFJ",
        "ESFP",
        "ESTJ",
        "ESTP",
        "INFJ",
        "INFP",
        "INTJ",
        "INTP",
        "ISFJ",
        "ISFP",
        "ISTJ",
        "ISTP",
      ]) {
        range = range.concat(this[i].set(0, this[i].length));
      }
      range = shuffle(range);
      if (Number(count)) {
        range = range.slice(0, Number(count));
      }
      return range;
    };

    // Generates non-overlaping training and a test sets, with the desired ammount of samples
    MBTI.set = function (_training: number, _test: number) {
      MBTI.__MINLENGTH = 1000;

      let training = (_training / 16) | 0;
      let test = (_test / 16) | 0;

      if (training < 1) training = 1;
      if (test < 1) test = 1;

      // check that there are enough samples to make the sets, and change the ammounts if they are too big
      if (training + test + 1 > MBTI.__MINLENGTH) {
        console.warn(
          "There are not enough samples to make a training set of " +
            training +
            " elements and a test set of " +
            test +
            " elements."
        );
        if (training > test) {
          test = MBTI.__MINLENGTH * (test / training);
          training = MBTI.__MINLENGTH - training;
        } else {
          training = MBTI.__MINLENGTH * (training / test);
          test = MBTI.__MINLENGTH - test;
        }
      }

      // make both sets
      let trainingSet: Sample[] = [];
      let testSet: Sample[] = [];

      for (let i = 0; i < 16; i++) {
        trainingSet = trainingSet.concat(MBTI[i].set(0, training - 1));
        testSet = testSet.concat(MBTI[i].set(training, training + test - 1));
      }

      // return the sets, shuffled
      return {
        training: shuffle(trainingSet),
        test: shuffle(testSet),
      };
    };

    // draws a given MBTI in a canvas context
    MBTI.draw = function (
      mbti: Raw,
      context: CanvasRenderingContext2D,
      offsetX: number,
      offsetY: number
    ) {
      const imageData = context.getImageData(
        offsetX || 0,
        offsetY || 0,
        WIDTH,
        HEIGHT
      );
      for (let i = 0; i < mbti.length; i++) {
        imageData.data[i * 4] = mbti[i] * 255;
        imageData.data[i * 4 + 1] = mbti[i] * 255;
        imageData.data[i * 4 + 2] = mbti[i] * 255;
        imageData.data[i * 4 + 3] = 255;
      }
      context.putImageData(imageData, offsetX || 0, offsetY || 0);
    };

    // takes an array of 16 Myers Briggs Type Indicators representing a number from 0 to 15 (ie. any output in a dataset) and returns the actual Type Indicator
    MBTI.toNumber = function (array: Output) {
      return array.indexOf(Math.max(...array));
    };

    //+ Jonas Raoni Soares Silva
    //@ http://jsfromhell.com/array/shuffle [rev. #1]
    function shuffle(v: Sample[]) {
      for (
        let j: number, x: Sample, i = v.length;
        i;
        j = parseInt(`${Math.random() * i}`), x = v[--i], v[i] = v[j], v[j] = x
      );
      return v;
    }
  });

export default MBTI;
