// Prototypes for functions for executing builtin_commandline functions.
#ifndef FISH_BUILTIN_COMMANDLINE_H
#define FISH_BUILTIN_COMMANDLINE_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_commandline(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_commandline_ffi(void *parser, void *streams, const void *argv);
#endif
