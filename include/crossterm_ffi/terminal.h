#ifndef CROSSTERM_FFI_TERMINAL_H
#define CROSSTERM_FFI_TERMINAL_H

#include <stdint.h>

struct crossterm_stream;

int crossterm_enter_alternate_screen(struct crossterm_stream *stream);
int crossterm_leave_alternate_screen(struct crossterm_stream *stream);
int crossterm_scroll_up(struct crossterm_stream *stream, uint16_t nlines);
int crossterm_scroll_down(struct crossterm_stream *stream, uint16_t nlines);

#endif
