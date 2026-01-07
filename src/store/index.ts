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

  return {
    inited,
    filePath,
    defaceConfig,
    outputPath,
  };
})