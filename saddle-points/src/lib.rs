// use std::ops::Deref; // How to use it with those code?

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_length = input.first().unwrap_or(&vec![]).len();
    if row_length == 0 { return vec![] }

    let flat_vec = input.iter().flat_map(|x| x.iter()).collect::<Vec<&u64>>();
    let mut min_cols: Vec<_> =  Vec::with_capacity(row_length);
    input.iter().enumerate().filter_map(|tuple| {
        let max = (*tuple.1).iter().max().unwrap();

        if min_cols.is_empty() {
            for index in 0..row_length {
                let min = flat_vec.iter().skip(index).step_by(row_length).min().unwrap();
                min_cols.push((index, *min));
            }
        }
        Some(min_cols.iter().filter_map( |tuple_min| {
            if tuple_min.1 == max { return Some((tuple.0, tuple_min.0)) }
            None
        }).collect::<Vec<_>>())
    }).flatten().collect()
}
