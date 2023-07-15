// Prototypes for executing builtin_read function.
#ifndef FISH_BUILTIN_READ_H
#define FISH_BUILTIN_READ_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_read(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_read_ffi(void *parser, void *streams, const void *argv);
#endif
