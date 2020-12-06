#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

pub mod ast;

// Testing modules
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests;
