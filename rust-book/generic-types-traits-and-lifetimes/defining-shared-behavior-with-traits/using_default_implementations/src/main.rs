use using_default_implementations::{
    Summary,
    aggregator::{
        NewsArticle,
        SocialPost,
    },
};

fn main() {
    println!(
        "\nChapter defining-shared-behavior-with-traits/using_default_implementations\n"
    );

    let article_01 = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburg PEnguins once again are the best \
            hockey team in the NHL."
        ),
    };

    println!(
        "New article available! {}",
        article_01.summarize()
    );

    let post_01 = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };

    println!(
        "1 new post: {}",
        post_01.summarize()
    );
}