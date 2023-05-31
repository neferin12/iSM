import array


class Student:
    name: str
    wVotes: array
    pVotes: array
    pointsPerRun: array

    def __init__(self, name: str, wVotes: array, pVotes: array, runs: int):
        self.name = name
        self.wVotes = wVotes
        self.pVotes = pVotes
        self.pointsPerRun = array.array('i', [0] * runs)
