from typing import Dict, List, Optional
from uuid import uuid4
from .models import PostModel


class PostStorage:
    def __init__(self):
        self._posts: Dict[str, PostModel] = {}

    def list_posts(self) -> List[PostModel]:
        return list(self._posts.values())

    def get_post(self, post_id: str) -> Optional[PostModel]:
        return self._posts.get(post_id)

    def create_post(self, author: str, date, content: str) -> PostModel:
        new_id = str(uuid4())
        post = PostModel(id=new_id, author=author, date=date, content=content)
        self._posts[new_id] = post
        return post

    def update_post(self, post_id: str, author: str, date, content: str) -> Optional[PostModel]:
        if post_id not in self._posts:
            return None
        post = PostModel(id=post_id, author=author, date=date, content=content)
        self._posts[post_id] = post
        return post

    def delete_post(self, post_id: str) -> bool:
        return self._posts.pop(post_id, None) is not None


storage = PostStorage()
