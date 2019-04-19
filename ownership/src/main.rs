fn main() {
    let _s1 = gives_ownership();
    let s2 = String::from("Hello");
    let _s3 = takes_and_gives_back(s2);
} // here, drop will only be called on _s1 and _s3 as s2 ownership was moved

fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
