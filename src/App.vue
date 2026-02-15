<template>
  <div class="title_bar" data-tauri-drag-region></div>
  <InitView v-if="store().filePath.length==0" />
  <ConfigView v-else />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { envCheck } from './utils/env_check';
import InitView from './views/InitView.vue';
import ConfigView from './views/ConfigView.vue';
import store from './store';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useTheme } from 'vuetify';
const theme = useTheme()

onMounted(async ()=>{
  envCheck();
  const appWindow = getCurrentWindow()
  const systemTheme = await appWindow.theme();
  theme.change(systemTheme || 'light')
  await appWindow.listen('tauri://theme-changed', (event) => {
    theme.change(event.payload as string)
  })
  getCurrentWindow().show();
})
</script>

<style scoped>
.title_bar{
  position: fixed;
  top: 0;
  width: 100vw;
  height: 35px;
}
</style>