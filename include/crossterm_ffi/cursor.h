#ifndef CROSSTERM_FFI_CURSOR_H
#define CROSSTERM_FFI_CURSOR_H

struct crossterm_stream;

int crossterm_show_cursor(struct crossterm_stream *stream);
int crossterm_hide_cursor(struct crossterm_stream *stream);

#endif
