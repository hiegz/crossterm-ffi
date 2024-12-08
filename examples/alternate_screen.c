#include <crossterm_ffi/error.h>
#include <crossterm_ffi/stream.h>
#include <crossterm_ffi/terminal.h>

#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    (void)argc, (void)argv;
    int rt;
    struct crossterm_stream *stream = crossterm_stream_new(stdout);
    if (stream == NULL) {
        fprintf(stderr, "Couldn't allocate crossterm stream : %s\n",
                strerror(errno));
        return 1;
    }
    rt = crossterm_enter_alternate_screen(stream);
    if (0 != rt) {
        fprintf(stderr, "Unable to enter alternate screen : %s\n",
                crossterm_strerror(-rt));
        crossterm_stream_free(stream);
        return 1;
    }
    rt = crossterm_stream_flush(stream);
    assert(0 == rt);
    sleep(1);
    rt = crossterm_leave_alternate_screen(stream);
    if (0 != rt) {
        fprintf(stderr, "Unable to leave alternate screen : %s\n",
                crossterm_strerror(-rt));
        crossterm_stream_free(stream);
        return 1;
    }
    rt = crossterm_stream_flush(stream);
    assert(0 == rt);
    crossterm_stream_free(stream);
    return 0;
}
