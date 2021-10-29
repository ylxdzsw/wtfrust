wtfrust
=======

Collection of "surprising behaviour" in Rust. See also [wtfpython] and [wtfjs].

[wtfpython]: https://github.com/satwikkansal/wtfpython
[wtfjs]: https://github.com/denysdovhan/wtfjs

This document avoids common and general problems in programming like float arithmetics, string encodings, etc. and focus
on the behaviours specific to Rust.

Luckily, most of the examples either require `unsafe` or get warnings from the compiler or clippy.

## Mutable Const

```rust
use std::sync::atomic::{AtomicU8, Ordering::Relaxed};

const a: AtomicU8 = AtomicU8::new(1);

fn main() {
    println!("{:?}", a);
    a.store(2, Relaxed);
    println!("{:?}", a);
}
```

**Result**: Print 1 twice.

**Explanation**: `const` in Rust is more like a macro replacement than a variable. It copies the value to wherever it is
used as stack variables or immediate values.

**Solution**: 1). Use `static` for types with interior mutability, and use `const` for real constants. 2). `cargo clippy`.

**References**: [the book].

[the book]: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants

## Reading uninitialized memory

> example taken from [https://users.rust-lang.org/t/is-it-ub-to-read-uninitialized-integers-in-a-vector-why/39682/8]()

```rust
fn main() {
    let a: u32 = unsafe {
        std::mem::uninitialized()
    };

    if a < 150 {
        println!("small");
    }

    if a > 100 {
        println!("big");
    }
}
```

**Result**: It may print "small", or "big", or, in release mode, *nothing*.

**Explanation**: Reading uninitialized memory is always undefined behaviour, even for integers, and the compiler applys
optimizations with the assumption that all undefined behaviour never happen.

**Solution**: Don't do it. Unless you are making a hacker tool, reading uninitialized memory is simply wrong. Note that
dropping uninitialized memory is equally bad because it may read it during dropping. Therefore, one should always use
`ptr::write` to write to uninitialized memory, as the move operator (`=`) implicitly drops the old value. Also use
`MaybeUninit` instead of `std::mem::uninitialized` to protect from implicit dropping during panicking.

**References**: [doc of `std::hint::unreachable_unchecked`] states "In particular, the compiler assumes that all UB must
never happen, and therefore will eliminate all branches that reach to ...".

[doc of `std::hint::unreachable_unchecked`]: https://doc.rust-lang.org/std/hint/fn.unreachable_unchecked.html#safety

## Cloning a reference

```rust
fn main() {
    let a = std::cell::RefCell::new(1);
    *a.clone().borrow_mut() += 1;
    println!("a = {:?}", a);
    
    let b = &a;
    *b.clone().borrow_mut() += 1;
    println!("b = {:?}", b);
    
    let c = &b;
    *c.clone().borrow_mut() += 1;
    println!("c = {:?}", c);
}
```

**Result**: `a = 1, b = 1, c = 2`.

**Explanation**: Rust automatically adds as much of `&` and `*` as necessary when using the "dot" method call. This
behaviour becomes confusing when a reference type or smart pointer implements the same method. In this example, both
`RefCell` and `&RefCell` implement `Clone`. Therefore, `c.clone()` resolves to `<&_ as Clone>::clone`, while for `a` and
`b` it is `<RefCell as Clone>::clone`.

**Solution**: 1). Use fully qualified function call syntax `<type as trait>::function()` when there are potential
ambiguity. 2). `cargo clippy`.

**References**: [the book].

[the book]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name





TODO: moving to `_` is not an actual moving, rather it drops immediately.
