use markup::Render;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub enum Option<R>
where
    R: std::cmp::PartialEq + std::fmt::Display,
{
    Same,
    Different(R),
}

impl<R> std::fmt::Display for Option<R>
where
    R: std::fmt::Display + std::cmp::PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            crate::ditto::Option::Same => write!(f, ""),
            crate::ditto::Option::Different(inner) => write!(f, "{}", inner),
        }
    }
}

impl<R> Render for Option<R>
where
    R: std::fmt::Display + std::cmp::PartialEq,
{
    fn render(&self, writer: &mut impl Write) -> std::fmt::Result {
        match self {
            crate::ditto::Option::Same => write!(writer, ""),
            crate::ditto::Option::Different(inner) => write!(writer, "{}", inner),
        }
    }
}

pub struct Ditto<I, T, const N: usize>
where
    I: Iterator<Item = [T; N]>,
    T: std::cmp::PartialEq,
{
    iterator: I,
    prev: std::option::Option<[T; N]>,
}

impl<I: Iterator<Item = [T; N]>, T: std::cmp::PartialEq, const N: usize> Ditto<I, T, N> {
    pub const fn new(iterator: I) -> Self {
        Self {
            iterator,
            prev: None,
        }
    }
}

impl<
        I: Iterator<Item = [T; N]>,
        T: std::cmp::PartialEq + std::clone::Clone + std::fmt::Debug + std::fmt::Display,
        const N: usize,
    > Iterator for Ditto<I, T, N>
{
    type Item = [Option<T>; N];

    fn next(&mut self) -> std::option::Option<[Option<T>; N]> {
        match &self.prev {
            Some(prev) => match self.iterator.next() {
                None => None,
                Some(next) => {
                    let result = next
                        .clone()
                        .into_iter()
                        .zip(prev.clone().into_iter())
                        .map(|(a, b)| {
                            if a == b {
                                Option::Same
                            } else {
                                Option::Different(a)
                            }
                        })
                        .array_chunks()
                        .next();
                    self.prev = Some(next);
                    result
                }
            },
            None => match self.iterator.next() {
                None => None,
                Some(next) => {
                    let result = next
                        .clone()
                        .into_iter()
                        .map(|a| Option::Different(a))
                        .array_chunks()
                        .next();
                    self.prev = Some(next);
                    result
                }
            },
        }
    }
}

#[test]
fn test_ditto_iterator_empty() {
    assert_eq!(
        Ditto::new(Vec::<[usize; 0]>::new().into_iter()).next(),
        None
    );
}

#[test]
fn test_ditto_iterator_one() {
    use crate::ditto::Option::Different;

    let iterator = vec![["foo", "bar"]].into_iter();
    let mut ditto = Ditto::new(iterator);
    assert_eq!(ditto.next(), Some([Different("foo"), Different("bar")]));
    assert_eq!(ditto.next(), None);
}

#[test]
fn test_ditto_iterator_two_same_left() {
    use crate::ditto::Option::{Different, Same};

    let iterator = vec![["foo", "bar"], ["foo", "baz"]].into_iter();
    let mut ditto = Ditto::new(iterator);
    assert_eq!(ditto.next(), Some([Different("foo"), Different("bar")]));
    assert_eq!(ditto.next(), Some([Same, Different("baz")]));
    assert_eq!(ditto.next(), None);
}

#[test]
fn test_ditto_iterator_two_same_right() {
    use crate::ditto::Option::{Different, Same};

    let iterator = vec![["foo", "bar"], ["baz", "bar"]].into_iter();
    let mut ditto = Ditto::new(iterator);
    assert_eq!(ditto.next(), Some([Different("foo"), Different("bar")]));
    assert_eq!(ditto.next(), Some([Different("baz"), Same]));
    assert_eq!(ditto.next(), None);
}
