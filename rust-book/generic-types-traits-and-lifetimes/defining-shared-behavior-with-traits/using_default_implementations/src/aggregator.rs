use crate::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(
        &self
    ) -> String {
        format!(
            "@{}",
            self.author
        )
    }
}

impl Summary for SocialPost {
    fn summarize_author(
        &self
    ) -> String {
        format!(
            "@{}",
            self.username
        )
    }
}