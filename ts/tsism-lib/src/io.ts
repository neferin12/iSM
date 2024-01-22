import {Seminar, SeminarType} from './seminar'
import * as fs from 'node:fs'
import {parse} from 'csv-parse'
import {Student} from './student'
import {Iteration} from './iteration'

export type SeminarData = { wSeminars: Seminar[], pSeminars: Seminar[] }

export function importSeminars(file: string): Promise<SeminarData> {
    return new Promise<SeminarData>((resolve, reject) => {
        const content = fs.readFileSync(file)
        parse(content, {bom: true, delimiter: ';'}, (err, results) => {
            if (err) {
                reject(err)
            } else {
                const ret: SeminarData = {
                    wSeminars: [],
                    pSeminars: [],
                }
                let currID = 0;
                for (const result of results) {
                    if (!Array.isArray(result) || result.length < 3) {
                        console.warn(`Skip invalid line: "${result}"`)
                        continue
                    }

                    switch (result[2]) {
                        case 'W':
                            ret.wSeminars.push(new Seminar(currID, result[0], Number.parseInt(result[1], 10), SeminarType.W_SEMINAR))
                            break
                        case 'P':
                            ret.pSeminars.push(new Seminar(currID, result[0], Number.parseInt(result[1], 10), SeminarType.P_SEMINAR))
                            break
                        default:
                            throw new Error('Unknown Seminar Type ' + result[3])
                    }
                    currID++;
                }

                resolve(ret)
            }
        })
    })
}

export function importStudents(file: string, seminars: SeminarData): Promise<Student[]> {
    return new Promise<Student[]>((resolve, reject) => {
        const content = fs.readFileSync(file)
        parse(content, {bom: true, delimiter: ';'}, (err, results) => {
            if (err) {
                reject(err)
            } else {
                const ret: Student[] = []
                for (const [i, result] of results.entries()) {
                    if (!Array.isArray(result) || result.length < 7) {
                        console.warn(`Skip invalid line: "${result}"`)
                        continue
                    }

                    ret.push(
                        new Student(
                            i,
                            result[0],
                            result.slice(1, 4).map(i => Number.parseInt(i, 10)).map(i => seminars.wSeminars[i]) as [Seminar, Seminar, Seminar],
                            result.slice(4, 7).map(i => Number.parseInt(i, 10)).map(i => seminars.pSeminars[i]) as [Seminar, Seminar, Seminar],
                        ),
                    );
                }

                resolve(ret)
            }
        })
    })
}

export function printStudents(students: Student[], iteration: Iteration): void {
    console.log(`---------|${iteration.totalPoints()}|---------`)
    for (const [i, student] of students.entries()) {
        console.log(`(${i + 1}) ${student.name}, ${iteration.pointsPerStudent.get(student.id)}, (W: ${iteration.assignments.get(student)?.wSeminar?.name || 'none'} | P: ${iteration.assignments.get(student)?.pSeminar?.name || 'none'})`)
    }
}
