export type Seminar = {
    id: number,
    name: string,
    size: number,
}

export type Student = {
    name: string,
    mimiPoints: number,
    wVotes: Seminar[],
    pVotes: Seminar[],
    wSeminar: Seminar,
    pSeminar: Seminar,
}