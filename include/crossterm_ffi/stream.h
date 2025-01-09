#ifndef CROSSTERM_FFI_STREAM_H
#define CROSSTERM_FFI_STREAM_H

#include <stdint.h>
#include <stdio.h>
struct crossterm_style;

/// @brief Represents a stream used to send commands to the terminal.
struct crossterm_stream;

/// @brief Creates a new crossterm stream for a specified file.
struct crossterm_stream *crossterm_stream_new(FILE *file);

/// @brief Writes the specified number of bytes from the buffer to a stream
///        applying the given style.
int crossterm_stream_write(struct crossterm_stream *stream, const uint8_t *buf,
                           size_t buflen, const struct crossterm_style *style);

/// @brief Flushes the given stream, ensuring that all intermediately
///        buffered contents reach their destination.
int crossterm_stream_flush(struct crossterm_stream *stream);

/// @brief Frees the memory allocated for a crossterm stream.
void crossterm_stream_free(struct crossterm_stream *stream);

#endif
