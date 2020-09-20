pub trait Extension<T> {
    fn get(&self) -> Option<&T>;
    fn set(&mut self, val: T);
}
