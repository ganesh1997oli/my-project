#[openbrush::wrapper]
pub type BaseRef = dyn Base;

#[openbrush::trait_definition]
pub trait Base {
    #[ink(message)]
    fn add(&self, a: i32, b: i32) -> i32;
}