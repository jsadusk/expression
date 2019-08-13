#[cfg(test)]
mod tests {
    use crate::engine::*;
    use crate::ops::*;

    #[test]
    fn two_term() {
        let mut engine = Engine::new();
        
        let term1 = engine.term(Value::<i32>{ val: 5 });
        let term2 = engine.term(Coefficient::<i32>{ operand: term1, factor: 2 });
        assert_eq!(*engine.eval(&term2).unwrap(), 10);

        println!("OK");
    }

    #[test]
    fn three_term_triangle() {
        let mut engine = Engine::new();

        let val_a = engine.term(Value::<i32>{ val: 5 });
        let val_b = engine.term(Value::<i32>{ val: 4 });

        let mult = engine.term(Multiply::<i32>{ a: val_a, b: val_b });

        assert_eq!(*engine.eval(&mult).unwrap(), 20);
    }

    #[test]
    fn three_term_linear() {
        let mut engine = Engine::new();

        let val = engine.term(Value::<i32>{ val: 5 });
        let coef_a = engine.term(Coefficient::<i32>{ operand: val, factor: 4 });

        let coef_b = engine.term(Coefficient::<i32>{ operand: coef_a,
                                                     factor: 6 });

        assert_eq!(*engine.eval(&coef_b).unwrap(), 120);
    }

    #[test]
    fn four_term_diamond() {
        let mut engine = Engine::new();

        let val = engine.term(Value::<i32>{ val: 5 });
        let coef_a = engine.term(Coefficient::<i32>{ operand: val, factor: 4 });

        let coef_b = engine.term(Coefficient::<i32>{ operand: val,
                                                     factor: 6 });
        let mult = engine.term(Multiply::<i32>{ a: coef_a, b: coef_b });

        assert_eq!(*engine.eval(&mult).unwrap(), 600);
    }
    
}
