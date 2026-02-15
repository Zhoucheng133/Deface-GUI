import { path } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { defineStore } from "pinia";
import { ref } from "vue";

export enum ReplaceWith{
  blur,
  solid,
  none,
  mosaic,
}

class DefaceConfig{
  constructor(
    public thresh: number = 0.7,
    public keepAudio: boolean = true,
    public maskScale: number = 1.3,
    public replaceWith: ReplaceWith = ReplaceWith.blur
  ){}
}

export default defineStore("index", ()=>{
  let inited = ref(false);
  let filePath= ref("");

  let defaceConfig = ref<DefaceConfig>(new DefaceConfig());

  let outputPath = ref("");
  let running = ref(false);

  let logs=ref<string[]>([]);
  let name=ref("output");

  async function handler(){
    if(name.value.length==0){
      await message('没有设置输出文件名', { title: '无法执行', kind: 'error' });
      return;
    }
    running.value = !running.value;
    logs.value = [];

    if(running.value){
      if(outputPath.value.length==0){
        running.value = false;
        logs.value.unshift("❌ No Output Path");
        await message('没有选择输出目录', { title: '错误', kind: 'error' });
        return;
      }

      const inValidPath = await invoke("check_path", {
        dir: outputPath.value,
        file: `${name.value}.mp4`
      });

      if(inValidPath){
        running.value = false;
        await message('文件已存在', { title: '错误', kind: 'error' });
        return;
      }

      const fullOutputPath=await path.join(outputPath.value, `${name.value}.mp4`);
      let args=[
        filePath.value,
        '--output',
        fullOutputPath,
        '--thresh',
        defaceConfig.value.thresh.toString(),
        '--replacewith',
        ReplaceWith[defaceConfig.value.replaceWith],
        '--mask-scale',
        defaceConfig.value.maskScale.toString(),
      ];

      if(defaceConfig.value.keepAudio){
        args.push('-k');
      }

      await invoke("run_task", { args: args });
    }else{
      await invoke("stop_task");
    }
  }

  return {
    inited,
    filePath,
    defaceConfig,
    outputPath,
    running,
    handler,
    logs,
    name
  };
})