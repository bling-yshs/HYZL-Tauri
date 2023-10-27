import {Command} from "@tauri-apps/api/shell";

async function checkProcessExist(pid: number): Promise<boolean> {
  const checkCommand = new Command('ps', ['Get-Process', '-Id', pid.toString(), '-ErrorAction', 'SilentlyContinue']);
  const childProcess = await checkCommand.execute();
  if (childProcess.code === 1) {
    return false
  }
  if (childProcess.code === 0) {
    return true
  }
  return true
}

export default checkProcessExist