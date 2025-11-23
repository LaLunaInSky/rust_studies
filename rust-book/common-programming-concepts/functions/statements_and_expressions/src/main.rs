fn main() {
    let y = {
        let x = 3;

        x + 1 // return block, without semicolon.
    };

    println!(
      "The value of y is: {y}"  
    );
}
