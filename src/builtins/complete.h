// Prototypes for functions for executing builtin_complete functions.
#ifndef FISH_BUILTIN_COMPLETE_H
#define FISH_BUILTIN_COMPLETE_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_complete(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_complete_ffi(void *parser, void *streams, const void *argv);
#endif
