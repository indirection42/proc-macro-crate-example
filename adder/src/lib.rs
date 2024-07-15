pub mod submod {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

pub use adder_proc_macro::adder_macro;

#[test]
fn it_works() {
    let result = adder_macro!((2, 2));
    assert_eq!(result, 4);
}
