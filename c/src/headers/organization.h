#ifndef CISM_ORGANIZATION_H
#define CISM_ORGANIZATION_H

#include <glib.h>

struct seminar {
    char *name;
    char seminarType;
    int size;
    int *id;
};
typedef struct seminar seminar;

struct student {
    char *name;
    int mimiPoints;
    seminar *wVotes;
    seminar *pVotes;
    seminar wSeminar;
    seminar pSeminar;
};
typedef struct student student;

void freeSeminars(GArray *seminars);
void freeSeminar(seminar *seminar);
void freeStudents(GArray *students);
void freeStudent(student *students);

int accumulatePoints(const GArray *students);

#endif //CISM_ORGANIZATION_H
