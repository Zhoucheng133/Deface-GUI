<template>
  <div class="page">
    <div class="config" :style="{paddingTop: os=='macos' ? '50px': '20px'}">
      <div class="panel shadow-lg">
        <div class="title">
          <div class="line-clamp-2">{{ fileName }}</div>
          <v-btn class="close_btn" density="comfortable" icon="mdi-close" @click="closeFile"></v-btn>
        </div>
        <div class="item">
          <div class="key">路径</div>
          <v-tooltip :text="filePath" location="top">
            <template v-slot:activator="{ props }">
              <div v-bind="props" class="value line-clamp-3">{{ filePath }}</div>
            </template>
          </v-tooltip>
        </div>
        <div class="item" style="grid-template-columns: 95px auto;">
          <div class="key">Thresh</div>
          <div class="value slider">
            <v-tooltip text="识别的阈值，越小越准确同时更有可能会误判" location="top">
              <template v-slot:activator="{ props }">
                <v-slider v-bind="props" v-model="defaceConfig.thresh" :max="1" :min="0" :step="0.1" density="compact" style="margin-bottom: 0;" :hide-details="true" color="primary" :disabled="running"></v-slider>
              </template>
            </v-tooltip>
            <div class="thresh_value text-right">{{ defaceConfig.thresh }}</div>
          </div>
        </div>
        <div class="item">
          <div class="key">保留音频</div>
          <div class="value">
            <v-switch v-model="defaceConfig.keepAudio" color="primary" :hide-details="true" :disabled="running"></v-switch>
          </div>
        </div>
        <div class="item">
          <div class="key">遮罩</div>
          <div class="value">
            <v-select v-model="selectedReplace" :items="replaceItems" item-title="text" item-value="value" density="compact" :hide-details="true" @update:modelValue="replaceChanged" :disabled="running"></v-select>
          </div>
        </div>
        <div class="item" style="grid-template-columns: 95px auto;">
          <div class="key">遮罩缩放</div>
          <div class="value slider">
            <v-slider v-model="defaceConfig.maskScale" :max="2" :min="0.1" :step="0.1" density="compact" style="margin-bottom: 0;" :hide-details="true" color="primary" :disabled="running"></v-slider>
            <div class="scale_value text-right">{{ defaceConfig.maskScale }}</div>
          </div>
        </div>
        <div class="item" style="margin-bottom: 20px;">
          <div class="key">重命名</div>
          <div class="value flex align-center">
            <v-text-field v-model="name" :hide-details="true" density="compact"></v-text-field>
            <div style="margin-left: 5px;">.mp4</div>
          </div>
        </div>
        <div class="output">
          <div style="font-weight: bold; margin-right: 10px;">输出</div>
          <v-tooltip :text="outputPath" location="top">
            <template v-slot:activator="{ props }">
              <v-text-field v-bind="props" density="compact" readonly v-model="outputPath" :hide-details="true"></v-text-field>
            </template>
          </v-tooltip>
          <v-btn @click="pickOutput" :disabled="running">选择</v-btn>
        </div>
      </div>
    </div>
    <div class="log" :style="{paddingTop: os=='macos' ? '50px': '20px'}">
      <div class="log_content" v-if="logs.length!=0">
        <div class="log_item" v-for="item in logs">{{ item }}</div>
      </div>
      <div class="loading" v-else-if="running">
        <v-progress-circular indeterminate></v-progress-circular>
      </div>
    </div>
  </div>
  <v-fab class="float_icon" :icon="running ? 'mdi-stop' : 'mdi-play'" @click="useStore().handler"></v-fab>
</template>


<script lang="ts" setup>
import useStore, { ReplaceWith } from '../store';
import { storeToRefs } from 'pinia'
import { path } from '@tauri-apps/api';
import { open } from '@tauri-apps/plugin-dialog';
import { onMounted, onUnmounted, ref, shallowRef } from 'vue';
import { platform } from '@tauri-apps/plugin-os';
import { listen } from '@tauri-apps/api/event';
const os = platform();

const { defaceConfig, filePath, outputPath, running, logs, name } = storeToRefs(useStore())

const fileName=ref("");

const selectedReplace = shallowRef({ text: "模糊", value: ReplaceWith.blur })
const replaceItems=[
  { text: "无", value: ReplaceWith.none },
  { text: "马赛克", value: ReplaceWith.mosaic },
  { text: "模糊", value: ReplaceWith.blur },
  { text: "色块", value: ReplaceWith.solid }
]

function closeFile(){
  filePath.value="";
  logs.value=[];
}

async function pickOutput(){
  const file = await open({
    directory: true,
  });
  if(file){
    outputPath.value=file;
  }
  localStorage.setItem("outputPath", outputPath.value);
}

let unlistenLog: any;
let unlistenEnd: any;

onMounted(async ()=>{
  fileName.value=await path.basename(filePath.value);
  outputPath.value=localStorage.getItem("outputPath")|| "";

  unlistenLog=await listen<string>("log", (event)=>{
    if(event.payload.includes("resource_tracker")){
      return;
    }
    logs.value.unshift(event.payload);
    if(logs.value.length>50){
      logs.value.pop();
    }
  })

  unlistenEnd=await listen<string>("end", (event)=>{
    logs.value=[];
    logs.value.unshift(event.payload);
    running.value=false;
  })
})

onUnmounted(()=>{
  unlistenLog?.();
  unlistenEnd?.();
})

function replaceChanged(value: any) {
  defaceConfig.value.replaceWith=value;
}

</script>

<style scoped>
.loading{
  height: 100%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.title_text{
  width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
}
.close_btn{
  margin-left: auto;
}
.log_item{
  width: 100%;
  overflow: hidden;
}
.log_content{
  height: 100%;
  width: 100%;
  overflow-y: auto;
}
.log{
  padding-left: 20px;
  padding-right: 20px;
  padding-bottom: 20px;
  display: flex;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
}

.float_icon{
  position: fixed;
  right: 20px;
  bottom: 40px;
}
.output{
  width: 100%;
  display: flex;
  margin-top: auto;
  align-items: center;
}
.slider{
  display: grid;
  grid-template-columns: auto 20px;
  align-items: center;
}
.key{
  font-weight: bold;
}
.value{
  width: 100%;
}
.item{
  display: grid;
  grid-template-columns: 100px auto;
  margin-top: 15px;
  align-items: center;
}
.title{
  font-size: 20px;
  text-overflow: ellipsis;
  font-weight: bold;
  width: 100%;
  display: flex;
  align-items: center;
}
.panel{
  width: 100%;
  height: 100%;
  border-radius: 20px;
  background-color: white;
  padding: 20px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.config{
  width: 100%;
  height: 100%;
  display: flex;
  padding-bottom: 20px;
  padding-left: 20px;
  min-width: 0;
}
.page{
  display: grid;
  justify-content: center;
  height: 100vh;
  align-items: center;
  width: 100%;
  grid-template-columns: 1fr 400px;
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}
</style>