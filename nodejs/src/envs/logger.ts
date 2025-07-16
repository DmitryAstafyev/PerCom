import pino from "pino";
import fs from "fs";
import path from "path";

import { getLogs } from "./paths";

const logger = (function init() {
    const logsPath = getLogs();
    if (!fs.existsSync(logsPath)) {
        fs.mkdirSync(logsPath, { recursive: true });
    }
    const streams = [
        // { stream: process.stdout },
        {
            stream: fs.createWriteStream(
                path.join(logsPath, `${Date.now()}.nodejs.log`),
                { flags: "a" }
            ),
        },
    ];
    return pino(
        {
            level: "info",
        },
        pino.multistream(streams)
    );
})();

process.on("uncaughtException", (err) => {
    logger.error("Uncaught Exception:", err);
});

process.on("unhandledRejection", (reason) => {
    logger.error("Unhandled Rejection:", reason);
});

export { logger };
