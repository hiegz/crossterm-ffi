#ifndef CROSSTERM_FFI_EVENT_H
#define CROSSTERM_FFI_EVENT_H
// clang-format off

#include <stdint.h>

#define CROSSTERM_NO_KEY_MODIFIERS       (0b0000000000000000)
#define CROSSTERM_CONTROL_KEY_MODIFIER   (0b0000000000000001)
#define CROSSTERM_SHIFT_KEY_MODIFIER     (0b0000000000000010)
#define CROSSTERM_ALT_KEY_MODIFIER       (0b0000000000000100)
#define CROSSTERM_CAPS_LOCK_KEY_MODIFIER (0b0000000000001000)
#define CROSSTERM_NUM_LOCK_KEY_MODIFIER  (0b0000000000010000)
#define CROSSTERM_KEYPAD_KEY_MODIFIER    (0b0000000000100000)
#define CROSSTERM_SUPER_KEY_MODIFIER     (0b0000000001000000)
#define CROSSTERM_HYPER_KEY_MODIFIER     (0b0000000010000000)
#define CROSSTERM_META_KEY_MODIFIER      (0b0000000100000000)
#define CROSSTERM_ALL_KEY_MODIFIERS      (0b0000000111111111)

typedef void (*crossterm_character_key_event_handler)(uint32_t character, uint16_t modifiers, void *data);
typedef void (*crossterm_special_key_event_handler)   (uint8_t key,       uint16_t modifiers, void *data);
typedef void (*crossterm_resize_event_handler)       (uint16_t rows,      uint16_t cols,      void *data);

enum crossterm_special_key {
    CROSSTERM_BACKSPACE_KEY = 0,
    CROSSTERM_ENTER_KEY,
    CROSSTERM_LEFT_ARROW_KEY,
    CROSSTERM_RIGHT_ARROW_KEY,
    CROSSTERM_UP_ARROW_KEY,
    CROSSTERM_DOWN_ARROW_KEY,
    CROSSTERM_HOME_KEY,
    CROSSTERM_END_KEY,
    CROSSTERM_PAGE_UP_KEY,
    CROSSTERM_PAGE_DOWN_KEY,
    CROSSTERM_TAB_KEY,
    CROSSTERM_BACKTAB_KEY,
    CROSSTERM_DELETE_KEY,
    CROSSTERM_INSERT_KEY,
    CROSSTERM_ESCAPE_KEY,

    CROSSTERM_F1_KEY = 244,
    CROSSTERM_F2_KEY,
    CROSSTERM_F3_KEY,
    CROSSTERM_F4_KEY,
    CROSSTERM_F5_KEY,
    CROSSTERM_F6_KEY,
    CROSSTERM_F7_KEY,
    CROSSTERM_F8_KEY,
    CROSSTERM_F9_KEY,
    CROSSTERM_F10_KEY,
    CROSSTERM_F11_KEY,
    CROSSTERM_F12_KEY,
};

struct crossterm_event_handler_registry;

struct crossterm_event_handler_registry *crossterm_event_handler_registry_new(void);
void crossterm_event_handler_registry_free(struct crossterm_event_handler_registry *);

void crossterm_register_character_key_event_handler(
    struct crossterm_event_handler_registry *,
    crossterm_character_key_event_handler character_key_event_handler);
void crossterm_deregister_character_key_event_handler(
    struct crossterm_event_handler_registry *);

void crossterm_register_special_key_event_handler(
    struct crossterm_event_handler_registry *,
    crossterm_special_key_event_handler special_key_event_handler);
void crossterm_deregister_special_key_event_handler(
    struct crossterm_event_handler_registry *);

void crossterm_register_resize_event_handler(
    struct crossterm_event_handler_registry *,
    crossterm_resize_event_handler resize_event_handler);
void crossterm_deregister_resize_event_handler(
    struct crossterm_event_handler_registry *);

void crossterm_set_event_handler_data(
    struct crossterm_event_handler_registry *,
    void *handler_data);

int crossterm_event_read(struct crossterm_event_handler_registry *);
int crossterm_event_poll(int *is_available);

// clang-format on
#endif
