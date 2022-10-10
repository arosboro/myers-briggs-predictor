let NetworkWeights = "";

const setNetworkWeights = (weights: string) => {
  NetworkWeights = weights;
};

const getNetworkWeights = () => {
  return NetworkWeights;
};

const fetchNetworkWeights = async () =>
  await fetch("../data/network.json")
    .then((response) => response.json())
    .then((raw) => {
      setNetworkWeights(JSON.stringify(raw));
      console.log("Network weights loaded");
    });

const clearWeights = () => {
  setNetworkWeights("");
  console.log("Network weights cleared");
};

export {
  fetchNetworkWeights,
  setNetworkWeights,
  getNetworkWeights,
  clearWeights,
};
