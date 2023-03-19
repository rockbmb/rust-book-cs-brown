use chap_17_oop_in_rust::blog::{self as blog, PostE};
use blog::Post;

fn main() {
    let mut post = Post::new();
    let mut postE = PostE::new();

    post.add_text("I ate a salad for lunch today");
    postE.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    assert_eq!("", postE.content());

    post.request_review();
    postE.request_review();
    assert_eq!("", post.content());
    assert_eq!("", postE.content());

    post.reject();
    postE.reject();
    assert!(post.is_draft()); post.request_review();
    assert!(postE.is_draft()); postE.request_review();

    assert!(post.is_pending() && !post.is_scheduled());
    assert!(postE.is_pending() && !postE.is_scheduled());
    post.approve();
    postE.approve();

    assert!(!post.is_pending() && post.is_scheduled());
    assert!(!postE.is_pending() && postE.is_scheduled());
    post.approve();
    postE.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!("I ate a salad for lunch today", postE.content());
}