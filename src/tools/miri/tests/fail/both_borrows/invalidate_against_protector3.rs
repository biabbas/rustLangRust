//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows

use std::alloc::{Layout, alloc};

fn inner(x: *mut i32, _y: &i32) {
    // If `x` and `y` alias, retagging is fine with this... but we really
    // shouldn't be allowed to write to `x` at all because `y` was assumed to be
    // immutable for the duration of this call.
    unsafe { *x = 0 };
    //~[stack]^ ERROR: protect
    //~[tree]| ERROR: /write access through .* is forbidden/
}

fn main() {
    unsafe {
        let ptr = alloc(Layout::for_value(&0i32)) as *mut i32;
        inner(ptr, &*ptr);
    };
}
