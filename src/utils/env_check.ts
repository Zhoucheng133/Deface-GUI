import { message } from '@tauri-apps/plugin-dialog';
import { Command } from '@tauri-apps/plugin-shell';

async function ffmpegCheck(): Promise<boolean> {
  try {
    const ffmpeg = await Command.create('ffmpeg', ['-version']).execute();
    return ffmpeg.code === 0;
  } catch (_) {}
  return false;
};

async function pythonCheck(): Promise<boolean> {
  const commands = ['python3', 'python'];

  for (const cmd of commands) {
    try {
      const result = await Command.create(cmd, ['-V']).execute();
      if (result.code === 0) {
        return true;
      }
    } catch (_) {}
  }
  return false;
}

async function defaceCheck(): Promise<boolean> {
  try {
    const result = await Command.create('deface', ['--version']).execute();
    return result.code === 0;
  } catch (_) {}
  return false;
}

export async function envCheck(){

  let prefsFFmpeg=localStorage.getItem('ffmpeg');
  if(prefsFFmpeg==null){
    if(!await ffmpegCheck()){
      await message('没有找到FFmpeg', { title: '初始化失败', kind: 'error' });
    }
  }

  let prefsPython=localStorage.getItem('python');
  if(prefsPython==null){
    if(!await pythonCheck()){
      await message('没有找到Python', { title: '初始化失败', kind: 'error' });
    }
  }

  let prefsDeface=localStorage.getItem('deface');
  if(prefsDeface==null){
    if(!await defaceCheck()){
      await message('没有找到Deface', { title: '初始化失败', kind: 'error' });
    }
  }
}