// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: use of moved value

use std::sync::Arc;
use std::task;

fn main() {
    let v = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let arc_v = Arc::new(v);

    task::spawn(proc() {
        assert_eq!(*arc_v.get(3), 4);
    });

    assert_eq!(*arc_v.get(2), 3);

    println!("{}", *arc_v);
}
