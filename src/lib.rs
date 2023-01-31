#[ctor::ctor]
fn ctor() {
    println!("Hi from library");
}
