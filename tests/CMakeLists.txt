cmake_minimum_required(VERSION 3.20)

add_custom_target(check)
add_dependencies(check cargo_test)

function(add_single_file_test)
    set(options)
    set(oneValueArgs TEST_NAME)
    set(multiValueArgs TEST_SOURCES)
    cmake_parse_arguments(
        PARSE_ARGV 0 arg
        "${options}" "${oneValueArgs}" "${multiValueArgs}")

    add_executable(${arg_TEST_NAME} EXCLUDE_FROM_ALL ${arg_TEST_SOURCES})
    add_dependencies(${arg_TEST_NAME} build)

    target_compile_options(
        ${arg_TEST_NAME}
        PRIVATE ${C_COMPILE_OPTIONS})

    target_include_directories(
        ${arg_TEST_NAME}
        PRIVATE ${PROJECT_SOURCE_DIR}/include)
    target_link_directories(
        ${arg_TEST_NAME}
        PRIVATE ${CMAKE_SOURCE_DIR}/crossterm-rs/target/release)
    target_link_libraries(
        ${arg_TEST_NAME}
        PRIVATE crossterm_ffi)

    add_custom_target(check-${arg_TEST_NAME} ${arg_TEST_NAME})
    add_dependencies(check check-${arg_TEST_NAME})
endfunction()
