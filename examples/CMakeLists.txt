cmake_minimum_required(VERSION 3.20)

function(add_crossterm_example)
    set(options)
    set(oneValueArgs EXAMPLE_NAME)
    set(multiValueArgs EXAMPLE_SOURCES)
    cmake_parse_arguments(
        PARSE_ARGV 0 arg
        "${options}" "${oneValueArgs}" "${multiValueArgs}")

    add_executable(${arg_EXAMPLE_NAME} ${arg_EXAMPLE_SOURCES})
    add_dependencies(${arg_EXAMPLE_NAME} crossterm_rs)

    target_compile_options(
        ${arg_EXAMPLE_NAME}
        PRIVATE ${C_COMPILE_OPTIONS})

    target_include_directories(
        ${arg_EXAMPLE_NAME}
        PRIVATE ${PROJECT_SOURCE_DIR}/include)
    target_link_directories(
        ${arg_EXAMPLE_NAME}
        PRIVATE ${CMAKE_SOURCE_DIR}/crossterm-rs/target/release)
    target_link_libraries(
        ${arg_EXAMPLE_NAME}
        PRIVATE crossterm_ffi)

    add_custom_target(${arg_EXAMPLE_NAME}-example ${arg_EXAMPLE_NAME})
endfunction()

add_crossterm_example(EXAMPLE_NAME alternate-screen EXAMPLE_SOURCES alternate_screen.c)
