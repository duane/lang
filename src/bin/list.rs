extern crate lang;

use lang::languages;
use lang::scripts;

fn main() {
  for language in ::languages() {
    println!("{:?}", language);
  }

  for script in ::scripts() {
    println!("{:?}", script);
  }
}