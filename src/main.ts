declare global {
  interface Window {
    logProgress: (progress: number) => void;
    logBatchLoss: (loss: number) => void;
    getNetworkWeights: () => string;
  }
}

import { createApp } from "vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import store from "./store";

createApp(App).use(store).use(router).mount("#app");
