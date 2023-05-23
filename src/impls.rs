use crate::types::{self, T1, T2};

impl<T> types::S<T> {
    pub fn foo() {}
}

impl<T, U> T1<T> for types::S<U> {
    fn t1(self, t: T) {}
}

impl<T:Default> T1<T> for T{
    fn t1(self,t:T) {
        
    }
}