use std::collections::HashMap;
use std::iter::Map;

trait Expression {
    fn evaluate(&self) -> f64;
    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression>;
    fn contains_x(&self) -> bool;
    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>);
}

struct OperationExpression {
    left: Option<Box<dyn Expression>>,
    left_name: String,
    token: char,
    right: Option<Box<dyn Expression>>,
    right_name: String,
}

struct OneOverExpression {
    expression: Box<dyn Expression>
}

impl Expression for OneOverExpression {
    fn evaluate(&self) -> f64 {
        1.0 / self.expression.evaluate()
    }

    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression> {
        self.expression.reorder(Box::new(OneOverExpression{
            expression: other_side
        }))
    }

    fn contains_x(&self) -> bool {
        self.expression.contains_x()
    }

    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>) {
        todo!()
    }
}

struct NegativeExpression {
    expression: Box<dyn Expression>
}

impl Expression for NegativeExpression {
    fn evaluate(&self) -> f64 {
        -1.0 * self.expression.evaluate()
    }

    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression> {
        self.expression.reorder(Box::new(NegativeExpression{
            expression: other_side
        }))
    }

    fn contains_x(&self) -> bool {
        self.expression.contains_x()
    }

    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>) {
        todo!()
    }
}


impl OperationExpression {
    fn apply_token(&self, a: f64, b: f64) -> f64 {
        match self.token {
            '/' =>  a / b,
            '*' =>  a * b,
            '+' =>  a + b,
            '-' =>  a - b,
            _ => panic!()
        }.round()
    }

    fn invert_operation(operation: char) -> char {
        match operation {
            '/' => '*',
            '*' => '/',
            '+' => '-',
            '-' => '+',
            _ => panic!()
        }
    }
}

impl Expression for OperationExpression {
    fn evaluate(&self) -> f64 {
        self.apply_token(self.left.as_ref().unwrap().evaluate(), self.right.as_ref().unwrap().evaluate())
    }

    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression>{
        let mut to_move;
        let mut to_keep;
        let mut operation = self.token;
        if self.left.as_ref().unwrap().contains_x() {
            to_keep = self.left.unwrap();
            to_move = self.right;

            operation = OperationExpression::invert_operation(operation);
        }
        else {
            to_move = self.left;

            operation = OperationExpression::invert_operation(operation);
            to_keep = self.right.unwrap();
        }

        let new_right = Box::new(OperationExpression {
            left: Some(other_side),
            right: to_move,
            token: operation,
            left_name: "".to_string(),
            right_name: "".to_string(),
        });

        to_keep.reorder(new_right)
    }

    fn contains_x(&self) -> bool {
        return self.left.as_ref().unwrap().contains_x() || self.right.as_ref().unwrap().contains_x();
    }

    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>) {
        let mut right = expression.remove(self.right_name.as_str()).unwrap();
        right.fill(expression);

        match self.token {
            '/' => {
                self.right = Some(Box::new(OneOverExpression{expression: right}));
                self.token = '*';
            },
            '-' => {
                self.token = '+';
                self.right = Some(Box::new(NegativeExpression{expression: right}));
            }
            _ => {
                self.right = Some(right);
            }
        }

        let mut left = expression.remove(self.left_name.as_str()).unwrap();
        left.fill(expression);
        self.left = Some(left);
    }
}

struct XExpression {

}

impl Expression for XExpression {
    fn evaluate(&self) -> f64 {
        panic!()
    }

    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression>{
        other_side
    }

    fn contains_x(&self) -> bool {
        return true;
    }

    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>) {

    }
}

struct LiteralExpression {
    value: f64
}

impl Expression for LiteralExpression {
    fn evaluate(&self) -> f64 {
        self.value
    }

    fn reorder(self: Box<Self>, other_side: Box<dyn Expression>) -> Box<dyn Expression>{
        panic!()
    }

    fn contains_x(&self) -> bool {
        return false;
    }

    fn fill(&mut self, expression: &mut HashMap<String, Box<dyn Expression>>) {

    }
}

fn parse_expression(expression_line: &str, expressions: &mut HashMap<String, Box<dyn Expression>>, fancy: bool) {
    let split_string = expression_line.replace(":", "");
    let mut tokens = split_string.split(" ");

    let name = tokens.next().unwrap();

    if fancy {
        if name == "root" {
            let left_name = tokens.next().unwrap().parse().unwrap();
            let token = tokens.next().unwrap().chars().next().unwrap();
            let right_name = tokens.next().unwrap().parse().unwrap();

            expressions.insert(name.to_string(), Box::new(OperationExpression {
                left_name,
                right_name,
                token: '-',
                right: None,
                left: None
            }));

            return;
        }

        if name == "humn" {
            expressions.insert(name.to_string(), Box::new(XExpression{}));
            return;
        }
    }

    if(tokens.clone().count() == 3) {
        let left_name = tokens.next().unwrap().parse().unwrap();
        let token = tokens.next().unwrap().chars().next().unwrap();
        let right_name = tokens.next().unwrap().parse().unwrap();

                expressions.insert(name.to_string(), Box::new(OperationExpression {
                    left_name,
                    right_name,
                    token,
                    right: None,
                    left: None,
                }));

    }
    else {
        expressions.insert(name.to_string(), Box::new(LiteralExpression {
            value: tokens.next().unwrap().parse().unwrap()
        }));
    }
}

fn parse_expressions(expressions_string: &str, fancy: bool) -> Box<dyn Expression>{
    let mut expressions = HashMap::new();

    for line in expressions_string.lines(){
        parse_expression(line, &mut expressions, fancy);
    }

    let mut root = expressions.remove("root").unwrap();

    root.fill(&mut expressions);

    return root;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let result = parse_expressions(include_str!("day21/test_simple.txt"), false).evaluate();

        assert_eq!(result, 152.0);
    }

    #[test]
    fn large_test() {
        let result = parse_expressions(include_str!("day21/test_large.txt") ,false).evaluate();

        assert_eq!(result, 364367103397416.0);
    }

    #[test]
    fn large_test_2() {
        let mut expressions = parse_expressions(include_str!("day21/test_large.txt"), true);

        let result = expressions.reorder(Box::new(LiteralExpression{value: 0.0})).evaluate();
        assert_eq!(result, 364367103397416.0);
    }

    #[test]
    fn simple_test_2() {
        let mut expressions = parse_expressions(include_str!("day21/test_simple.txt"), true);

        let result = expressions.reorder(Box::new(LiteralExpression{value: 0.0})).evaluate();
        assert_eq!(result, 301.0);
    }
}