from typing import List

from Seminar import Seminar


class Student:
    name: str
    id: int
    w_wishes: List[Seminar]
    p_wishes: List[Seminar]

    def __init__(self, name: str, id: int, w_wishes: List[Seminar], p_wishes: List[Seminar]):
        self.name = name
        self.id = id
        self.w_wishes = w_wishes
        self.p_wishes = p_wishes
