pub mod vector;
pub mod point;
pub mod dimension;

const F64_PRECISION: u32 = 53;

pub(crate) fn count_digits(num: f64) -> u64 {
    let num_str = format!("{}", num);
    num_str.replace(".", "").len().try_into().unwrap()
}