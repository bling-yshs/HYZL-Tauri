import {Command} from "@tauri-apps/api/shell";

function fastCommand(commandContext: string, cwd?: string, withWindow?: boolean): Command {
  if (withWindow) {
    return new Command('ps', ['Start-Process', 'powershell.exe', '-ArgumentList', `"-NoExit","-Command","${commandContext}"`], {
      cwd: cwd,
      encoding: 'gbk'
    })
  } else {
    return new Command('ps', ['-Command', `${commandContext}`], {cwd: cwd, encoding: 'gbk'});
  }
  
}

export default fastCommand