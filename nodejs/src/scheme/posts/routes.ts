import express, { Request, Response } from "express";
import { IId, PostInput, IPostInput, Post } from "./model";
import { provider } from "./providers/dummy";

import { logger } from "../../envs/logger";

const router = express.Router();

// GET /posts
router.get("/", (_req: Request, res: Response) => {
    res.json(provider.get_all());
});

// POST /posts
router.post("/", (req: Request<{}, {}, IPostInput>, res: Response) => {
    logger.info("[posts]: post request");
    const postInput: PostInput | Error = PostInput.from(req.body);
    if (postInput instanceof Error) {
        res.status(400).json({ error: postInput.message });
        return;
    }
    const post: Post = provider.create(postInput);
    res.status(201).json(post);
});

// GET /posts/:id
router.get("/:id", (req: Request<IId>, res: Response) => {
    logger.info("[posts]: get request");
    if (typeof req.params.id !== "string" || req.params.id.trim() === "") {
        res.status(400).json({ error: `Invalid ID` });
        return;
    }
    const post = provider.get(req.params.id);

    if (!post) {
        res.status(404).json({ error: "Post not found" });
    } else {
        res.json(post);
    }
});

// PUT /posts/:id
router.put("/:id", (req: Request<IId, {}, IPostInput>, res: Response) => {
    logger.info("[posts]: put request");
    if (typeof req.params.id !== "string" || req.params.id.trim() === "") {
        res.status(400).json({ error: `Invalid ID` });
        return;
    }

    const postInput: PostInput | Error = PostInput.from(req.body);
    if (postInput instanceof Error) {
        res.status(400).json({ error: postInput.message });
        return;
    }

    const updated = provider.update(req.params.id, postInput);
    if (!updated) {
        res.status(404).json({ error: "Post not found" });
    } else {
        res.json(updated);
    }
});

// DELETE /posts/:id
router.delete("/:id", (req: Request<IId>, res: Response) => {
    logger.info("[posts]: delete request");
    if (typeof req.params.id !== "string" || req.params.id.trim() === "") {
        res.status(400).json({ error: `Invalid ID` });
        return;
    }
    if (provider.delete(req.params.id)) {
        res.status(204).send();
    } else {
        res.status(404).json({ error: "Post not found" });
    }
});

export default router;
