pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool
}

impl Summary for NewArticle {
    pub fn summarize(
        &self
    ) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

impl Summary for SocialPost {
    pub fn summarize(
        &self
    ) -> String {
        format!(
            "{}: {}",
            self.username,
            self.content
        )
    }
}