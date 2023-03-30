#include "organization.h"

#ifndef CISM_ALGORITHM_H
#define CISM_ALGORITHM_H
GArray *runAlgorithm(const GArray *students, const GArray *w_seminars, const GArray *p_seminars);
GArray *batchRunAlgorithmn(int times, const GArray *students, const GArray *w_seminars, const GArray *p_seminars);
#endif //CISM_ALGORITHM_H
