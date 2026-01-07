fn main() {
    // Example 01
    let mut s = String::from("foo");
    s.push_str("bar");

    println!(
        "{s} -- Example 01"
    );

    // Example 02
    let mut s_01 = String::from("foo");
    let s_02 = "bar";
    s_01.push_str(s_02);

    println!(
        "{s_01} -- Example 02\ns_02 is {s_02} -- Example 02"
    );

    // Example 03
    let mut s = String::from("lo");
    s.push('l');

    println!(
        "{s} -- Example 03"
    );
}
