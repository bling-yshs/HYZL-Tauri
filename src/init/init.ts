import {createDir, exists} from "@tauri-apps/api/fs";
import {getAppDir, getAnnouncementDir, getAppCacheDir} from "@/entity/hyzlPath.ts";

async function init() {
  await createAppDir()
  await createAnnouncementDir()
  await createAppCacheDir()
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

async function createAppCacheDir() {
  if (!await exists(await getAppCacheDir())) {
    await createDir(await getAppCacheDir(), {recursive: true})
  }
}

export default init;