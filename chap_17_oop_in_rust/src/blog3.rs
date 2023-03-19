pub struct DraftPost {
    /// Only made public for tests, as with the other types of posts.
    pub content: String,
}

impl DraftPost {
    /// We still have a Post::new function, but instead of returning an instance
    /// of PublishedPost, it returns an instance of DraftPost.
    ///
    /// Because content is private
    /// and there aren’t any functions that return any kind of `Post` other than a draft,
    /// it’s not possible to create an instance of Post right now.
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Note how this method, and all of the `approves` and `rejects`,
    /// take ownership of the argument to consume the previous instance.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> ScheduledPost {
        ScheduledPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}

pub struct ScheduledPost {
    content: String,
}

impl ScheduledPost {
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PublishedPost {
    content: String,
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}