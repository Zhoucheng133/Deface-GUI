<template>
  <div class="page">
    <v-btn color="primary" icon="mdi-plus" @click="pickFile"></v-btn>
    <div class="tip">添加一个视频文件，你也可以将文件拖拽到这里</div>
  </div>
</template>

<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-dialog';
async function pickFile(){
  const file = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: 'Video',
        extensions: ['mp4', 'mkv', 'avi']
      }
    ]
  });

  if(file){
    // TODO: 添加文件
  }
}

</script>

<style scoped>
.tip{
  margin-top: 20px;
  font-size: 14px;
  user-select: none;
  -webkit-user-select: none;
}
.page {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>