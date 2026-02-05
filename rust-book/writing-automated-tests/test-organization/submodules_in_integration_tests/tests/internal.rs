use submodules_in_integration_tests::add;

mod common;

#[test]
fn it_works() {
    common::setup();

    let result = add(2, 2);
    
    assert_eq!(result, 4);
}