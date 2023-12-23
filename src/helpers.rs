pub fn iter_get<T: Default + Copy, I: Iterator<Item = T>, const N: usize>(iter: &mut I) -> [T; N] {
    let mut result = [T::default(); N];

    for (index, item) in iter.take(N).enumerate() {
        result[index] = item;
    }

    result
}
