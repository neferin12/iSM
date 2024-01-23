from enum import Enum


class SeminarType(Enum):
    SCIENTIFIC = 1
    PRACTICAL = 2


class Seminar:
    name: str
    id: int
    capacity: int
    seminar_type: SeminarType

    def __init__(self, name: str, id: int, capacity: int, seminar_type: SeminarType):
        self.name = name
        self.id = id
        self.capacity = capacity
        self.seminar_type = seminar_type