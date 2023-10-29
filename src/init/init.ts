import {createDir, exists} from "@tauri-apps/api/fs";
import {getAppDir, getAnnouncementDir} from "@/entity/hyzlPath.ts";

async function init() {
  await createAppDir()
  await createAnnouncementDir()
}

async function createAnnouncementDir() {
  if (!await exists(await getAnnouncementDir())) {
    await createDir(await getAnnouncementDir(), {recursive: true})
  }
}

async function createAppDir() {
  if (!await exists(await getAppDir())) {
    await createDir(await getAppDir(), {recursive: true})
  }
}

export default init;