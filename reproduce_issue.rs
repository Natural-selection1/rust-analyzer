
trait Foo<A> {
    fn bar<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
struct MyStruct<T> {
    value: T,
}
impl<T> Foo<T> for MyStruct<T> {
    fn bar<T: IntoIterator<Item = T>>(iter: T) -> Self {
        todo!()
    }
}
fn main() {}
