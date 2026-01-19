fn main() {
    println!(
        "\nChapter defining-shared-behavior-with-traits/clearer_trait_bounds_whith_where_clauses\n"
    );

    // fn some_function<T: Display + Clone, U: Clone + Debug>(
    //     t: &T,
    //     u: &U 
    // ) -> i32 {}

    // Better
    fn some_function<T, U>(
        t: &T, 
        u: &U
    ) -> i32 
    where
        T: Display + Clone,
        U: Clone + Debug,
    {}
}
