fn main() {
    
    struct W(i32);

    let w = W(123);
    let a: &dyn Sync = &w;
    let b: &dyn Sync = &w.0;

    println!("{:?}", std::ptr::eq(a,b));

}
