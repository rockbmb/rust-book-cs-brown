/*
use chap_14_2_publish_crate::utils::mix;
use chap_14_2_publish_crate::kinds::PrimaryColor;
*/

use chap_14_2_publish_crate::mix;
use chap_14_2_publish_crate::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}