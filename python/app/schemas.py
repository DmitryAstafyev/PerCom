from pydantic import BaseModel
from typing import List
from datetime import datetime


class Post(BaseModel):
    id: str
    author: str
    date: datetime
    content: str


class PostInput(BaseModel):
    author: str
    date: datetime
    content: str
