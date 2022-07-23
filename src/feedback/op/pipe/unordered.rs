use std::cmp::Ordering;

pub struct Unordered<T>(pub T);
impl<T> PartialEq for Unordered<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl<T> Eq for Unordered<T> {}
impl<T> PartialOrd for Unordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Unordered<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}
