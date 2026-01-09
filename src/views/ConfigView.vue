<template>
  <div class="page">
    <div class="config">
      <div class="panel shadow-lg">
        <div class="title">
          {{ fileName }}
        </div>
        <div class="item">
          <div class="key">路径</div>
          <div class="value line-clamp-3">{{ filePath }}</div>
        </div>
        <div class="item" style="grid-template-columns: 95px auto;">
          <div class="key">Thresh</div>
          <div class="value slider">
            <v-slider v-model="defaceConfig.thresh" :max="1" :min="0" :step="0.1" density="compact" style="margin-bottom: 0;" :hide-details="true" color="primary"></v-slider>
            <div class="thresh_value text-right">{{ defaceConfig.thresh }}</div>
          </div>
        </div>
        <div class="item">
          <div class="key">保留音频</div>
          <div class="value">
            <v-switch v-model="defaceConfig.keepAudio" color="primary" :hide-details="true"></v-switch>
          </div>
        </div>
        <div class="item">
          <div class="key">遮罩</div>
          <div class="value">
            <v-select v-model="selectedReplace" :items="replaceItems" item-title="text" item-value="value" density="compact" :hide-details="true" @update:modelValue="replaceChanged"></v-select>
          </div>
        </div>
        <div class="item" style="grid-template-columns: 95px auto;" v-if="defaceConfig.replaceWith == ReplaceWith.mosaic">
          <div class="key">马赛克大小</div>
          <div class="value slider">
            <v-slider v-model="defaceConfig.maskScale" :max="2" :min="0.1" :step="0.1" density="compact" style="margin-bottom: 0;" :hide-details="true" color="primary"></v-slider>
            <div class="scale_value text-right">{{ defaceConfig.maskScale }}</div>
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
    <div class="log"></div>
  </div>
</template>


<script lang="ts" setup>
import useStore, { ReplaceWith } from '../store';
import { storeToRefs } from 'pinia'
import { path } from '@tauri-apps/api';
import { open } from '@tauri-apps/plugin-dialog';
import { onMounted, ref, shallowRef } from 'vue';

const { defaceConfig, filePath, outputPath, running } = storeToRefs(useStore())

const fileName=ref("");

const selectedReplace = shallowRef({ text: "模糊", value: ReplaceWith.blur })
const replaceItems=[
  { text: "无", value: ReplaceWith.none },
  { text: "马赛克", value: ReplaceWith.mosaic },
  { text: "模糊", value: ReplaceWith.blur },
  { text: "色块", value: ReplaceWith.solid }
]

async function pickOutput(){
  const file = await open({
    directory: true,
  });
  if(file){
    outputPath.value=file;
  }
  localStorage.setItem("outputPath", outputPath.value);
}

onMounted(async ()=>{
  fileName.value=await path.basename(filePath.value);
  outputPath.value=localStorage.getItem("outputPath")||filePath.value;
})

function replaceChanged(value: any) {
  defaceConfig.value.replaceWith=value;
}

</script>

<style scoped>
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
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: bold;
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
.video_preview{
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.config{
  width: 100%;
  height: 100%;
  display: flex;
  padding-bottom: 20px;
  padding-left: 20px;
  padding-top: 50px;
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