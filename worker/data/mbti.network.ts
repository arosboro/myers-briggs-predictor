let NetworkWeights = "";

const fetchWeights = async () =>
  await fetch("../data/network.json")
    .then((response) => response.json())
    .then((raw) => {
      NetworkWeights = JSON.stringify(raw);
      console.log("Network weights loaded");
    });

export { NetworkWeights, fetchWeights };
