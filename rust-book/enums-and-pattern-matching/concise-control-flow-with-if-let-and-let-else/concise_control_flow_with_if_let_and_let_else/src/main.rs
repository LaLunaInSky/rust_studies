fn main() {
    println!("\nchapter concise-control-flow-with-if-let-and-let-else/concise_control_flow_with_if_let_and_let_else\n");

    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!(
    //         "The maximum is configured to be {max}"
    //     ),
    //     _ => (),
    // } 

    if let Some(max) = config_max {
        println!(
            "The maximum is configured to be {max}"
        );
    }   
}
