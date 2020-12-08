use std::ops::Range;


enum Operator {
    Exponentiation,
    Addition,
    Multiplication,
    Any,
}

/// if the value is None, then the operand is a placeholder
#[derive(Clone)]
enum Operand {
    Number(Option<f64>),
    PrefixExpression(Option<Box<PrefixExpression>>),
    Any,
}


#[derive(Clone)]
struct PrefixExpression {
    operator: Operator,
    first: Operand,
    second: Operand,
}


const TRYING_OPERATORS: [Operator; 3] = [Operator::Exponentiation, Operator::Addition, Operator::Multiplication];


struct HomonumericalPrefixExpressionGroup<'a> {
    numbers: &'a Vec<f64>,
    op_len: usize,
    expression_placeholders: Vec<PrefixExpression>,
}

impl<'a> HomonumericalPrefixExpressionGroup<'a> {

    fn new(numbers: &'a Vec<f64>) ->  Result<Self, &str> {
        let len = numbers.len();
        if len < 2 {
            Err("Need 2 or more numbers")
        } else {
            let res = Self {
                numbers,
                op_len: numbers.len() - 1,
                expression_placeholders: vec![],
            };
            res.build();
            Ok(res)
        }
    }

    fn _build(&self, root ,current:&mut PrefixExpression, placeheld_count:usize){

        todo!("finish this, every else case recurses into 3 cases: both operator / diff ");

        if placeheld_count == self.op_len {
            let clone = current.clone();

            current.first = Operand::Number(None);
            current.second = Operand::Number(None);
        }else{
            let clone = current.clone();
            current.first = Operand::PrefixExpression(Some(Box::new(
                PrefixExpression { operator: Operator::Any, first: Operand::Any, second: Operand::Any }
            )));


        }



    }

    fn build(&self){
        let mut initial = PrefixExpression { operator: Operator::Any, first: Operand::Any, second: Operand::Any };
        let number_len = self.numbers.len() - 1;
        self._build(& mut initial, 1);

        // an operator is placeheld if its one of the operand is not any anymore

        // theorem: a placeheld operator has this equivalent representation: onnonnonn
        // todo: this is a really lazy explanation
        let placeheld_operators:usize = 0;



        while placeheld_operators < number_len{
            if self.expression_placeholder{

            }

        }



    }

}

// PrefixExpression { operator: Operator::Any, first: Operand::Any, second: Operand::Any }

// impl Iterator for HomonumericalPrefixExpressionCounter<'_> {
//     type Item = PrefixExpression;
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//
//
//         let root = self.curr;
//         let buffer = self.expression_buffer.clone();
//
//
//         self.curr += 1;
//         // increase buffer state(s)
//         return self.next(self)
//     }
// }


fn _exhaust_prefix_expression() {}

/// The numbers and their inverses in `power_range` will be tried
fn exhaust_prefix_expressions(numbers: Vec<f64>, power_range: Range<i8>) -> PrefixExpression {
    let operand_limit = numbers.len() - 1;

    for op in TRYING_OPERATORS.iter() {}

    println!("received {:?} {:?}", numbers, power_range);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaust_prefix_expressions() {
        exhaust_prefix_expressions(vec![1f64, 2f64], 1..2);
        todo!()
    }
}
