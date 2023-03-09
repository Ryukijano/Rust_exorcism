#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}
use std::cmp::min;
pub fn sublist<'a, T: Eq>(xs: &'a [T], ys: &'a [T]) -> Comparison {
    let l1 = xs.len();
    let l2 = ys.len();
    if l1 == l2 && xs == ys {
        Comparison::Equal
    } else if l1 > l2 && contains(xs, ys) {
        Comparison::Superlist
    } else if l2 > l1 && contains(ys, xs) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
fn contains<'a, T: Eq>(xs: &'a [T], ys: &'a [T]) -> bool {
    let l1 = xs.len();
    let l2 = ys.len();
    let diff = l1 - l2;
    l2 == 0 || (0 .. l1 - diff)
        .any(|start| &xs[start .. min(l1, start + l2)] == ys)
}