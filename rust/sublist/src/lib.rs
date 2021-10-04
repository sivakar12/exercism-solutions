#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
fn is_sublist<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    if sublist.len() > superlist.len() { return false; }
    if sublist == [] { return true; }
    superlist.windows(sublist.len())
        .any(|window| {
            *window == *sublist
        })
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (is_sublist(_first_list, _second_list), is_sublist(_second_list, _first_list)) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal
    }
}
