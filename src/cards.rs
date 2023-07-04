struct Simple<T>(String, T);
impl<T> KadeuFactory for Simple<T> {
    type BACK = T;
}

struct Multi<T>(String, Vec<T>);
impl<T> KadeuFactory for Multi<T> {
    type BACK = Vec<T>;
}
