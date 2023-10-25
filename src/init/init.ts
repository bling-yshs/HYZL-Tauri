import {createDir, exists} from "@tauri-apps/api/fs";
import {appDir} from "@/entity/hyzlPath.ts";


async function init() {
  await createAppDir()
}

async function createAppDir(){
  let appData = await appDir()
  if (!await exists(appData)) {
    await createDir(appData, {recursive: true})
  }
}

export default init;