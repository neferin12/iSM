#include "ism/algorithm.h"
#include "ism/errorHandling.h"
#include "ism/constants.h"
#include <stdlib.h>
#include <sys/time.h>
#include <glib.h>
#include <stdbool.h>

/**
 * Shuffles the elements of an integer array.
 *
 * @param array The array to be shuffled.
 * @param n The number of elements in the array.
 */
static void shuffle(int *array, int n) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    long usec = tv.tv_usec;
    srand48(usec);

    if (n > 1) {
        int i;
        for (i = n - 1; i > 0; i--) {
            size_t j = (unsigned int) (drand48() * (i + 1));
            int t = array[j];
            array[j] = array[i];
            array[i] = t;
        }
    }
}

/**
 * @brief Returns an array of integers in the range [0, n-1].
 *
 * This function dynamically allocates an array of integers and fills it with
 * values in the range [0, n-1]. The caller is responsible for freeing the
 * memory allocated by this function.
 *
 * @param n The upper bound of the range (exclusive).
 * @return A pointer to the dynamically allocated array of integers.
 */
static int *getIntRange(int n){
    int *ints = malloc(sizeof(int) * n);
    failIfNull(ints, "could not malloc ints range");
    for (int i = 0; i < n; i++) {
        ints[i] = i;
    }
    return ints;
}


/**
 * Creates a copy of the given array of students.
 *
 * @param students The original array of students.
 * @return A new array containing a copy of the students.
 */
GArray *copyStudents(const GArray *students) {
    GArray *lStudents = g_array_new(FALSE, FALSE, sizeof(student));
    //g_array_set_clear_func(lStudents, (GDestroyNotify) freeStudent);
    failIfNull(lStudents, "could not create copy of lStudents (malloc)");
    for (guint i = 0; i < students->len; i++) {
        student h = g_array_index(students, student, i);
        student t = {.mimiPoints = 0, .name=h.name, .pSeminar=NULL, .wSeminar = NULL, .pVotes = h.pVotes, .wVotes = h.wVotes};
        lStudents = g_array_append_vals(lStudents, &t, 1);
    }
    return lStudents;
}

/**
 * Compares two students based on their points.
 *
 * @param s1 Pointer to the first student.
 * @param s2 Pointer to the second student.
 * @return Negative value if s1 has fewer points than s2, positive value if s1 has more points than s2,
 *         and 0 if both students have the same number of points.
 */
static int compareStudentsByPoints(const student *s1, const student *s2){
    return s2->mimiPoints-s1->mimiPoints;
}


/**
 * Tries to assign a student to a seminar.
 *
 * @param s The student to assign.
 * @param sel The seminar to assign the student to.
 * @param assignments The array of number of assignments for each seminar.
 * @param points The points to add to the student's mimiPoints.
 * @param semType The type of the seminar ('w' for W-Seminar, 'p' for P-Seminar).
 * @return TRUE if the assignment was successful, FALSE otherwise.
 */
static bool tryAssignment(student *s, seminar sel, int *assigments, int points, char semType){
    int assignedStudents = assigments[*sel.id];
    if (assignedStudents < sel.size) {
        assigments[*sel.id] = assignedStudents + 1;
        s->mimiPoints += points;
        switch (semType) {
            case 'w':
                s->wSeminar = sel;
                break;
            case 'p':
                s->pSeminar = sel;
                break;
            default:
                dieWithoutErrno("invalid seminar type");
        }
        return TRUE;
    }
    return FALSE;
}

/**
 * Runs the algorithm multiple times on the given input data.
 *
 * @param times The number of times to run the algorithm.
 * @param students The array of students.
 * @param w_seminars The array of workshops for each student.
 * @param p_seminars The array of presentations for each student.
 * @return A pointer to the resulting array.
 */
GArray *batchRunAlgorithm(int times, const GArray *students, const GArray *w_seminars, const GArray *p_seminars){
    GArray *best = NULL;
    int bestPoints = INT_MAX;
    for (int i = 0; i < times; i++) {
        GArray *tmp = runAlgorithm(students, w_seminars, p_seminars);
        int tmpPoints = accumulatePoints(tmp);
        if (tmpPoints < bestPoints) {
            if (best != NULL) {
                g_array_free(best, TRUE);
            }
            best = tmp;
            bestPoints = tmpPoints;
        } else {
            g_array_free(tmp, TRUE);
        }
    }
    return best;
}

/**
 * Runs the algorithm to assign students to seminars based on their votes.
 *
 * @param students The array of students.
 * @param w_seminars The array of seminars for the 'w' category.
 * @param p_seminars The array of seminars for the 'p' category.
 * @return The array of students with assigned seminars.
 */
GArray *runAlgorithm(const GArray *students, const GArray *w_seminars, const GArray *p_seminars) {
    int *assignments = calloc(w_seminars->len + p_seminars->len, sizeof(int));

    GArray *cStudents = copyStudents(students);
    int *intrange = getIntRange(cStudents->len);
    shuffle(intrange, cStudents->len);
    for (guint i = 0; i < cStudents->len; i++) {
        student *s = &g_array_index(cStudents, student, intrange[i]);
        if (!tryAssignment(s, s->wVotes[0], assignments, default_points.first_selection, 'w')) {
            if (!tryAssignment(s, s->wVotes[1], assignments, default_points.second_selection, 'w')) {
                if (!tryAssignment(s, s->wVotes[2], assignments, default_points.third_selection, 'w')) {
                    s->mimiPoints += default_points.no_selection;
                }
            }
        }

    }
    free(intrange);
    g_array_sort(cStudents, (GCompareFunc) compareStudentsByPoints);
    for (guint i = 0; i < cStudents->len; i++) {
        student *s = &g_array_index(cStudents, student, i);
        if (!tryAssignment(s, s->pVotes[0], assignments, default_points.first_selection, 'p')) {
            if (!tryAssignment(s, s->pVotes[1], assignments, default_points.second_selection, 'p')) {
                if (!tryAssignment(s, s->pVotes[2], assignments, default_points.third_selection, 'p')) {
                    s->mimiPoints += default_points.no_selection;
                }
            }
        }

    }
    free(assignments);
    return cStudents;
}
