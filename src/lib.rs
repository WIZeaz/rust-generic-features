pub mod impls;
pub mod types;
pub mod visibility;
pub mod generics;

fn foo(){
    visibility::Visible::visible_fn();
}