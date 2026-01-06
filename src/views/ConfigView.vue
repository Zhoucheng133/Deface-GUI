<template>
  <div class="page">
    <video :src="convertFileSrc(store().filePath)" width="400px" controls></video>
  </div>
</template>


<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import store from '../store';
</script>

<style scoped>

</style>