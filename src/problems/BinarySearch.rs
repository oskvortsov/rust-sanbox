// https://exercism.org/tracks/rust/exercises/binary-search/iterations?idx=2

// topics #[Generic, Iterable, checked_sub]
use std::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: Ord>(space: R, key: T) -> Option<usize> {
    let space = space.as_ref();
    let mut left: usize = 0;
    let mut right  = space.len();

    while left <= right {
        let mid = left + (right - left) / 2;

        match key.cmp(space.get(mid)?) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
            Ordering::Less => right = mid.checked_sub(1)?
        }
    }

    None
}

pub fn find1<R: AsRef<[T]>, T: Ord>(space: R, key: T) -> Option<usize> {
    let space = space.as_ref();
    let mid = space.len() / 2;

    match key.cmp(space.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&space[..mid], key),
        Ordering::Greater => find(&space[mid+1..], key).map(|x| x + mid + 1)
    }
}

pub fn find2<T: PartialOrd>(array: &[T], key: T) -> Option<usize> {
    let mut left: isize = 0;
    let mut right: isize = array.len() as isize;

    while left <= right {
        let mid: isize = left + (right - left) / 2;

        if mid >= array.len() as isize {
            return None
        }

        if array[mid as usize] == key {
            return Some(mid as usize)
        }

        if array[mid as usize] < key {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }

    None
}
