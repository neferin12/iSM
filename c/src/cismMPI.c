#include "headers/cismMPI.h"
#include "ism/io.h"
#include "ism/algorithm.h"
#include "ism/errorHandling.h"
#include "ism/log.h"
#include <mpi.h>
#include <stdbool.h>

GArray *mpiRun(int runsPerProcess, const char *filenameVotes, const char *filenameSeminars) {
    log_set_level(LOG_INFO);

    int process_Rank, size_Of_Comm, points;
    bool iAmBest;

    MPI_Init(NULL, NULL);
    MPI_Comm_size(MPI_COMM_WORLD, &size_Of_Comm);
    MPI_Comm_rank(MPI_COMM_WORLD, &process_Rank);

    GArray *wSeminars = getSeminars(filenameSeminars, 'W');
    GArray *pSeminars = getSeminars(filenameSeminars, 'P');

    GArray *students = getStudents(filenameVotes, wSeminars, pSeminars);

    log_debug("%i running algorithm", process_Rank);
    GArray *finished = batchRunAlgorithmn(runsPerProcess, students, wSeminars, pSeminars);

    points = accumulatePoints(finished);
    log_debug("%i finished algorithm", process_Rank);

    if (0 == process_Rank) {
        int *allPoints = calloc(size_Of_Comm, sizeof(int));
        failIfNull(allPoints, "Failed to allocate array to save points\n");

        allPoints[0] = points;

        for (int i = 1; i < size_Of_Comm; i++) {
            log_debug("Receiving points from %i", i);
            MPI_Recv(&allPoints[i], 1, MPI_INT, i, 1, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        }

        int bestIndex = 0;
        for (int j = 1; j < size_Of_Comm; j++) {
            if (allPoints[j] < allPoints[bestIndex]) {
                bestIndex = j;
            }
        }
        log_debug("Best index: %i", bestIndex);
        if (bestIndex == 0) {
            outputResult(finished);
        }
        for (int i = 1; i < size_Of_Comm; i++) {
            log_debug("Send result to %i", i);
            bool best = i == bestIndex;
            MPI_Send(&best, 1, MPI_C_BOOL, i, 2, MPI_COMM_WORLD);
        }
    } else {
        for (int i = 1; i < size_Of_Comm; i++) {
            if (i == process_Rank) {
                log_debug("Sending points from %i", i);
                MPI_Send(&points, 1, MPI_INT, 0, 1, MPI_COMM_WORLD);
            }
        }
        for (int i = 1; i < size_Of_Comm; i++) {
            if (i == process_Rank) {
                MPI_Recv(&iAmBest, 1, MPI_C_BOOL, 0, 2, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
                if (iAmBest) {
                    log_debug("%i is printing results, because it's best", i);
                    outputResult(finished);
                }
            }
        }
    }

    log_debug("%i is awaiting barrier", process_Rank);
    MPI_Barrier(MPI_COMM_WORLD);
    log_debug("%i is after barrier", process_Rank);

    log_debug("%i points from process %d of %d", points, process_Rank, size_Of_Comm);
    freeStudents(students);
    freeSeminars(wSeminars);
    freeSeminars(pSeminars);

    MPI_Finalize();
    return NULL;
}
