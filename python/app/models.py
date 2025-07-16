from dataclasses import dataclass
from datetime import datetime


@dataclass
class PostModel:
    id: str
    author: str
    date: datetime
    content: str
