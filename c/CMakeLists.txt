cmake_minimum_required(VERSION 3.0)

project(cISM)

set(CMAKE_C_STANDARD 11)

set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

include_directories(${PROJECT_SOURCE_DIR}/src/headers)

add_subdirectory(src)

install(TARGETS cism cismMPI)

# Tests
enable_testing()
find_package(PkgConfig REQUIRED)
find_package(MPI REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0)


add_executable(testISM tests/testIsm.c src/algorithm.c src/cismMPI.c src/constants.c src/errorHandling.c src/io.c src/organization.c src/log.c)
target_link_libraries(testISM PkgConfig::deps MPI::MPI_C)
add_test(NAME IO COMMAND testISM io ${PROJECT_SOURCE_DIR}/example-files/Wahlen.csv ${PROJECT_SOURCE_DIR}/example-files/Seminare.csv)
add_test(NAME Algorithm COMMAND testISM alg ${PROJECT_SOURCE_DIR}/example-files/Wahlen.csv ${PROJECT_SOURCE_DIR}/example-files/Seminare.csv)
