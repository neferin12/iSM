#include "ism/organization.h"
#include <stdlib.h>

/**
 * Frees the memory allocated for a seminar.
 *
 * @param sp Pointer to the seminar structure to be freed.
 */
void freeSeminar(seminar *sp){
    seminar s = *sp;
    free(s.name);
    g_free(sp->id);
}

/**
 * Frees the memory allocated for an array of seminars.
 *
 * @param seminars The array of seminars to be freed.
 */
void freeSeminars(GArray *seminars){
    g_array_free(seminars, TRUE);
}


/**
 * Frees the memory allocated for a student object.
 * 
 * @param sp Pointer to the student object to be freed.
 */
void freeStudent(student *sp){
    student s = *sp;
    free(s.name);
    free(s.wVotes);
    free(s.pVotes);
}

/**
 * Frees the memory allocated for an array of students.
 * 
 * @param students The array of students to be freed.
 */
void freeStudents(GArray *students){
    g_array_free(students, TRUE);
}

/**
 * Calculates the total accumulated points of the students.
 *
 * This function takes a GArray of students and iterates through each student,
 * accumulating their mimiPoints to calculate the total points.
 *
 * @param students A pointer to the GArray of students.
 * @return The total accumulated points of the students.
 */
int accumulatePoints(const GArray *students)
{
    int p = 0;
    for (guint i = 0; i < students->len; i++)
    {
        p += g_array_index(students, student, i).mimiPoints;
    }
    return p;
}
