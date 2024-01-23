from typing import Dict

from Seminar import Seminar, SeminarType
from Student import Student


class Iteration:
    points: int | None
    assignments: Dict[Student, Dict[SeminarType, Seminar]]
    seminarCapacities: Dict[Seminar, int]
    points_per_student: Dict[Student, int]

    def __init__(self):
        self.points = None
        self.assignments = {}
        self.seminarCapacities = {}
        self.points_per_student = {}

    def total_points(self) -> int:
        if self.points is None:
            self.points = sum(self.points_per_student.values())
        return self.points
