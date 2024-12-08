#include <crossterm_ffi/error.h>
#include <crossterm_ffi/stream.h>
#include <crossterm_ffi/terminal.h>

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
        int eos = errno;
        fprintf(stdout, "Unable to enter alternate screen : ");
        if (-rt == CROSSTERM_EOS) {
            fprintf(stdout, "%s\n", strerror(eos));
        } else {
            fprintf(stdout, "undefined error\n");
        }
        crossterm_stream_free(stream);
        return 1;
    }
    sleep(1);
    rt = crossterm_leave_alternate_screen(stream);
    if (0 != rt) {
        int eos = errno;
        fprintf(stdout, "Unable to leave alternate screen : ");
        if (-rt == CROSSTERM_EOS) {
            fprintf(stdout, "%s\n", strerror(eos));
        } else {
            fprintf(stdout, "undefined error\n");
        }
        crossterm_stream_free(stream);
        return 1;
    }
    crossterm_stream_free(stream);
    return 0;
}
