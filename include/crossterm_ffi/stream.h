#ifndef CROSSTERM_FFI_STREAM_H
#define CROSSTERM_FFI_STREAM_H

#include <stdio.h>

/// @brief Represents a stream used to send commands to the terminal.
struct crossterm_stream;

/// @brief Creates a new crossterm stream for a specified file.
struct crossterm_stream *crossterm_stream_new(FILE *file);

/// @brief Frees the memory allocated for a crossterm stream.
void crossterm_stream_free(struct crossterm_stream *stream);

#endif
