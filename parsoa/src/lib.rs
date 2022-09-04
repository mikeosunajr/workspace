extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "clinch.pest"]
pub struct CSVParser;

pub fn parse(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    CSVParser::parse(Rule::main, input)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let successful_parse = CSVParser::parse(Rule::main, "-- test\ntest");
        println!("{:?}", successful_parse);

        let unsuccessful_parse = CSVParser::parse(Rule::main, "this is not a number");
        println!("{:?}", unsuccessful_parse);
    }
}
