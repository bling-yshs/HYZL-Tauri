import {createDir, exists} from "@tauri-apps/api/fs";
import {appDataDir} from '@tauri-apps/api/path';


async function init() {
  let appData = await appDataDir();
  if (!await exists(appData)) {
    await createDir(appData, {recursive: true})
  }
}

export default init;