import {NumberToNumberArray, NumberToNumberMapInterface} from "./maps";
import {AssignmentMapArray, AssignmentMapInterface} from "./maps";

export class Iteration {
    private points: number | null = null;
    assignments: AssignmentMapInterface = new AssignmentMapArray();
    seminarCapacity: NumberToNumberMapInterface = new NumberToNumberArray();
    pointsPerStudent: NumberToNumberMapInterface = new NumberToNumberArray();

    /**
     * Returns the total points of this iteration. Once calculated, the value is cached.
     */
    totalPoints(): number {
        if (!this.points) {
            this.points = this.pointsPerStudent.getAllValues().reduce((a, b) => a + b, 0);
        }
        return this.points;
    }

}