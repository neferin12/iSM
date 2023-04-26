#include <stdio.h>
#include <stdlib.h>
#include "headers/io.h"
#include "headers/organization.h"
#include "headers/algorithm.h"
#include "errorHandling.h"

int main(int argc, char *argv[]) {
    if (argc < 4) {
        fprintf(stderr, "Benutzung: cism <Wahldatei> <Seminardatei> <runs>\n");
        exit(1);
    }

    long runs = strtol(argv[3], NULL, 10);
    if (runs == LONG_MIN || runs == LONG_MAX) {
        dieWithErrno("Failed to parse runs argument");
    }
    if (runs > INT_MAX || runs < INT_MIN) {
        dieWithoutErrno("Invalid size for runs argument");
    }
    GArray *w_seminars = getSeminars(argv[2], 'W');
    GArray *p_seminars = getSeminars(argv[2], 'P');

    printf("W-Seminare:\n");
    for (guint i = 0; i < w_seminars->len; i++) {
        printf("    %d: %s (%i)\n", i, g_array_index(w_seminars, seminar, i).name, *g_array_index(w_seminars, seminar, i).id);
    }

    printf("P-Seminare:\n");
    for (guint i = 0; i < p_seminars->len; i++) {
        printf("    %d: %s (%i)\n", i, g_array_index(p_seminars, seminar, i).name, *g_array_index(p_seminars, seminar, i).id);
    }

    GArray *students = getStudents(argv[1], w_seminars, p_seminars);

    printf("Schüler: %i\n", students->len);
    printf("Seminare: %d\n", p_seminars->len + w_seminars->len);

    GArray *finished = batchRunAlgorithmn(runs,students, w_seminars, p_seminars);

    outputResult(finished);

    //free(finished.students);
    freeStudents(students);
    freeSeminars(w_seminars);
    freeSeminars(p_seminars);

    return 0;
}


