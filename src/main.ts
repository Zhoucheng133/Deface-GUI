import { createApp } from "vue";
import App from "./App.vue";
import '@mdi/font/css/materialdesignicons.css';

import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

import './style.css';

import { createPinia } from 'pinia';

const vuetify = createVuetify({
  components,
  directives,
})

const pinia = createPinia()

createApp(App)
.use(vuetify)
.use(pinia)
.mount("#app");
