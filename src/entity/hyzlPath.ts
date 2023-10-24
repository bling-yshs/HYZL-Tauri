import {appDataDir, join} from "@tauri-apps/api/path";

let cachedHyzlDir: string | null = null;

export const appDir = async (): Promise<string> => cachedHyzlDir ? cachedHyzlDir : cachedHyzlDir = await appDataDir();

let cachedYunzaiDir: string | null = null;
export const yunzaiDir = async (): Promise<string> => cachedYunzaiDir ? cachedYunzaiDir : cachedYunzaiDir = await join(await appDir(), "Miao-Yunzai");
