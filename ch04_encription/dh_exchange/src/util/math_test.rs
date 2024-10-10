use super::math;
#[cfg(test)]
mod tests {
    use super::math;
    #[test]
    fn bin_vec() {
        let target = vec![0, 1, 0, 1];
        assert_eq!(math::to_binary_list(&10), target);
    }

    #[test]
    fn bin_vec_scaled() {
        let target = vec![0, 1, 0, 1, 0, 0, 0, 0];
        assert_eq!(math::to_bibary_vec_scaled(&10, &8), target);
    }

    #[test]
    fn pow_test() {
        assert_eq!(math::pow_by_montgomery_ladder(&2, &10, &987), 37);
    }
}
