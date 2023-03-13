/// Acceptable to `expect` as the compiler cannot understand that
/// failure here is a logical impossibility.
fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

/*
From the Rust book, chapter 9.3:

Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.
In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken,
such as when invalid values, contradictory values, or missing values are passed to your code—plus one or
more of the following:

    * The bad state is something that is unexpected, as opposed to something that will likely happen
    occasionally, like a user entering data in the wrong format.
    * Your code after this point needs to rely on not being in this bad state, rather than checking
    for the problem at every step.
    * There’s not a good way to encode this information in the types you use. 

 */