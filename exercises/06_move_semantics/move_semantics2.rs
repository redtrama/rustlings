fn fill_vec(vec: &mut Vec<i32>) ->  Vec<i32> {
    let mut vec1 = vec.clone();

    vec1.push(88);

    vec1
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];
        // we pass here &mut vec0 but we need to copy it inside the function in order to reuser vec0
        let vec1 = fill_vec(&mut vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
