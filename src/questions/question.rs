use super::Subject;

pub struct Question {
    subject: Subject,
    formula: Formula,
}
pub struct Formula {
    left: Expression,
    right: Vec<Expression>,
}
pub enum Expression {
    Value(Value),
    Add(Value, Value),
    Square(Value, Value),
    Divide(Value, Value),
}
pub enum Value {
    Variable(String, Option<f64>),
    Hard(f64),
}
/*
Quesiton
subject = Physcis
ke = 1/2 m *
s = d / t


Expression::Value(
    Value::Variable("speed" , None)
)
=
[
    Expression::Divide(
        Value::Variable("distance" , None),
        Value::Variable("time" , None)
    )
]

*/
