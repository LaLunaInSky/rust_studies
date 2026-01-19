struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(
        &self
    ) -> i32 {
        3
    }

    fn announce_and_return_part(
        &self,
        announcement: &str
    ) -> &str {
        println!(
            "Attencion please: {announcement}"
        );

        self.part
    }
}

fn main() {
    println!(
        "\nChapter validating-references-with-lifetimes/in_method_definitions\n"
    );


}
