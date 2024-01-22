#include <stdio.h>
#include <string.h>
#include "ism/io.h"
#include <glib.h>
#include "ism/algorithm.h"
#include "copy.h"
#include "ism/constants.h"

int main(int argc, char *argv[]){


    if (argc == 2 && strcmp(argv[1], "version") == 0) {
        printf("1.0\n");
        return 0;
    }

    if (argc < 4) {
        printf("Usage: testISM <command> <Votes.csv> <Seminars.csv>\n");
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

        finished = batchRunAlgorithm(10000, students, wSeminars, pSeminars);
        if (accumulatePoints(finished) > 1000) {
            printf("Large mimi points");
            return -1;
        }

        int manuallyAccumulatedPoints = 0;

        for(int i = 0; i < finished->len; i++) {
            student s = g_array_index(finished, student, i);
            int studentPoints = 0;

            if (s.wSeminar.id == s.wVotes[0].id) {
                studentPoints += default_points.first_selection;
            }else if (s.wSeminar.id == s.wVotes[1].id) {
                studentPoints += default_points.second_selection;
            }else if (s.wSeminar.id == s.wVotes[2].id) {
                studentPoints += default_points.third_selection;
            }else {
                studentPoints += default_points.no_selection;
            }

            if(s.pSeminar.id == s.pVotes[0].id) {
                studentPoints += default_points.first_selection;
            }else if (s.pSeminar.id == s.pVotes[1].id) {
                studentPoints += default_points.second_selection;
            }else if (s.pSeminar.id == s.pVotes[2].id) {
                studentPoints += default_points.third_selection;
            }else {
                studentPoints += default_points.no_selection;
            }

            if (s.mimiPoints != studentPoints) {
                printf("Student's points are incorrect. Expected %i, got %i for student %s at index %i", studentPoints, s.mimiPoints, s.name, i);
                return -1;
            }

            manuallyAccumulatedPoints += studentPoints;
        }

        if (manuallyAccumulatedPoints != accumulatePoints(finished)) {
            printf("Accumulated points are incorrect. Expected %i, got %i", manuallyAccumulatedPoints, accumulatePoints(finished));
            return -1;
        }
        return 0;
    }

    if (strcmp(argv[1], "copy") == 0) {
        return run_copyStudents_test();
    }

    return -1;
}
