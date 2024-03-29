import "quasar/dist/quasar.css";
import "@quasar/extras/material-icons/material-icons.css";
import "@quasar/extras/material-icons-outlined/material-icons-outlined.css";
import { Dialog, Notify } from "quasar";

// To be used on app.use(Quasar, { ... })
export default {
  config: {},
  plugins: [Dialog, Notify],
};
