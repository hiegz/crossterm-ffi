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

typedef void (*crossterm_character_key_event_handler)(uint32_t character, uint16_t modifiers, void *token);
typedef void (*crossterm_f_key_event_handler)        (uint8_t key,        uint16_t modifiers, void *token);
typedef void (*crossterm_backspace_key_event_handler)                    (uint16_t modifiers, void *token);
typedef void (*crossterm_enter_key_event_handler)                        (uint16_t modifiers, void *token);
typedef void (*crossterm_left_arrow_key_event_handler)                   (uint16_t modifiers, void *token);
typedef void (*crossterm_right_arrow_key_event_handler)                  (uint16_t modifiers, void *token);
typedef void (*crossterm_up_arrow_key_event_handler)                     (uint16_t modifiers, void *token);
typedef void (*crossterm_down_arrow_key_event_handler)                   (uint16_t modifiers, void *token);
typedef void (*crossterm_home_key_event_handler)                         (uint16_t modifiers, void *token);
typedef void (*crossterm_end_key_event_handler)                          (uint16_t modifiers, void *token);
typedef void (*crossterm_page_up_key_event_handler)                      (uint16_t modifiers, void *token);
typedef void (*crossterm_page_down_key_event_handler)                    (uint16_t modifiers, void *token);
typedef void (*crossterm_tab_key_event_handler)                          (uint16_t modifiers, void *token);
typedef void (*crossterm_backtab_key_event_handler)                      (uint16_t modifiers, void *token);
typedef void (*crossterm_delete_key_event_handler)                       (uint16_t modifiers, void *token);
typedef void (*crossterm_insert_key_event_handler)                       (uint16_t modifiers, void *token);
typedef void (*crossterm_escape_key_event_handler)                       (uint16_t modifiers, void *token);
typedef void (*crossterm_escape_key_event_handler)                       (uint16_t modifiers, void *token);
typedef void (*crossterm_resize_event_handler)            (uint16_t rows, uint16_t cols,      void *token);

struct crossterm_event_executor {
    crossterm_character_key_event_handler   character_key_event_handler;
    void                                   *character_key_event_handler_token;
    crossterm_f_key_event_handler           f_key_event_handler;
    void                                   *f_key_event_handler_token;
    crossterm_backtab_key_event_handler     backspace_key_event_handler;
    void                                   *backspace_key_event_handler_token;
    crossterm_enter_key_event_handler       enter_key_event_handler;
    void                                   *enter_key_event_handler_token;
    crossterm_left_arrow_key_event_handler  left_arrow_key_event_handler;
    void                                   *left_arrow_key_event_handler_token;
    crossterm_right_arrow_key_event_handler right_arrow_key_event_handler;
    void                                   *right_arrow_key_event_handler_token;
    crossterm_up_arrow_key_event_handler    up_arrow_key_event_handler;
    void                                   *up_arrow_key_event_handler_token;
    crossterm_down_arrow_key_event_handler  down_arrow_key_event_handler;
    void                                   *down_arrow_key_event_handler_token;
    crossterm_home_key_event_handler        home_key_event_handler;
    void                                   *home_key_event_handler_token;
    crossterm_end_key_event_handler         end_key_event_handler;
    void                                   *end_key_event_handler_token;
    crossterm_page_up_key_event_handler     page_up_key_event_handler;
    void                                   *page_up_key_event_handler_token;
    crossterm_page_down_key_event_handler   page_down_key_event_handler;
    void                                   *page_down_key_event_handler_token;
    crossterm_tab_key_event_handler         tab_key_event_handler;
    void                                   *tab_key_event_handler_token;
    crossterm_backtab_key_event_handler     backtab_key_event_handler;
    void                                   *backtab_key_event_handler_token;
    crossterm_delete_key_event_handler      delete_key_event_handler;
    void                                   *delete_key_event_handler_token;
    crossterm_insert_key_event_handler      insert_key_event_handler;
    void                                   *insert_key_event_handler_token;
    crossterm_escape_key_event_handler      escape_key_event_handler;
    void                                   *escape_key_event_handler_token;
    crossterm_resize_event_handler          resize_event_handler;
    void                                   *resize_event_handler_token;
};

void crossterm_event_executor_init(struct crossterm_event_executor *event_executor);
int  crossterm_event_execute(struct crossterm_event_executor *event_executor);
int  crossterm_event_poll(int *is_available);

// clang-format on
#endif
