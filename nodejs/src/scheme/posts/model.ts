export interface IId {
    id: string;
}

export interface Post {
    id: string;
    author: string;
    date: string; // ISO format
    content: string;
}

export interface IPostInput {
    author: string;
    date: string;
    content: string;
}

export class PostInput implements IPostInput {
    static from(smth: IPostInput): PostInput | Error {
        if (typeof smth !== "object" || !smth) {
            return new Error(`Expecting object, but gotten: ${typeof smth}`);
        }
        if (typeof smth.author !== "string" || smth.author.trim() === "") {
            return new Error(
                `Expecting field "author" isn't empty string, but gotten: ${smth.author}`
            );
        }
        if (typeof smth.date !== "string" || smth.date.trim() === "") {
            return new Error(
                `Expecting field "date" isn't empty string, but gotten: ${smth.date}`
            );
        }
        if (typeof smth.content !== "string") {
            return new Error(
                `Expecting field "content" is string, but gotten: ${smth.content}`
            );
        }
        return new PostInput(smth);
    }

    public readonly author: string;
    public readonly date: string;
    public readonly content: string;

    constructor(input: IPostInput) {
        this.author = input.author;
        this.date = input.date;
        this.content = input.content;
    }
}
