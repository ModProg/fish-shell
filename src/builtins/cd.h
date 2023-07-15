// Prototypes for executing builtin_cd function.
#ifndef FISH_BUILTIN_CD_H
#define FISH_BUILTIN_CD_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_cd(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_cd_ffi(void *parser, void *streams, const void *argv);
#endif
