use adder::adder_macro;

#[test]
fn integration_test() {
    adder_macro!((1, 2));
}
