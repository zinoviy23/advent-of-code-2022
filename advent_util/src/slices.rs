pub fn foreach_windows<T, const N: usize, F>(slice: &mut [T], mut f: F)
where
    F: FnMut(&mut [T; N]),
{
    let mut start = 0usize;
    let mut end = N;
    while end <= slice.len() {
        let array: &mut [T; N] = (&mut slice[start..end]).try_into().unwrap();
        f(array);
        start += 1;
        end += 1;
    }
}
