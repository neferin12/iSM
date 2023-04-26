#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <glib.h>
#include "headers/io.h"
#include "headers/organization.h"
#include "headers/errorHandling.h"
#include "headers/log.h"


GArray* getSeminars(const char *filename, char type) {
    GArray *seminars = g_array_new(FALSE, FALSE, sizeof(seminar));
    g_array_set_clear_func(seminars, (GDestroyNotify) freeSeminar);
    FILE *file = fopen(filename, "r");
    if (file == NULL) {
        dieWithErrno("could not open file");
    }
    char *line = NULL;
    size_t len = 0;
    int linec = 0;
    while (getline(&line, &len, file) != -1) {
        linec++;
        line[strcspn(line, "\n")] = 0;
        failIfNull(seminars, "realloc");
        char *parsed = strtok(line, ";");
        char buf[80];
        snprintf(buf, 80, "seminars incorrectly configured at line %d\n", linec);
        failIfNull(parsed, buf);
        seminar lSeminar;
        lSeminar.name = malloc((strlen(parsed) + 1) * sizeof(char));
        strcpy( lSeminar.name, parsed);
        parsed = strtok(NULL, ";");
        failIfNull(parsed, buf);
        lSeminar.size = strtol(parsed, NULL, 10);
        parsed = strtok(NULL, ";");
        failIfNull(parsed, buf);
        if (strcmp(parsed, "W")==0) {
            lSeminar.seminarType = 'W';
        } else if (strcmp(parsed, "P")==0) {
            lSeminar.seminarType = 'P';
        } else {
            dieWithoutErrno(buf);
        }
        lSeminar.id = g_new0(gint, 1);
        *lSeminar.id = linec;
        if (parsed[0]==type) {
            seminars = g_array_append_vals(seminars, &lSeminar, 1);
        } else {
            free(lSeminar.name);
        }
    }
    fclose(file);

    free(line);
    return seminars;
}

GArray *getStudents(const char *filename, GArray *wSeminars, GArray *pSeminars) {
    GArray *students = g_array_new(FALSE, FALSE, sizeof(student));
    g_array_set_clear_func(students, (GDestroyNotify) freeStudent);
    FILE *file = fopen(filename, "r");
    if (file == NULL) {
        dieWithErrno("could not open file");
    }
    char *line = NULL;
    size_t len = 0;
    int linec = 0;
    while (getline(&line, &len, file) != -1) {
        linec++;
        line[strcspn(line, "\n")] = 0;
        failIfNull(students, "realloc");
        char *parsed = strtok(line, ";");
        char buf[80];
        snprintf(buf, 80, "students incorrectly configured at line %d\n", linec);
        failIfNull(parsed, buf);
        student lStudent;
        lStudent.name = malloc((strlen(parsed) + 1) * sizeof(char));
        strcpy(lStudent.name, parsed);
        // W Seminars
        lStudent.wVotes = malloc(sizeof(seminar) * 3);
        for (int i = 0; i < 3; ++i) {
            parsed = strtok(NULL, ";");
            guint ind = strtol(parsed, NULL, 10);
            failIfNull(parsed, buf);
            if (ind > wSeminars->len-1 ) {
                dieWithoutErrno(buf);
            }
            lStudent.wVotes[i] = g_array_index(wSeminars, seminar, ind);
        }
        // P Seminars
        lStudent.pVotes = malloc(sizeof(seminar) * 3);
        for (int i = 0; i < 3; ++i) {
            parsed = strtok(NULL, ";");
            failIfNull(parsed, buf);
            guint ind = strtol(parsed, NULL, 10);
            if ( ind > pSeminars->len-1) {
                dieWithoutErrno(buf);
            }
            lStudent.pVotes[i] = g_array_index(pSeminars, seminar, ind);
        }

        lStudent.mimiPoints = 0;

        students = g_array_append_vals(students, &lStudent, 1);
    }
    fclose(file);

    free(line);
    return students;
}

void outputResult(const GArray *finished){
    GString *result = g_string_new(NULL);
    g_string_append_printf(result,"Result:\n---------|%i|---------\n", accumulatePoints(finished));

    for (guint i = 0; i < finished->len; i++) {
        student s = g_array_index(finished, student, i);
        g_string_append_printf(result, "(%i) %s, %i, (W: %s | P: %s)\n", i + 1, s.name, s.mimiPoints, s.wSeminar.name, s.pSeminar.name);
    }

    log_info(result->str);

    g_string_free(result,TRUE);
}
