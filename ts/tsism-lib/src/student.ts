import {Seminar} from './seminar'



export class Student {
    readonly name: string
    readonly id: number
    readonly wWishes: [Seminar, Seminar, Seminar]
    readonly pWishes: [Seminar, Seminar, Seminar]

    constructor(id: number, name: string, wWishes: [Seminar, Seminar, Seminar], pWishes: [Seminar, Seminar, Seminar]) {
        this.id = id
        this.name = name
        this.pWishes = pWishes
        this.wWishes = wWishes
    }
}
