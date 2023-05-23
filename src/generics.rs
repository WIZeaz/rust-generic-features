use crate::types::{T1,T2};

pub fn generic1<T:T1<i32>+T2>(t:T){

}