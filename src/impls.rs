use crate::types::{self, T1, T2, T3, S};

impl<T> types::S<T> {
    pub fn foo() {}
}

impl<T, U> T1<T> for types::S<U> {
    fn t1(self, t: T) {}
}

impl<T> T2 for T{
    fn t2() {}
}

impl<T> T3 for S<T>{
    fn t3() {}
}

impl<T> AsRef<str> for S<T>{
    fn as_ref(&self) -> &str {
        "hello"
    }
}