import { message } from '@tauri-apps/plugin-dialog';
import { Command } from '@tauri-apps/plugin-shell';
import store from '../store';

async function defaceCheck(): Promise<boolean> {
  try {
    const result = await Command.create('deface', ['--version']).execute();
    return result.code === 0;
  } catch (_) {}
  return false;
}

export async function envCheck(){

  let prefsDeface=localStorage.getItem('deface');
  if(prefsDeface==null){
    if(!await defaceCheck()){
      await message('没有找到Deface', { title: '初始化失败', kind: 'error' });
    }
  }

  store().inited=true;
  return;
}