#include "headers/cismMPI.h"
#include "headers/io.h"
#include "headers/algorithm.h"
#include "headers/errorHandling.h"
#include <mpi.h>
#include <stdio.h>
#include <stdbool.h>

GArray *mpiRun(int runsPerProcess, const char *filenameVotes, const char *filenameSeminars) {
    int process_Rank, size_Of_Comm, points;
    bool iAmBest;

    MPI_Init(NULL, NULL);
    MPI_Comm_size(MPI_COMM_WORLD, &size_Of_Comm);
    MPI_Comm_rank(MPI_COMM_WORLD, &process_Rank);

    GArray *wSeminars = getSeminars(filenameSeminars, 'W');
    GArray *pSeminars = getSeminars(filenameSeminars, 'P');

    GArray *students = getStudents(filenameVotes, wSeminars, pSeminars);

//    printf("%i running algorithm\n", process_Rank);
    GArray *finished = batchRunAlgorithmn(runsPerProcess, students, wSeminars, pSeminars);

    points = accumulatePoints(finished);
    //printf("%i finished algorithm\n", process_Rank);

    if (0 == process_Rank) {
        int *allPoints = calloc(size_Of_Comm, sizeof(int));
        failIfNull(allPoints, "Failed to allocate array to save points\n");

        allPoints[0] = points;

        for (int i = 1; i < size_Of_Comm; i++) {
            //printf("Receiving points from %i\n", i);
            MPI_Recv(&allPoints[i], 1, MPI_INT, i, 1, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        }

        int bestIndex = 0;
        for (int j = 1; j < size_Of_Comm; j++) {
            if (allPoints[j] < allPoints[bestIndex]) {
                bestIndex = j;
            }
        }
        //printf("Best index: %i\n", bestIndex);
        if (bestIndex == 0) {
            outputResult(finished);
        }
        for (int i = 1; i < size_Of_Comm; i++) {
            //printf("Send result to %i\n", i);
            bool best = i == bestIndex;
            MPI_Send(&best, 1, MPI_C_BOOL, i, 2, MPI_COMM_WORLD);
        }
    } else {
        for (int i = 1; i < size_Of_Comm; i++) {
            if (i == process_Rank) {
                //printf("Sending points from %i\n", i);
                MPI_Send(&points, 1, MPI_INT, 0, 1, MPI_COMM_WORLD);
            }
        }
        for (int i = 1; i < size_Of_Comm; i++) {
            if (i == process_Rank) {
                MPI_Recv(&iAmBest, 1, MPI_C_BOOL, 0, 2, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
                if (iAmBest) {
                    outputResult(finished);
                }
            }
        }
    }

    MPI_Barrier(MPI_COMM_WORLD);

    //printf("%i points from process %d of %d\n", points, process_Rank, size_Of_Comm);
    freeStudents(students);
    freeSeminars(wSeminars);
    freeSeminars(pSeminars);

    MPI_Finalize();
    return NULL;
}
