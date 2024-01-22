export enum SeminarType {
  W_SEMINAR,
  P_SEMINAR,
}

export class Seminar {
  readonly id: number
  readonly name: string
  readonly capacity: number
  readonly type: SeminarType

  constructor(id: number, name: string, capacity: number, type: SeminarType) {
    this.id = id
    this.name = name
    this.capacity = capacity
    this.type = type
  }
}
