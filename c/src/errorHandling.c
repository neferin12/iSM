#include "ism/errorHandling.h"
#include "ism/log.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>


/**
 * @brief Terminates the program with an error message and the current value of errno.
 *
 * This function logs a fatal error message using the provided message and then terminates
 * the program with the current value of errno as the exit code.
 *
 * @param msg The error message to be logged.
 */
void dieWithErrno(const char *msg){
    log_fatal(msg);
    exit(errno);
}


/**
 * @brief Terminates the program with a fatal error message without setting the errno variable.
 *
 * This function logs a fatal error message using the log_fatal function and then terminates the program
 * with an exit status of EXIT_FAILURE.
 *
 * @param msg The error message to be logged.
 */
void dieWithoutErrno(const char *msg){
    log_fatal("%s", msg);
    exit(EXIT_FAILURE);
}


/**
 * @brief Terminates the program with a fatal error message if the given pointer is NULL.
 *
 * This function checks if the given pointer is NULL and if so, terminates the program with a fatal error message.
 *
 * @param anyPointer The pointer to be checked.
 * @param msg The error message to be logged.
 */
void failIfNull(const void *anyPointer, const char *msg){
    if (anyPointer == NULL) {
        dieWithoutErrno(msg);
    }
}