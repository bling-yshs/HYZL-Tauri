import {appDataDir, join} from "@tauri-apps/api/path";

let cachedHyzlDir: string | null = null;
export const getAppDir = async (): Promise<string> => cachedHyzlDir ? cachedHyzlDir : cachedHyzlDir = await appDataDir();

let cachedYunzaiDir: string | null = null;
export const getYunzaiDir = async (): Promise<string> => cachedYunzaiDir ? cachedYunzaiDir : cachedYunzaiDir = await join(await getAppDir(), "Miao-Yunzai");

let cachedAnnouncementDir: string | null = null;
export const getAnnouncementDir = async (): Promise<string> => cachedAnnouncementDir ? cachedAnnouncementDir : cachedAnnouncementDir = await join(await getAppDir(), "hyzl-announcement");

// 公告路径
let cachedAnnouncementPath: string | null = null;
export const getAnnouncementPath = async (): Promise<string> => cachedAnnouncementPath ? cachedAnnouncementPath : cachedAnnouncementPath = await join(await getAnnouncementDir(), "announcement.json");

// 签名API路径
let cachedSignApiPath: string | null = null;
export const getSignApiDir = async (): Promise<string> => cachedSignApiPath ? cachedSignApiPath : cachedSignApiPath = await join(await getYunzaiDir(), "hyzl-sign-api");