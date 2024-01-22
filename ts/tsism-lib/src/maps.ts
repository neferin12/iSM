import {Student} from "./student";
import {Seminar} from "./seminar";

export interface AssignmentMapInterface {
    get: (s: Student) => { wSeminar?: Seminar, pSeminar?: Seminar } | undefined
    set: (s: Student, value: { wSeminar?: Seminar, pSeminar?: Seminar }) => void
}

export class AssignmentMapArray implements AssignmentMapInterface {
    readonly a: Array<{ wSeminar?: Seminar; pSeminar?: Seminar }> = []

    get(s: Student): { wSeminar?: Seminar; pSeminar?: Seminar } | undefined {
        return this.a[s.id];
    }

    set(s: Student, value: { wSeminar?: Seminar; pSeminar?: Seminar }): void {
        this.a[s.id] = value;
    }
}

export interface NumberToNumberMapInterface {
    get: (id: number) => number | undefined
    set: (id: number, value: number) => void
    getAllValues: () => number[]
}

export class NumberToNumberArray implements NumberToNumberMapInterface {
    readonly a: number[] = []
    get(id: number): number | undefined {
        return (this.a)[id];
    }

    set(id: number, value: number): void {
        this.a[id] = value;
    }

    getAllValues(): number[] {
        return this.a;
    }
}