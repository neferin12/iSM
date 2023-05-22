import {Args, Command} from '@oclif/core'
import {importSeminars, importStudents, printStudents} from '../../io'
import runAlgorithm from '../../algorithm'

export default class Run extends Command {
  static description = 'Run the algorithm';

  static args = {
    students: Args.file({description: 'The file containing the students', required: true}),
    seminars: Args.file({description: 'The file containing the seminars', required: true}),
    runs: Args.integer({description: 'Number of iterations of the algorithm', min: 1, required: true}),
  }

  async run(): Promise<any> {
    const {args} = await this.parse(Run)
    const seminars = await importSeminars(args.seminars)
    const students = await importStudents(args.students, seminars)
    const it = await runAlgorithm(args.runs, students)
    printStudents(students, it)
    return null
  }
}
