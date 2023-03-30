set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

add_executable(cism main.c io.c errorHandling.c organization.c headers/algorithm.h algorithm.c headers/constants.h constants.c)
add_executable(cismMPI mpiMain.c io.c errorHandling.c organization.c headers/algorithm.h algorithm.c headers/constants.h constants.c cismMPI.c headers/cismMPI.h)

find_package(PkgConfig REQUIRED)
find_package(MPI REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0)

target_link_libraries(cism PkgConfig::deps)
target_link_libraries(cismMPI PkgConfig::deps MPI::MPI_C)
