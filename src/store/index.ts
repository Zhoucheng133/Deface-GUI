import { path } from "@tauri-apps/api";
import { Child, Command } from "@tauri-apps/plugin-shell";
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

  let child: Child | null = null;

  async function handler(){
    running.value = !running.value;
    logs.value = [];

    if(running.value){
      const fullOutputPath=await path.join(outputPath.value, "output.mp4");
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

      console.log(args);

      const command = Command.create(defaceConfig.value.keepAudio ? "defaceWithAudio" : "defaceWithoutAudio", args);

      command.stdout.on('data', (line) => {
        logs.value.unshift(line);
        if(logs.value.length > 50){
          logs.value.pop();
        }
      });

      command.stderr.on('data', (line) => {
        logs.value.unshift(line);
        if(logs.value.length > 50){
          logs.value.pop();
        }
      });

      command.on('close', async (_) => {
        running.value = false;
      })

      child = await command.spawn();
    }else{
      await child?.kill();
      child = null;
    }
  }

  return {
    inited,
    filePath,
    defaceConfig,
    outputPath,
    running,
    handler,
    logs
  };
})