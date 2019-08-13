#[cfg(test)]
mod tests {
    use crate::engine::*;
    use crate::ops::*;

    #[test]
    fn two_term() {
        let mut builder = Engine::new();
        
        let term1 = builder.term(Value::<i32>{ val: 5 });
        let term2 = builder.term(Multiply::<i32>{ operand: term1, factor: 2 });
        assert_eq!(*builder.eval(&term2).unwrap(), 10);

        println!("OK");
    }
}
