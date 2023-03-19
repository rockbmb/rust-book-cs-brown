use chap_17_oop_in_rust::blog::{self as blog, PostE};
use blog::Post;

fn main() {
    let mut post = Post::new();
    let mut post_e = PostE::new();

    let str = "I ate a salad for lunch today";
    let next = ". It was very good.";

    post.add_text(str);
    post_e.add_text(str);
    assert_eq!("", post.content());
    assert_eq!("", post_e.content());
    assert_eq!(str, post.content);
    assert_eq!(str, post_e.content);

    post.request_review();
    post_e.request_review();
    assert_eq!("", post.content());
    assert_eq!("", post_e.content());
    post.add_text(next);
    post_e.add_text(next);
    assert_eq!(str, post.content);
    assert_eq!(str, post_e.content);

    post.reject();
    post_e.reject();
    assert!(post.is_draft()); post.request_review();
    assert!(post_e.is_draft()); post_e.request_review();

    assert!(post.is_pending() && !post.is_scheduled());
    assert!(post_e.is_pending() && !post_e.is_scheduled());
    post.approve();
    post_e.approve();

    assert!(!post.is_pending() && post.is_scheduled());
    assert!(!post_e.is_pending() && post_e.is_scheduled());
    post.approve();
    post_e.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!("I ate a salad for lunch today", post_e.content());
}