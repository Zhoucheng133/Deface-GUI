<template>
  <InitView v-if="store().filePath.length==0" />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { envCheck } from './utils/env_check';
import InitView from './views/init_view.vue';
import store from './store';

onMounted(()=>{
  envCheck();
})
</script>