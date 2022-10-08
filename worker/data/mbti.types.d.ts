interface MBTIData extends Array<MBTypeSet> {
  __MINLENGTH?: number;
  get?: (count: number) => Sample[];
  set?: (_training: number, _test: number) => MBTIDataset;
  draw?: (
    mbti: Raw,
    context: CanvasRenderingContext2D,
    offsetX: number,
    offsetY: number
  ) => void;
  toNumber?: (array: Output) => number;
  ninja?: () => MBTIData;
}

type MBTypeSet = {
  id: number;
  epoch: number;
  raw: Raw[];
  length: number;
  get: (_which: number) => Raw;
  range: (start: number, end: number) => Raw[];
  set: (start: number, end: number) => Sample[];
};

type Raw = number[];

type Output = [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number
];

type Sample = {
  input: Raw;
  output: Output;
};

type MBTIDataset = {
  training: Sample[];
  test: Sample[];
};

export { MBTIData, MBTypeSet, Raw, Output, Sample, MBTIDataset };
