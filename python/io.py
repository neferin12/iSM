from seminar import Seminar, SeminarType


def importSeminars(path: str):
    seminars = []
    with open(path, 'r') as f:
        for line in f:
            line = line.strip()
            if line.startswith('#') or line == '':
                continue
            seminar = line.split(';')
            seminars.append(Seminar(seminar[0], int(seminar[1]), SeminarType(seminar[2]), int(seminar[3])))
    return seminars
