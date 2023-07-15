use crate::io::IoStreams;
use crate::parser::Parser;
use crate::wchar;
use crate::wchar_ffi::WCharToFFI;
#[rustfmt::skip]
use ::std::pin::Pin;
#[rustfmt::skip]
use ::std::slice;
use crate::env::{EnvMode, EnvStackRef, EnvStackRefFFI};
pub use crate::wait_handle::{WaitHandleRef, WaitHandleStore};
use crate::wchar::{wstr, WString};
use crate::wchar_ffi::WCharFromFFI;
use autocxx::prelude::*;
use cxx::SharedPtr;
use libc::pid_t;

// autocxx has been hacked up to know about this.
pub type wchar_t = u32;

include_cpp! {
    #include "autoload.h"
    #include "color.h"
    #include "common.h"
    #include "complete.h"
    #include "env.h"
    #include "env_universal_common.h"
    #include "env_dispatch.h"
    #include "event.h"
    #include "exec.h"
    #include "fallback.h"
    #include "fds.h"
    #include "fish_indent_common.h"
    #include "flog.h"
    #include "function.h"
    #include "highlight.h"
    #include "history.h"
    #include "io.h"
    #include "input_common.h"
    #include "kill.h"
    #include "parse_constants.h"
    #include "parser.h"
    #include "parse_util.h"
    #include "path.h"
    #include "proc.h"
    #include "reader.h"
    #include "screen.h"
    #include "tokenizer.h"
    #include "wildcard.h"
    #include "wutil.h"

    #include "builtins/argparse.h"
    #include "builtins/bind.h"
    #include "builtins/cd.h"
    #include "builtins/commandline.h"
    #include "builtins/complete.h"
    #include "builtins/eval.h"
    #include "builtins/fg.h"
    #include "builtins/functions.h"
    #include "builtins/history.h"
    #include "builtins/path.h"
    #include "builtins/read.h"
    #include "builtins/set.h"
    #include "builtins/set_color.h"
    #include "builtins/source.h"
    #include "builtins/status.h"
    #include "builtins/string.h"
    #include "builtins/test.h"
    #include "builtins/ulimit.h"

    safety!(unsafe_ffi)

    generate_pod!("wcharz_t")
    generate!("wcstring_list_ffi_t")
    generate!("make_wcharz_vec")
    generate!("make_fd_nonblocking")
    generate!("wperror")

    generate_pod!("pipes_ffi_t")
    generate!("env_dispatch_var_change_ffi")

    generate!("make_pipes_ffi")

    generate!("log_extra_to_flog_file")

    generate!("wgettext_ptr")

    generate!("highlighter_t")

    generate!("pretty_printer_t")

    generate!("fd_event_signaller_t")

    generate!("rgb_color_t")
    generate_pod!("color24_t")
    generate!("reader_status_count")
    generate!("reader_read_ffi")
    generate!("kill_entries_ffi")

    generate!("get_history_variable_text_ffi")

    generate_pod!("escape_string_style_t")


    generate!("screen_set_midnight_commander_hack")
    generate!("screen_clear_layout_cache_ffi")
    generate!("reader_schedule_prompt_repaint")
    generate!("reader_change_history")
    generate!("reader_change_cursor_selection_mode")
    generate!("reader_set_autosuggestion_enabled_ffi")

    generate!("builtin_argparse_ffi")
    generate!("builtin_bind_ffi")
    generate!("builtin_cd_ffi")
    generate!("builtin_commandline_ffi")
    generate!("builtin_complete_ffi")
    generate!("builtin_eval_ffi")
    generate!("builtin_fg_ffi")
    generate!("builtin_functions_ffi")
    generate!("builtin_history_ffi")
    generate!("builtin_path_ffi")
    generate!("builtin_read_ffi")
    generate!("builtin_set_ffi")
    generate!("builtin_set_color_ffi")
    generate!("builtin_source_ffi")
    generate!("builtin_status_ffi")
    generate!("builtin_string_ffi")
    generate!("builtin_test_ffi")
    generate!("builtin_ulimit_ffi")
}

/// Allow wcharz_t to be "into" wstr.
impl From<wcharz_t> for &wchar::wstr {
    fn from(w: wcharz_t) -> Self {
        let len = w.length();
        let v = unsafe { slice::from_raw_parts(w.str_ as *const u32, len) };
        wchar::wstr::from_slice(v).expect("Invalid UTF-32")
    }
}

/// Allow wcharz_t to be "into" WString.
impl From<wcharz_t> for wchar::WString {
    fn from(w: wcharz_t) -> Self {
        let len = w.length();
        let v = unsafe { slice::from_raw_parts(w.str_ as *const u32, len).to_vec() };
        Self::from_vec(v).expect("Invalid UTF-32")
    }
}

/// Allow wcstring_list_ffi_t to be "into" Vec<WString>.
impl From<&wcstring_list_ffi_t> for Vec<wchar::WString> {
    fn from(w: &wcstring_list_ffi_t) -> Self {
        let mut result = Vec::with_capacity(w.size());
        for i in 0..w.size() {
            result.push(w.at(i).from_ffi());
        }
        result
    }
}

/// A bogus trait for turning &mut Foo into Pin<&mut Foo>.
/// autocxx enforces that non-const methods must be called through Pin,
/// but this means we can't pass around mutable references to types like Parser.
/// We also don't want to assert that Parser is Unpin.
/// So we just allow constructing a pin from a mutable reference; none of the C++ code.
/// It's worth considering disabling this in cxx; for now we use this trait.
/// Eventually Parser and IoStreams will not require Pin so we just unsafe-it away.
pub trait Repin {
    fn pin(&mut self) -> Pin<&mut Self> {
        unsafe { Pin::new_unchecked(self) }
    }

    fn unpin(self: Pin<&mut Self>) -> &mut Self {
        unsafe { self.get_unchecked_mut() }
    }
}

// Implement Repin for our types.
impl Repin for IoStreams<'_> {}
impl Repin for Parser {}
impl Repin for wcstring_list_ffi_t {}

pub use autocxx::c_int;
pub use ffi::*;
pub use libc::c_char;

/// A version of [`* const core::ffi::c_void`] (or [`* const libc::c_void`], if you prefer) that
/// implements `Copy` and `Clone`, because those two don't. Used to represent a `void *` ptr for ffi
/// purposes.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct void_ptr(pub *const core::ffi::c_void);

impl core::fmt::Debug for void_ptr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:p}", &self.0)
    }
}

unsafe impl Send for void_ptr {}
unsafe impl Sync for void_ptr {}

impl core::convert::From<*const core::ffi::c_void> for void_ptr {
    fn from(value: *const core::ffi::c_void) -> Self {
        Self(value as *const _)
    }
}

impl core::convert::From<*const u8> for void_ptr {
    fn from(value: *const u8) -> Self {
        Self(value as *const _)
    }
}

impl core::convert::From<*const autocxx::c_void> for void_ptr {
    fn from(value: *const autocxx::c_void) -> Self {
        Self(value as *const _)
    }
}

impl core::convert::From<void_ptr> for *const u8 {
    fn from(value: void_ptr) -> Self {
        value.0 as *const _
    }
}

impl core::convert::From<void_ptr> for *const core::ffi::c_void {
    fn from(value: void_ptr) -> Self {
        value.0 as *const _
    }
}

impl core::convert::From<void_ptr> for *const autocxx::c_void {
    fn from(value: void_ptr) -> Self {
        value.0 as *const _
    }
}
