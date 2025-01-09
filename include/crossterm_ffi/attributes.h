#ifndef CROSSTERM_FFI_ATTRIBUTES_H
#define CROSSTERM_FFI_ATTRIBUTES_H

#include <stdint.h>

// clang-format off

#define CROSSTERM_RESET_ATTRIBUTE      (1 << 1)
#define CROSSTERM_BOLD_ATTRIBUTE       (1 << 2)
#define CROSSTERM_DIM_ATTRIBUTE        (1 << 3)
#define CROSSTERM_UNDERLINED_ATTRIBUTE (1 << 5)
#define CROSSTERM_REVERSE_ATTRIBUTE    (1 << 12)
#define CROSSTERM_HIDDEN_ATTRIBUTE     (1 << 13)

// clang-format on

typedef uint16_t crossterm_attributes;

#endif
