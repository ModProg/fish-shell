// Prototypes for executing builtin_source function.
#ifndef FISH_BUILTIN_SOURCE_H
#define FISH_BUILTIN_SOURCE_H

#include "../maybe.h"
#include "../parser.h"
#include "cxx.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_source(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_source_ffi(void *parser, void *streams, const void *argv);

#endif
