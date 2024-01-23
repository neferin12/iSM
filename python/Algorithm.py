import random
from random import shuffle
from typing import List

from Iteration import Iteration
from Seminar import Seminar, SeminarType
from Student import Student


def try_assignment(iteration: Iteration, student: Student, seminar: Seminar, seminar_type: SeminarType,
                   selection_points: int) -> bool:
    cap = iteration.seminarCapacities.get(seminar, seminar.capacity)

    if cap > 0:
        iteration.seminarCapacities[seminar] = cap - 1
        iteration.assignments[student][seminar_type] = seminar
        iteration.points_per_student[student] += selection_points
        return True

    return False


def run_algorithm(students: List[Student], iterations: int) -> Iteration:
    points = {
        'FIRST_SELECTION': 0,
        'SECOND_SELECTION': 5,
        'THIRD_SELECTION': 10,
        'NO_SELECTION': 30
    }

    best_iteration: Iteration | None = None

    for i in range(iterations):
        iteration = Iteration()
        iteration.points_per_student = {student: 0 for student in students}

        random.shuffle(students)

        for student in students:
            iteration.assignments[student] = {}

            if (
                    not try_assignment(iteration, student, student.w_wishes[0], SeminarType.SCIENTIFIC,
                                       points['FIRST_SELECTION']) and
                    not try_assignment(iteration, student, student.w_wishes[1], SeminarType.SCIENTIFIC,
                                       points['SECOND_SELECTION']) and
                    not try_assignment(iteration, student, student.w_wishes[2], SeminarType.SCIENTIFIC,
                                       points['THIRD_SELECTION'])
            ):
                iteration.points_per_student[student] += points['NO_SELECTION']

            if (
                    not try_assignment(iteration, student, student.p_wishes[0], SeminarType.PRACTICAL,
                                       points['FIRST_SELECTION']) and
                    not try_assignment(iteration, student, student.p_wishes[1], SeminarType.PRACTICAL,
                                       points['SECOND_SELECTION']) and
                    not try_assignment(iteration, student, student.p_wishes[2], SeminarType.PRACTICAL,
                                       points['THIRD_SELECTION'])
            ):
                iteration.points_per_student[student] += points['NO_SELECTION']

        if best_iteration is None or iteration.total_points() < best_iteration.total_points():
            best_iteration = iteration

    if best_iteration is None:
        print('No iteration found')
        exit(-1)
    return best_iteration
