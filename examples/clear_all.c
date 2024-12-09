#include <crossterm_ffi/error.h>
#include <crossterm_ffi/stream.h>
#include <crossterm_ffi/terminal.h>

#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    (void)argc, (void)argv;
    int rt;
    struct crossterm_stream *stream = crossterm_stream_new(stdout);
    if (NULL == stream) {
        fprintf(stderr, "Couldn't allocate crossterm stream : %s\n",
                strerror(errno));
        return 1;
    }
    rt = crossterm_clear_all(stream);
    if (0 != rt) {
        fprintf(stderr, "Unable to clear all cells : %s\n",
                crossterm_strerror(-rt));
        crossterm_stream_free(stream);
        return 1;
    }
    rt = crossterm_stream_flush(stream);
    assert(0 == rt);
    crossterm_stream_free(stream);
    return 0;
}
