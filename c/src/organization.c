#include "headers/organization.h"
#include <stdlib.h>

void freeSeminar(seminar *sp){
    seminar s = *sp;
    free(s.name);
    g_free(sp->id);
}

void freeSeminars(GArray *seminars){
    g_array_free(seminars, TRUE);
}

void freeStudent(student *sp){
    student s = *sp;
    free(s.name);
    free(s.wVotes);
    free(s.pVotes);
}

void freeStudents(GArray *students){
    g_array_free(students, TRUE);
}

int accumulatePoints(const GArray *students){
    int p = 0;
    for (int i = 0; i < students->len; i++) {
        p += g_array_index(students, student, i).mimiPoints;
    }
    return p;
}
