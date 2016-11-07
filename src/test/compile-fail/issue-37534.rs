// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo<T: ?Hash> { }
//~^ ERROR trait `Hash` is not in scope [E0405]
//~^^ ERROR parameter `T` is never used [E0392]
//~^^^ WARN default bound relaxed for a type parameter, but this does nothing

fn main() { }