import {Seminar} from './seminar'

export class Student {
  readonly name: string
  readonly wWishes: [Seminar, Seminar, Seminar]
  readonly pWishes: [Seminar, Seminar, Seminar]
  readonly pointsPerRun: Map<number, number> = new Map<number, number>()

  constructor(name: string, wWishes: [Seminar, Seminar, Seminar], pWishes: [Seminar, Seminar, Seminar]) {
    this.name = name
    this.pWishes = pWishes
    this.wWishes = wWishes
  }
}
