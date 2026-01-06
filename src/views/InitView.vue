<template>
  <div class="page">
    <v-btn color="primary" icon="mdi-plus" @click="pickFile"></v-btn>
    <div class="tip">添加一个视频文件，你也可以将文件拖拽到这里</div>
  </div>
</template>

<script lang="ts" setup>
import { listen } from '@tauri-apps/api/event';
import { message, open } from '@tauri-apps/plugin-dialog';
import { onBeforeUnmount, onMounted } from 'vue';
import { path } from '@tauri-apps/api';
import store from '../store';

let unlisten: any;

async function pickFile() {
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
    store().filePath = file;
  }
}

async function dropHandler(targets: any) {
  const filePath=targets[0];
  const extension=await path.extname(filePath);
  if(extension==='.mp4'||extension==='.mkv'||extension==='.avi'){
    store().filePath = filePath;
  }else{
    await message('不支持的文件', { title: '无法处理', kind: 'error' });
  }

}

onMounted(async ()=>{
  unlisten = await listen('tauri://drag-drop', async (event: any) => {
    const payload = event?.payload;
    if (
      payload &&
      typeof payload === 'object' &&
      Array.isArray(payload.paths) &&
      typeof payload.paths[0] === 'string'
    ) {
      const targets = payload.paths;
      dropHandler(targets);
    }
  });
})

onBeforeUnmount(() => {
  if (unlisten) unlisten();
});


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