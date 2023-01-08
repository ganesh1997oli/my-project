use crate::traits::base::Base;

impl dyn Base {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}