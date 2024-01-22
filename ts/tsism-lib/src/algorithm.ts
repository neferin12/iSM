import {Student} from './student'
import {Seminar} from './seminar'
import {Iteration} from "./iteration";

export enum Points {
    FIRST_SELECTION = 0,
    SECOND_SELECTION = 5,
    THIRD_SELECTION = 10,
    NO_SELECTION = 30
}

function shuffle<T>(a: Array<T>) {
    for (let i = a.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [a[i], a[j]] = [a[j], a[i]]
    }

    return a
}

export async function runAlgorithm(iterations: number, students: Student[]): Promise<Iteration> {
    // TODO Multithreading
    return execute(iterations, students)
}

function tryAssignment(seminar: Seminar, student: Student, currentIteration: Iteration, seminarType: 'W' | 'P', points: Points) {
    let cap = currentIteration.seminarCapacity.get(seminar.id)
    if (cap === undefined) {
        cap = seminar.capacity
    }

    if (cap > 0) {
        currentIteration.seminarCapacity.set(seminar.id, cap - 1);

        const previousAssignment = currentIteration.assignments.get(student) || {}
        if (seminarType === 'W') {
            currentIteration.assignments.set(student, {...previousAssignment, wSeminar: seminar})
        }

        if (seminarType === 'P') {
            currentIteration.assignments.set(student, {...previousAssignment, pSeminar: seminar})
        }

        currentIteration.pointsPerStudent.set(student.id, (currentIteration.pointsPerStudent.get(student.id) || 0) + points)

        return true
    }

    return false
}

function execute(iterations: number, students: Student[]): Iteration {
    let best: Iteration | null = null
    for (let i = 0; i < iterations; i++) {
        const algRun = new Iteration();
        const currentIteration = new Iteration();
        const studentsCopy = shuffle([...students])
        for (const student of studentsCopy) {
            if (
                !tryAssignment(student.wWishes[0], student, currentIteration, 'W', Points.FIRST_SELECTION) &&
                !tryAssignment(student.wWishes[1], student, currentIteration, 'W', Points.SECOND_SELECTION) &&
                !tryAssignment(student.wWishes[2], student, currentIteration, 'W', Points.THIRD_SELECTION)
            ) {
                currentIteration.pointsPerStudent.set(student.id, Points.NO_SELECTION)
                algRun.pointsPerStudent.set(student.id, Points.NO_SELECTION)
            }
        }

        studentsCopy.sort((a, b) => {
            return (currentIteration.pointsPerStudent.get(b.id) || 0) - (currentIteration.pointsPerStudent.get(a.id) || 0)
        })

        for (const student of studentsCopy) {
            if (
                !tryAssignment(student.pWishes[0], student, currentIteration, 'P', Points.FIRST_SELECTION) &&
                !tryAssignment(student.pWishes[1], student, currentIteration, 'P', Points.SECOND_SELECTION) &&
                !tryAssignment(student.pWishes[2], student, currentIteration, 'P', Points.THIRD_SELECTION)
            ) {
                currentIteration.pointsPerStudent.set(student.id, Points.NO_SELECTION + (currentIteration.pointsPerStudent.get(student.id) || 0))
            }
        }

        if (best === null || best.totalPoints() > currentIteration.totalPoints()) {
            best = currentIteration
        }
    }

    if (!best) {
        throw new Error('Minimum of 1 iteration required')
    }

    return best
}
