#ifndef CROSSTERM_FFI_STREAM_H
#define CROSSTERM_FFI_STREAM_H

#include <stdint.h>
#include <stdio.h>

struct crossterm_style;

/// @brief Represents a generic stream.
struct crossterm_stream {
    void *context;

    ///
    /// @brief   Expected to write the specified number of bytes from the
    ///          buffer to an implementation-dependent destination.
    ///
    /// @returns Should return the number of bytes written successfully.
    ///          On failure, should return a negative value representing the
    ///          error.
    long (*write_fn)(const uint8_t *buf, size_t buflen, void *context);

    ///
    /// @brief    Expected to flush all intermediately buffered contents.
    ///
    /// @returns  Should return 0 on success. On failure, should return a
    ///           negative value.
    int (*flush_fn)(void *context);
};

///
/// @brief   Writes the specified number of bytes from the buffer to a generic
///          stream applying the given style.
///
/// @returns 0 on success. On failure, returns a negative error code.
///
int crossterm_stream_write(const struct crossterm_stream *stream,
                           const uint8_t *buf, size_t buflen,
                           const struct crossterm_style *style);

///
/// @brief   Flushes the given stream, ensuring that all intermediately
///          buffered contents reach their destination.
///
/// @returns 0 on success. On failure, returns a negative error code.
///
int crossterm_stream_flush(const struct crossterm_stream *stream);

#endif
