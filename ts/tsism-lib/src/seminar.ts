export enum SeminarType {
  W_SEMINAR,
  P_SEMINAR,
}

export class Seminar {
  readonly name: string
  readonly capacity: number
  readonly type: SeminarType
  readonly remainingCapacityPerRun: Map<number, number> = new Map<number, number>()

  constructor(name: string, capacity: number, type: SeminarType) {
    this.name = name
    this.capacity = capacity
    this.type = type
  }
}
