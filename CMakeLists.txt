cmake_minimum_required(VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

find_program(GCC_EXE gcc)
if (GCC_EXE)
    message(STATUS "Found gcc : ${GCC_EXE}.")
elseif (NOT GCC_EXE)
    message(FATAL_ERROR "gcc not found in $PATH.")
endif()
set(CMAKE_C_COMPILER ${GCC_EXE})
set(C_COMPILE_OPTIONS
    -Werror
    -Wall
    -Wextra
    -Wpedantic)

project(crossterm_ffi
        VERSION 0.1.0
        LANGUAGES C)

add_custom_target(
    cargo_test
    cargo test
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}/crossterm-rs)

add_custom_target(
    build
    cargo build --release
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}/crossterm-rs)

add_subdirectory(examples)
add_subdirectory(tests)

include(GNUInstallDirs)

install(
    FILES crossterm-rs/target/release/libcrossterm_ffi.a
    DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY include/crossterm_ffi
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
