use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?} -> {:?})", self.start, self.end)
    }
}

#[derive(Clone, PartialEq)]
pub struct Value<Elem> {
    pub value: Elem,
    pub location: Location,
}

impl<Elem> Value<Elem> {
    pub fn new(start: usize, end: usize, value: Elem) -> Self {
        Value {
            value,
            location: Location::new(start, end),
        }
    }
}

impl<Elem: fmt::Debug> fmt::Debug for Value<Elem> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Addition(Value<String>),
    Subtraction(Value<String>),
    Multiplication(Value<String>),
    Division(Value<String>),
    Modulo(Value<String>),
    Power(Value<String>),
    Equal(Value<String>),
    Lt(Value<String>),
    Gt(Value<String>),
    Custom(Value<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Num(Value<String>),
    Dec(Value<String>),
    Float(Value<String>),
    Operator(Value<String>),
    Atom(Value<String>),
    Identifier(Value<String>),
    BinOp(Value<(Box<Expression>, Operator, Box<Expression>)>),
    IfStatement(Value<(Box<Expression>, Box<Expression>)>),
    IfElseStatement(Value<(Box<Expression>, Box<Expression>, Box<Expression>)>),
}
