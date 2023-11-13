#include "ism/errorHandling.h"
#include "ism/log.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
void dieWithErrno(const char *msg){
    log_fatal(msg);
    exit(errno);
}
void dieWithoutErrno(const char *msg){
    log_fatal("%s", msg);
    exit(EXIT_FAILURE);
}

void failIfNull(const void *anyPointer, const char *msg){
    if (anyPointer == NULL) {
        dieWithoutErrno(msg);
    }
}