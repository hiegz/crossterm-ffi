#include <crossterm_ffi/stream.h>

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    (void)argc, (void)argv;
    fprintf(stdout, "[+--+] Testing stream allocation/deallocation functions ... ");
    fflush(stdout);
    struct crossterm_stream *stream = crossterm_stream_new(stdout);
    if (stream == NULL) {
        fprintf(stdout, "FAILURE\n");
        fflush(stdout);
        return 1;
    }
    crossterm_stream_free(stream);
    fprintf(stdout, "ok\n");
    fflush(stdout);
    return 0;
}
