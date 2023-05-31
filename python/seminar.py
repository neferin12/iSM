import array
import enum


# Enum for the different types of seminars
class SeminarType(enum.Enum):
    P = "P"
    W = "W"


class Seminar:
    name: str
    capacity: int
    seminarType: SeminarType
    capacityPerRun: array

    def __init__(self, name: str, capacity: int, seminarType: SeminarType, runs: int):
        self.name = name
        self.capacity = capacity
        self.seminarType = seminarType
        self.capacityPerRun = array.array('i', [capacity] * runs)

