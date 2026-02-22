use std::cmp::Ordering;

pub fn cmp_score_desc_id_asc<T: AsRef<str>>(
    a_score: i32,
    a_id: T,
    b_score: i32,
    b_id: T,
) -> Ordering {
    b_score.cmp(&a_score).then_with(|| a_id.as_ref().cmp(b_id.as_ref()))
}

pub fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
