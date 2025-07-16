import { Post, PostInput } from "./model";

export abstract class PostsProvider {
    abstract get_all(): Post[];
    abstract get(id: string): Post | undefined;
    abstract create(input: PostInput): Post;
    abstract update(id: string, input: PostInput): Post | undefined;
    abstract delete(id: string): boolean;
}
