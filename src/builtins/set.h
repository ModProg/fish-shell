// Prototypes for functions for executing builtin_set functions.
#ifndef FISH_BUILTIN_SET_H
#define FISH_BUILTIN_SET_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;
maybe_t<int> builtin_set(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_set_ffi(void *parser, void *streams, const void *argv);
#endif
