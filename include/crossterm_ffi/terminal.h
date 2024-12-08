#ifndef CROSSTERM_FFI_TERMINAL_H
#define CROSSTERM_FFI_TERMINAL_H

struct crossterm_stream;

int crossterm_enter_alternate_screen(struct crossterm_stream *stream);
int crossterm_leave_alternate_screen(struct crossterm_stream *stream);

#endif
