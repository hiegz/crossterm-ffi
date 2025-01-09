#ifndef CROSSTERM_FFI_STYLE_H
#define CROSSTERM_FFI_STYLE_H

#include <crossterm_ffi/attributes.h>
#include <crossterm_ffi/color.h>

#include <stdbool.h>

struct crossterm_style {
    struct crossterm_color foreground_color;
    struct crossterm_color background_color;
    struct crossterm_color underline_color;
    crossterm_attributes attributes;

    bool has_foreground_color;
    bool has_background_color;
    bool has_underline_color;
};

#endif
