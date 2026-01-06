<template>
  <InitView v-if="store().filePath.length==0" />
  <ConfigView v-else />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { envCheck } from './utils/env_check';
import InitView from './views/InitView.vue';
import ConfigView from './views/ConfigView.vue';
import store from './store';

onMounted(()=>{
  envCheck();
})
</script>