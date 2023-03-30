//
// Created by julian on 13.03.23.
//

#ifndef CISM_CISMMPI_H
#define CISM_CISMMPI_H

#include <glib.h>

GArray *mpiRun(int runsPerProcess, const char *filenameVotes, const char *filenameSeminars);
#endif //CISM_CISMMPI_H
