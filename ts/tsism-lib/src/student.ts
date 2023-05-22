import {Seminar} from './seminar'

export interface NumberToNumberMapInterface {
    get: (id: number) => number | undefined
    set: (id: number, value: number) => void
}

export class NumberToNumberArray implements NumberToNumberMapInterface {
    readonly a: number[] = []
    get(id: number): number | undefined {
        return (this.a)[id];
    }

    set(id: number, value: number): void {
        this.a[id] = value;
    }
}

export class Student {
    readonly name: string
    readonly id: number
    readonly wWishes: [Seminar, Seminar, Seminar]
    readonly pWishes: [Seminar, Seminar, Seminar]
    readonly pointsPerRun: NumberToNumberMapInterface = new NumberToNumberArray()

    constructor(id: number, name: string, wWishes: [Seminar, Seminar, Seminar], pWishes: [Seminar, Seminar, Seminar]) {
        this.id = id
        this.name = name
        this.pWishes = pWishes
        this.wWishes = wWishes
    }
}
