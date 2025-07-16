import express from "express";
import bodyParser from "body-parser";
import postsRouter from "./scheme/posts/routes";

import { logger } from "./envs/logger";

const app = express();
const PORT = 8080;

app.use(bodyParser.json());
app.use("/posts", postsRouter);
app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server is running at http://localhost:${PORT}`);
}).on("error", (err: NodeJS.ErrnoException) => {
    if (err.code === "EADDRINUSE") {
        logger.error(`Port ${PORT} is already in use.`);
    } else {
        logger.error("Server failed to start:", err);
    }
    process.exit(1);
});
