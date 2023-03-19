use chap_17_oop_in_rust::blog_oop_st_obj::{self as blog_oop_st_obj};
use chap_17_oop_in_rust::blog_enum::{self as blog_enum};
use chap_17_oop_in_rust::blog_idiomatic_rust::{self as blog_idiomatic_rust};
use blog_oop_st_obj::Post;
use blog_enum::PostE;
use blog_idiomatic_rust::*;

fn main() {
    let mut post = Post::new();
    let mut post_e = PostE::new();
    let mut draft_post = DraftPost::new();

    let str = "I ate a salad for lunch today";
    let next = ". It was very good.";

    post.add_text(str);
    post_e.add_text(str);
    draft_post.add_text(str);
    assert_eq!("", post.content());
    assert_eq!("", post_e.content());
    // Not possible, Rust's typesystem correctfully prevents this
    // as intented by the post workflow that is required.
    //assert_eq!("", draft_post.content());
    assert_eq!(str, post.content);
    assert_eq!(str, post_e.content);
    assert_eq!(str, draft_post.content);

    post.request_review();
    post_e.request_review();
    let pending_post = draft_post.request_review();
    assert_eq!("", post.content());
    assert_eq!("", post_e.content());
    // Again, not possible
    //assert_eq!("", pending_post.content());
    post.add_text(next);
    post_e.add_text(next);
    //Not possible
    //pending_post.add_text(next);
    assert_eq!(str, post.content);
    assert_eq!(str, post_e.content);

    post.reject();
    post_e.reject();
    let draft_post = pending_post.reject();
    assert!(post.is_draft());
    post.request_review();
    assert!(post_e.is_draft());
    post_e.request_review();
    // Because of the type system, it's clear that calling `reject` on a
    // pending post will result in another draft post - further assertions unnecessary.
    let pending_post = draft_post.request_review();

    assert!(post.is_pending() && !post.is_scheduled());
    assert!(post_e.is_pending() && !post_e.is_scheduled());
    post.approve();
    post_e.approve();
    let sched_post = pending_post.approve();

    assert!(!post.is_pending() && post.is_scheduled());
    assert!(!post_e.is_pending() && post_e.is_scheduled());
    post.approve();
    post_e.approve();
    let published_post = sched_post.approve();
    // Because approval, rejection and request of a review all consume
    // the prior post, the below variables have been moved, preventing any
    // improper accesses.
    //draft_post;
    //pending_post;
    //sched_post;

    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!("I ate a salad for lunch today", post_e.content());
    assert_eq!("I ate a salad for lunch today", published_post.content());
}