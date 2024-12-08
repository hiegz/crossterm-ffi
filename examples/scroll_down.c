#include <crossterm_ffi/error.h>
#include <crossterm_ffi/stream.h>
#include <crossterm_ffi/terminal.h>

#include <assert.h>
#include <errno.h>
#include <stdio.h>
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
    fprintf(stdout, "Scrolling down by five lines in 1 second ...\n");
    sleep(1);
    rt = crossterm_scroll_down(stream, 5);
    if (0 != rt) {
        fprintf(stderr, "Unable to scroll down by five lines : %s\n",
                crossterm_strerror(-rt));
        crossterm_stream_free(stream);
        return 1;
    }
    rt = crossterm_stream_flush(stream);
    assert(0 == rt);
    fprintf(stdout, "Scrolled down by five lines (if this messed up some lines "
                    "it worked)\n");
    crossterm_stream_free(stream);
    return 0;
}
