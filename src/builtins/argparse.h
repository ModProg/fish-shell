// Prototypes for executing builtin_getopt function.
#ifndef FISH_BUILTIN_ARGPARSE_H
#define FISH_BUILTIN_ARGPARSE_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_argparse(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_argparse_ffi(void *parser, void *streams, const void *argv);

#endif
