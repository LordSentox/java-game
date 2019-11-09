/// Checks if two vectors are equal (i.e. contain the same elements)
/// 
/// # Parameters
/// * `vec_a` - The first vector
/// * `vec_b` - The second vector
/// 
/// # Returns
/// `true` if both vectors contains the same elements, `false` otherwise
pub fn vec_equals<T: PartialEq>(vec_a: &Vec<T>, vec_b: &Vec<T>) -> bool {
    for elem in vec_a.iter() {
        if !vec_b.contains(elem) {
            return false;
        }
    }

    for elem in vec_b.iter() {
        if !vec_a.contains(elem) {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn equals() {
        let vec_a = vec![1, 2, 3, 4, 5];
        let vec_b = vec![5, 4, 3, 2, 1];

        assert_eq!(vec_equals(&vec_a, &vec_b), true);
    }

    #[test]
    fn not_equals() {
        let vec_a = vec![1, 2, 3, 4, 5];
        let vec_b = vec![5, 4, 3, 2, 42];

        assert_eq!(vec_equals(&vec_a, &vec_b), false);
    }
}