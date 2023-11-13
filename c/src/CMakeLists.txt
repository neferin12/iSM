add_library(ism SHARED algorithm.c io.c log.c organization.c errorHandling.c constants.c)

add_executable(cism main.c cismMPI.c headers/cismMPI.h log.c headers/ism/log.h)

find_package(PkgConfig REQUIRED)
find_package(MPI REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0)

target_link_libraries(ism PkgConfig::deps)

target_link_libraries(cism MPI::MPI_C ism)
