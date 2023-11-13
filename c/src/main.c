#include <stdio.h>
#include "headers/cismMPI.h"
#include "ism/errorHandling.h"
#include "ism/log.h"

int main(int argc, char *argv[]){
    if (argc < 4) {
        log_fatal("Benutzung: cism <Wahldatei> <Seminardatei> <runs>");
        exit(1);
    }
    long runs = strtol(argv[3], NULL, 10);
    if (runs == LONG_MIN || runs == LONG_MAX) {
        dieWithErrno("Failed to parse runs argument");
    }
    if (runs > INT_MAX || runs < INT_MIN) {
        dieWithoutErrno("Invalid size for runs argument");
    }
    mpiRun(runs, argv[1], argv[2]);
}
