// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod detail {
    extern {
        #[link_name = "llvm.trap"]
        pub fn abort() -> !;

        #[link_name = "llvm.debugtrap"]
        pub fn breakpoint();
    }
}

#[inline(always)]
pub fn abort() -> ! {
    unsafe { detail::abort() }
}

pub fn breakpoint() {
    unsafe { detail::breakpoint() }
}

#[inline]
#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) -> ! {
    abort()
}

#[inline]
#[lang="fail_"]
pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
    abort()
}

#[inline]
pub fn out_of_memory() -> ! {
    abort()
}

#[cfg(debug)]
#[inline(always)]
pub fn assert(b: bool) {
    if !b {
        abort()
    }
}

#[cfg(not(debug))]
#[inline(always)]
pub fn assert(_: bool) {
}
