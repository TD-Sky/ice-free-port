fn sample_token() -> String {
    crate::gen(0, "Fool".to_owned())
}

#[test]
fn gen() {
    println!("token: {}", sample_token());
}

#[test]
fn parse() {
    let _ = crate::parse(&sample_token()).unwrap();
}
