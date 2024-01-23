import csv
from typing import Dict, List

from Iteration import Iteration
from Seminar import Seminar, SeminarType
from Student import Student


def import_seminars(file: str) -> Dict[SeminarType, List[Seminar]]:
    with open(file, 'r') as f:
        reader = csv.reader(f, delimiter=';')
        seminars = {SeminarType.SCIENTIFIC: [], SeminarType.PRACTICAL: []}
        currID = 0
        for row in reader:
            if len(row) < 3:
                print(f'Skip invalid line: "{row}"')
                continue
            if row[2] == 'W':
                seminars[SeminarType.SCIENTIFIC].append(Seminar(row[0], currID, int(row[1]), SeminarType.SCIENTIFIC))
            elif row[2] == 'P':
                seminars[SeminarType.PRACTICAL].append(Seminar(row[0], currID, int(row[1]), SeminarType.PRACTICAL))
            else:
                raise ValueError('Unknown Seminar Type ' + row[3])
            currID += 1
        return seminars


def import_students(file: str, seminars: Dict[SeminarType, List[Seminar]]) -> List[Student]:
    with open(file, 'r') as f:
        reader = csv.reader(f, delimiter=';')
        students = []
        for i, row in enumerate(reader):
            if len(row) < 7:
                print(f'Skip invalid line: "{row}"')
                continue
            students.append(
                Student(
                    row[0],
                    i,
                    list(seminars[SeminarType.SCIENTIFIC][int(i)] for i in row[1:4]),
                    list(seminars[SeminarType.PRACTICAL][int(i)] for i in row[4:7]),
                )
            )
        return students


def print_students(students: List[Student], iteration: Iteration):
    print(f'---------|{iteration.total_points()}|---------')
    for i, student in enumerate(students):
        w_assignment = iteration.assignments.get(student).get(SeminarType.SCIENTIFIC).name if not iteration.assignments.get(student).get(SeminarType.SCIENTIFIC) is None else 'None'
        p_assignment = iteration.assignments.get(student).get(SeminarType.PRACTICAL).name if not iteration.assignments.get(student).get(SeminarType.PRACTICAL) is None else 'None'
        print(
            f'({i + 1}) {student.name}, {iteration.points_per_student.get(student)}, (W: {w_assignment} | P: {p_assignment})')
