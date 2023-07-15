#ifndef FISH_KILL_H
#define FISH_KILL_H

#include "common.h"
#if INCLUDE_RUST_HEADERS
#include "kill.rs.h"
#endif

/// Rotate the killring.
wcstring kill_yank_rotate();

/// Paste from the killring.
wcstring kill_yank();

/// Get copy of kill ring as vector of strings
std::vector<wcstring> kill_entries();

/// Rust-friendly kill entries.
wcstring_list_ffi_t kill_entries_ffi();

#endif
