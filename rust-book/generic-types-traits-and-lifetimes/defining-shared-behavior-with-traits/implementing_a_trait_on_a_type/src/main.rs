use implementing_a_trait_on_a_type::{
    Summary,
    aggregator::SocialPost
};

fn main() {
    println!(
        "\nChapter defining-shared-behavior-with-traits/implementing_a_trait_on_a_type\n"
    );

    let post  = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of coures, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };

    println!(
        "1 new post: {}",
        post.summarize()
    );
}
