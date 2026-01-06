<template>
  <v-btn color="primary">按钮</v-btn>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { envCheck } from './utils/env_check';

onMounted(()=>{
  envCheck();
})
</script>