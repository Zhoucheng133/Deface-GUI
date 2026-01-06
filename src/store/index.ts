import { defineStore } from "pinia";
import { ref } from "vue";
export default defineStore("index", ()=>{
  let inited = ref(false);
  let filePath= ref("");

  return {
    inited,
    filePath
  };
})