//@ revisions: old new
//@[old] edition:2015
//@[new] edition:2021
//@[new] run-rustfix
#![deny(bare_trait_objects)]
fn ord_prefer_dot(s: String) -> Ord {
    //[new]~^ ERROR expected a type, found a trait
    //[old]~^^ ERROR the trait `Ord` is not dyn compatible
    //[old]~| ERROR trait objects without an explicit `dyn` are deprecated
    //[old]~| WARNING this is accepted in the current edition (Rust 2015)
    (s.starts_with("."), s)
}
fn main() {
    let _ = ord_prefer_dot(String::new());
}
