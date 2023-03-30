//
// Created by julian on 15.12.21.
//

#ifndef CISM_ERRORHANDLING_H
#define CISM_ERRORHANDLING_H
void dieWithErrno(const char *msg);
void dieWithoutErrno(const char *msg);
void failIfNull(const void *anyPointer, const char *msg);
#endif //CISM_ERRORHANDLING_H
