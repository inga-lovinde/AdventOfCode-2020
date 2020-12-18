use std::io::{self, BufRead};

#[macro_use]
extern crate nom;
extern crate nom_peg;
use nom_peg::grammar;

fn main() {
    let arithmetic = grammar! {
        parse: i64 = <expr> "="

        expr: i64 = <l: sum> (" ")* "*" (" ")* <r: expr> => { l * r }
                  | sum
    
        sum: i64 = <l: value> (" ")* "+" (" ")* <r: sum> => { l + r }
                     | value
    
        value: i64 = ("0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9")+ => { result.join("").parse::<i64>().unwrap() }
                   | "(" <expr> ")"
    };

    let mut result = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let value = arithmetic.parse(&(line.unwrap() + "=")).unwrap().1;
        println!("{}", value);
        result += value;
    }

    println!("{}", result)
}
