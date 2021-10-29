fn main() {
    println!("use cargo test");
}

#[test]
fn mutable_const() {
    use std::sync::atomic::{AtomicU8, Ordering::SeqCst};

    const a: AtomicU8 = AtomicU8::new(1);

    let before = a.load(SeqCst);
    a.store(2, SeqCst);
    let after = a.load(SeqCst);
    assert!(before == after);
}

#[test]
fn clone_reference() {
    let a = std::cell::RefCell::new(1);
    *a.clone().borrow_mut() += 1;
    assert_eq!(*a.borrow(), 1);

    let b = &a;
    *b.clone().borrow_mut() += 1;
    assert_eq!(*b.borrow(), 1);
    
    let c = &b;
    *c.clone().borrow_mut() += 1;
    assert_eq!(*c.borrow(), 2);
}
