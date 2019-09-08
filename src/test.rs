#[cfg(test)]
mod tests {
    use crate::engine::*;
    use crate::ops::*;
    use crate::error::*;

    #[test]
    fn two_term() {
        let mut engine = Engine::<OpError>::new();

        let term1 = engine.term(Value{ val: 5 });
        let term2 = engine.term(Coefficient{ operand: term1, factor: 2 });
        assert_eq!(*engine.eval(&term2).unwrap(), 10);

        println!("OK");
    }

    #[test]
    fn three_term_triangle() {
        let mut engine = Engine::new();

        let val_a = engine.term(Value{ val: 5 });
        let val_b = engine.term(Value{ val: 4 });

        let mult = engine.term(Multiply{ a: val_a, b: val_b });

        assert_eq!(*engine.eval(&mult).unwrap(), 20);
    }

    #[test]
    fn three_term_linear() {
        let mut engine = Engine::new();

        let val = engine.term(Value{ val: 5 });
        let coef_a = engine.term(Coefficient{ operand: val, factor: 4 });

        let coef_b = engine.term(Coefficient{ operand: coef_a,
                                              factor: 6 });

        assert_eq!(*engine.eval(&coef_b).unwrap(), 120);
    }

    #[test]
    fn four_term_diamond() {
        let mut engine = Engine::new();

        let val = engine.term(Value{ val: 5 });
        let coef_a = engine.term(Coefficient{ operand: val, factor: 4 });

        let coef_b = engine.term(Coefficient{ operand: val,
                                                     factor: 6 });
        let mult = engine.term(Multiply{ a: coef_a, b: coef_b });

        assert_eq!(*engine.eval(&mult).unwrap(), 600);
    }

    #[test]
    fn random_list_expr() {
        let mut engine = Engine::new();

        let list = engine.term(Value { val: vec!(0, 1, 2, 3) });
        let val = engine.term(Value { val: 5 });

        let list_mul = engine.random_list_term(MultiplyListScalar{ l: list,
                                                                   c: val});

        assert_eq!(*engine.eval(&list_mul).unwrap(), vec!(0, 5, 10, 15));
    }

    #[test]
    fn sequential_list_expr() {
        let mut engine = Engine::new();

        let start = engine.term(Value { val: 0 });
        let end = engine.term(Value { val: 10 });
        let inc = engine.term(Value { val: 2 });

        let count = engine.sequential_list_term(CountList{ start: start,
                                                           end: end,
                                                           inc: inc});

        assert_eq!(*engine.eval(&count).unwrap(), vec!(0, 2, 4, 6, 8, 10));
    }

}
