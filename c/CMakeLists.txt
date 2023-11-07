cmake_minimum_required(VERSION 3.5)

project(cISM VERSION 1.0.1 DESCRIPTION "C implementation of iSM")

include(GNUInstallDirs)

set(CMAKE_C_STANDARD 11)

set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wextra -Wno-missing-braces -DLOG_USE_COLOR")
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -g")
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3")

option(INSTALL_LIB "Install the ism library" ON)
option(INSTALL_EXEC "Install the cism executable" ON)

add_subdirectory(src)
configure_file(ism.pc.in ism.pc @ONLY)

# Conditional install for the library
if(INSTALL_LIB)
    install(TARGETS ism
            LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
            PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
    install(FILES ${CMAKE_BINARY_DIR}/ism.pc DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/pkgconfig)
endif()

# Conditional install for the executable
if(INSTALL_EXEC)
    install(TARGETS cism
            RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
endif()


# Tests
enable_testing()
find_package(PkgConfig REQUIRED)
find_package(MPI REQUIRED)
pkg_check_modules(deps REQUIRED IMPORTED_TARGET glib-2.0)
add_executable(testISM tests/testIsm.c tests/copy.c src/cismMPI.c)
set_target_properties(testISM PROPERTIES EXCLUDE_FROM_ALL TRUE)
target_link_libraries(testISM PkgConfig::deps MPI::MPI_C ism)
add_test(test_build
  "${CMAKE_COMMAND}"
  --build "${CMAKE_BINARY_DIR}"
  --config "$<CONFIG>"
  --target testISM
)
set_tests_properties(test_build PROPERTIES FIXTURES_SETUP    test_fixture)
add_test(testISM testISM version)
set_tests_properties(testISM       PROPERTIES FIXTURES_REQUIRED test_fixture)
add_test(NAME IO COMMAND testISM io ${PROJECT_SOURCE_DIR}/../example-files/Wahlen.csv ${PROJECT_SOURCE_DIR}/../example-files/Seminare.csv)
add_test(NAME DeepCopyStudents COMMAND testISM copy ${PROJECT_SOURCE_DIR}/../example-files/Wahlen.csv ${PROJECT_SOURCE_DIR}/../example-files/Seminare.csv)
add_test(NAME Algorithm COMMAND testISM alg ${PROJECT_SOURCE_DIR}/../example-files/Wahlen.csv ${PROJECT_SOURCE_DIR}/../example-files/Seminare.csv)