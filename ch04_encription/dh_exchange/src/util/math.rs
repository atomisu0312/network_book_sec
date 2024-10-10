pub fn to_binary_list(a: &u32) -> Vec<u8> {
    let mut binary_list = Vec::new();

    let mut n = *a;
    while n > 0 {
        binary_list.push((n % 2) as u8);
        n >>= 1;
    }
    binary_list
}

pub fn to_bibary_vec_scaled(a: &u32, scale: &usize) -> Vec<u8> {
    let binary_list = to_binary_list(a);
    let length = binary_list.len();
    let zero_vec = vec![0; scale - length];
    [binary_list, zero_vec].concat()
}

pub fn pow_by_montgomery_ladder(base: &u32, exp: &u32, modulo: &u32) -> u32 {
    let exp_vec: Vec<u8> = to_bibary_vec_scaled(exp, &32);
    let length = 32;
    let mut a0 = 1;
    let mut a1 = *base % modulo;

    for i in (0..length).rev() {
        if exp_vec[i] == 0 {
            a1 = (a0 * a1) % modulo;
            a0 = (a0 * a0) % modulo;
        } else {
            a0 = (a0 * a1) % modulo;
            a1 = (a1 * a1) % modulo;
        }
    }
    return a0;
}
