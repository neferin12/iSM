#include <stdio.h>
#include <string.h>
#include <io.h>
#include <glib.h>
#include "algorithm.h"
#include "copy.h"

int main(int argc, char *argv[]){

    if (strcmp(argv[1], "version") == 0) {
        printf("1.0");
        return 0;
    }

    if (argc < 3) {
        printf("Usage: testISM <command> <Votes.csv> <Seminars.csv>");
        return -1;
    }
    GArray *wSeminars = getSeminars(argv[3], 'W');
    GArray *pSeminars = getSeminars(argv[3], 'P');
    GArray *students = getStudents(argv[2], wSeminars, pSeminars);

    if (strcmp(argv[1], "io") == 0) {

        if (wSeminars->len != 10) {
            printf("Invalid length WSeminars");
            return -1;
        }

        if (pSeminars->len != 10) {
            printf("Invalid length PSeminars");
            return -1;
        }

        if (students->len != 100) {
            printf("Invalid length Students");
            return -1;
        }
        return 0;
    }

    if (strcmp(argv[1], "alg") == 0) {
        GArray *finished = runAlgorithm(students, wSeminars, pSeminars);
        if (accumulatePoints(finished) > 1000) {
            printf("Large mimi points");
            return -1;
        }

        finished = batchRunAlgorithmn(10000, students, wSeminars, pSeminars);
        if (accumulatePoints(finished) > 1000) {
            printf("Large mimi points");
            return -1;
        }
        return 0;
    }

    if (strcmp(argv[1], "copy") == 0) {
        return run_copyStudents_test();
    }

    return -1;
}
