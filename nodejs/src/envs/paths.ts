import os from "os";
import path from "path";

const APP_DIR = ".ex_server";
const LOG_DIR = "logs";

export function getHome(): string {
    return path.join(os.homedir(), APP_DIR);
}

export function getLogs(): string {
    return path.join(getHome(), LOG_DIR);
}
