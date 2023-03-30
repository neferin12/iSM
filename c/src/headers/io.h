#include "organization.h"
#ifndef CISM_FILEINTERFACE_H
#define CISM_FILEINTERFACE_H
GArray *getSeminars(const char *filename, char type);
GArray *getStudents(const char *filename, GArray *wSeminars, GArray *pSeminars);

void outputResult(const GArray *finished);
#endif //CISM_FILEINTERFACE_H
