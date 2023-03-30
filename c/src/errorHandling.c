#include "headers/errorHandling.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
void dieWithErrno(const char *msg){
    perror(msg);
    exit(errno);
}
void dieWithoutErrno(const char *msg){
    fprintf(stderr, "%s", msg);
    exit(EXIT_FAILURE);
}

void failIfNull(const void *anyPointer, const char *msg){
    if (anyPointer == NULL) {
        dieWithoutErrno(msg);
    }
}