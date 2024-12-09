#include <crossterm_ffi/cursor.h>
#include <crossterm_ffi/error.h>
#include <crossterm_ffi/stream.h>

#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    (void)argc, (void)argv;
    int rt;
    struct crossterm_stream *stream = crossterm_stream_new(stdout);
    if (NULL == stream) {
        fprintf(stderr, "Couldn't allocate crossterm stream : %s\n",
                strerror(errno));
        return 1;
    }
    rt = crossterm_show_cursor(stream);
    if (0 != rt) {
        fprintf(stderr, "Unable to show cursor : %s\n",
                crossterm_strerror(-rt));
        crossterm_stream_free(stream);
        return 1;
    }
    rt = crossterm_stream_flush(stream);
    assert(0 == rt);
    crossterm_stream_free(stream);
    return 0;
}
