fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;


        //pop returns an Option because it may not succeed. this loop terminates when the returned
        //Option is None. now, when it returns Some, it gets unwrapped into the element type of the
        //Vec, which in this case is itself an Option!
        while let Some(opt_integer) = optional_integers.pop() {
            if let Some(integer) = opt_integer {
            assert_eq!(integer, cursor);
            cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
