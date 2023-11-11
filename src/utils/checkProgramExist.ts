import fastCommand from "@/utils/fastCommand.ts";

async function checkProgramExist(programName: string): Promise<boolean> {
  if ((await fastCommand(`tasklist | findstr ${programName}`).execute()).code === 0) {
    return true
  } else {
    return false
  }
}

export default checkProgramExist