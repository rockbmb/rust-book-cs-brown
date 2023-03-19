pub struct PostE {
    state : PostState,
    /// This field was made public so that tests to text addition while in
    /// Draft states and otherwise could be written:
    /// * if a post is not `Draft`, it will use the default implementation of
    ///   `State`'s `content` method, which is always "".
    pub content : String,
}

#[derive(PartialEq)]
pub enum PostState {
    Draft,
    PendingReview,
    Scheduled,
    Published
}

impl PostState {
    pub fn is_draft(&self) -> bool {
        match self {
            PostState::Draft => true,
            _ => false,
        }
    }

    pub fn is_pending(&self) -> bool {
        match self {
            PostState::PendingReview => true,
            _ => false,
        }
    }

    pub fn is_scheduled(&self) -> bool {
        match self {
            PostState::Scheduled => true,
            _ => false,
        }
    }

    pub fn is_published(&self) -> bool {
        match self {
            PostState::Published => true,
            _ => false,
        }
    }

    pub fn request_review(&self) -> Self {
        match self {
            PostState::Draft => PostState::PendingReview,
            PostState::PendingReview => PostState::PendingReview,
            PostState::Scheduled => PostState::Scheduled,
            PostState::Published => PostState::Published,
        }
    }

    pub fn approve(&self) -> Self {
        match self {
            PostState::Draft => PostState::Draft,
            PostState::PendingReview => PostState::Scheduled,
            PostState::Scheduled => PostState::Published,
            PostState::Published => PostState::Published,
        }
    }

    pub fn reject(&self) -> Self {
        match self {
            PostState::Draft => PostState::Draft,
            PostState::PendingReview => PostState::Draft,
            PostState::Scheduled => PostState::PendingReview,
            PostState::Published => PostState::Published,
        }
    }
}

impl PostE {
    pub fn new() -> PostE {
        PostE {
            state: PostState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.is_draft() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        if self.state.is_published() {
            return &self.content;
        } else {
            return ""
        }
    }

    pub fn request_review(&mut self) {
        self.state = self.state.request_review();
    }

    pub fn approve(&mut self) {
        self.state = self.state.approve();
    }

    pub fn reject(&mut self) {
        self.state = self.state.reject();
    }

    pub fn is_draft(&self) -> bool {
        self.state.is_draft()
    }

    pub fn is_pending(&self) -> bool {
        self.state.is_pending()
    }

    pub fn is_scheduled(&self) -> bool {
        self.state.is_scheduled()
    }
}