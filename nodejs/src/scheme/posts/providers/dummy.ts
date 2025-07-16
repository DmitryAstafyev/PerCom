import { PostsProvider } from "../provider";
import { Post, PostInput } from "../model";
import { v4 as uuidv4 } from "uuid";

export class DummyProvider extends PostsProvider {
    protected store: Map<string, Post> = new Map();
    public get_all(): Post[] {
        return Array.from(this.store.values());
    }
    public get(id: string): Post | undefined {
        return this.store.get(id);
    }
    public create(input: PostInput): Post {
        const id = uuidv4();
        const post: Post = { id, ...input };
        this.store.set(id, post);
        return post;
    }
    public update(id: string, input: PostInput): Post | undefined {
        const post = this.store.get(id);
        if (!post) {
            return undefined;
        }
        post.author = input.author;
        post.content = input.content;
        post.date = input.date;
        return post;
    }
    public delete(id: string): boolean {
        return this.store.delete(id);
    }
}

export const provider = new DummyProvider();
