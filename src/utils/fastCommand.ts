import {Command} from "@tauri-apps/api/shell";

function fastCommand(commandContext: string, cwd?: string, withWindow?: boolean): Command {
  if (withWindow) {
    const command = new Command('cmd', ['/c', 'start', 'cmd', '/k', commandContext], {cwd: cwd, encoding: 'gbk'});
    return command
  } else {
    const command = new Command('cmd', ['/c', commandContext], {cwd: cwd, encoding: 'gbk'});
    return command
  }
}

export default fastCommand