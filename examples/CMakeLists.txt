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
add_crossterm_example(EXAMPLE_NAME bad-alternate-screen EXAMPLE_SOURCES bad_alternate_screen.c)
add_crossterm_example(EXAMPLE_NAME scroll-up EXAMPLE_SOURCES scroll_up.c)
add_crossterm_example(EXAMPLE_NAME scroll-down EXAMPLE_SOURCES scroll_down.c)
add_crossterm_example(EXAMPLE_NAME clear-all EXAMPLE_SOURCES clear_all.c)
add_crossterm_example(EXAMPLE_NAME clear-purge EXAMPLE_SOURCES clear_purge.c)
add_crossterm_example(EXAMPLE_NAME clear-from-cursor-up EXAMPLE_SOURCES clear_from_cursor_up.c)
add_crossterm_example(EXAMPLE_NAME clear-current-line EXAMPLE_SOURCES clear_current_line.c)
add_crossterm_example(EXAMPLE_NAME show-cursor EXAMPLE_SOURCES show_cursor.c)
add_crossterm_example(EXAMPLE_NAME hide-cursor EXAMPLE_SOURCES hide_cursor.c)
