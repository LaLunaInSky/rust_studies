pub fn aggregator;

use crate::aggregator::SocialPost;

pub trait Summary {
    fn summarize(
        &self
    ) -> String;
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
