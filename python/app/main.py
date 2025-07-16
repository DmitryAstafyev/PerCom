from fastapi import FastAPI, HTTPException, Path
from .schemas import Post, PostInput
from .storage import storage

app = FastAPI()


@app.get("/posts", response_model=list[Post])
def list_posts():
    return storage.list_posts()


@app.post("/posts", response_model=Post, status_code=201)
def create_post(post: PostInput):
    return storage.create_post(post.author, post.date, post.content)


@app.get("/posts/{post_id}", response_model=Post)
def get_post(post_id: str = Path(..., description="ID of the post to retrieve")):
    post = storage.get_post(post_id)
    if not post:
        raise HTTPException(status_code=404, detail="Post not found")
    return post


@app.put("/posts/{post_id}", response_model=Post)
def update_post(post_id: str, post: PostInput):
    updated = storage.update_post(post_id, post.author, post.date, post.content)
    if not updated:
        raise HTTPException(status_code=404, detail="Post not found")
    return updated


@app.delete("/posts/{post_id}", status_code=204)
def delete_post(post_id: str):
    if not storage.delete_post(post_id):
        raise HTTPException(status_code=404, detail="Post not found")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "app.main:app",
        host="0.0.0.0",
        port=8080,
        access_log=False,
    )
