use crate::types::{T1, T2, T3};

pub fn generic1<T:T1<i32>+T2>(t:T){

}

pub fn generic_bounds_1<T:T2+T3>(t:T){}
pub fn generic_bounds_2(t:impl T2+T3){}
pub fn generic_bounds_3<T>(t:T) where T:T2+T3{}
