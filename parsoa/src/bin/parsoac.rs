use parsoa::parse;

fn main() {
    let successful_parse = parse("-- test\ntest");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = parse("this is not a number");
    println!("{:?}", unsuccessful_parse);
}
