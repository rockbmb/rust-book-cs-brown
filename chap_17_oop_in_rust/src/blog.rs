pub struct Post {
    ///
    /// | Because the state field of Post is private, there is no way to create a
    /// | Post in any other state! In the Post::new function, we set the content
    /// | field to a new, empty String.
    ///
    state: Option<Box<dyn State>>,
    pub content: String,
}

pub struct PostE {
    state : PostState,
    /// This field was made public so that tests to text addition while in
    /// Draft states and otherwise could be written:
    /// * if a post is not `Draft`, it will use the default implementation of
    ///   `State`'s `content` method, which is always "".
    pub content : String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.is_draft() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        // |
        // | We want the value returned from content to depend on the current
        // | state of the Post, so we’re going to have the Post delegate to a
        // | content method defined on its state
        // |
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // The reason the state field is wrapped in an `Option`:
        //
        // |
        // | To consume the old state, the request_review method needs to take
        // | ownership of the state value.
        // |
        // | This is where the Option in the state field of Post comes in: we call
        // | the take method to take the Some value out of the state field and leave
        // | a None in its place, because Rust doesn’t let us have unpopulated fields in structs.
        // |
        // | This lets us move the state value out of Post rather than borrowing it.
        // | Then we’ll set the post’s state value to the result of this operation.
        // |
        // | We need to set state to None temporarily rather than setting it
        // | directly with code like self.state = self.state.request_review(); to
        // | get ownership of the state value. This ensures Post can’t use the old
        // | state value after we’ve transformed it into a new state.
        // |
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn is_draft(&self) -> bool {
        self.state.as_ref().unwrap().is_draft()
    }

    pub fn is_pending(&self) -> bool {
        self.state.as_ref().unwrap().is_pending()
    }

    pub fn is_scheduled(&self) -> bool {
        self.state.as_ref().unwrap().is_scheduled()
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn is_draft(self: &Self) -> bool {
        false
    }
    fn is_pending(self: &Self) -> bool {
        false
    }
    fn is_scheduled(self: &Self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_draft(self: &Self) -> bool {
        true
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Scheduled {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn is_pending(self: &Self) -> bool {
        true 
    }
}

#[derive(Debug)]
struct Scheduled {}

impl State for Scheduled {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn is_scheduled(self: &Self) -> bool {
        true
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//
//
//

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