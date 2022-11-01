import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";

import "./assets/app.css";
import { Quasar } from "quasar";
import quasarUserOptions from "./quasar-user-options";

const pinia = createPinia();
const app = createApp(App);

app.use(Quasar, quasarUserOptions);
app.use(router);
app.use(pinia);

app.mount("#app");
