/// Sums the digits in a non-negative integer.
pub fn sum_digits(n: u64) -> u64 {
    let mut r = 0u64;
    let mut n = n;

    while n > 0 {
        r += n % 10;
        n /= 10;
    }

    r
}

/// Used in the lookahead iterator.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LookaheadPos {
    First,
    Last,
    Middle,
    Only,
}

pub struct Lookahead<I, T> {
    iter: I,
    seen_first: bool,
    curr_item: Option<T>
}

impl<I, T> Iterator for Lookahead<I, T>
where I: Iterator<Item = T>,
      T: Clone {
    type Item = (LookaheadPos, T);

    fn next(&mut self) -> Option<Self::Item> {
        // Set the current item if not already set.
        if let None = self.curr_item {
            // If the first item has already been seen,
            // that means a that None has alreeady been encountered on the main iterator.
            if self.seen_first {
                return None;
            }

            let first_opt_item = self.iter.next();

            match first_opt_item {
                None => {
                    // Iterator is already empty to begin with.
                    return None;
                },
                Some(first_item) => {
                    // Found a value, record it as the most recently seen item.
                    self.curr_item = Some(first_item);
                },
            }
        }

        // Get the new next value.
        let next_opt_item = self.iter.next();

        let is_curr_first = if !self.seen_first {
            self.seen_first = true;
            true
        }
        else {
            false
        };

        let to_return = match next_opt_item {
            Some(_) => {
                // A next element was found.
                if is_curr_first {
                    self.curr_item.clone().map(|i| (LookaheadPos::First, i))
                }
                else {
                    self.curr_item.clone().map(|i| (LookaheadPos::Middle, i))
                }
            },
            None => {
                // No next element was found.
                if is_curr_first {
                    self.curr_item.clone().map(|i| (LookaheadPos::Only, i))
                }
                else {
                    self.curr_item.clone().map(|i| (LookaheadPos::Last, i))
                }
            }
        };

        self.curr_item = next_opt_item;

        to_return
    }
}

trait LookaheadExt: Iterator {
    fn lookahead(self) -> Lookahead<Self, <Self as Iterator>::Item>
    where
        Self::Item: Clone,
        Self: Sized {
        Lookahead {
            iter: self,
            seen_first: false,
            curr_item: None,
        }
    }
}

impl<I: Iterator> LookaheadExt for I {}

#[cfg(test)]
mod tests {
    use super::sum_digits;

    #[test]
    fn test_sum_digits() {
        let inputs_and_expected: Vec<(u64, u64)> = vec![
            (0, 0),
            (1, 1),
            (10, 1),
            (15, 6),
            (247, 13),
            (1000, 1),
            (123456, 21),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = sum_digits(input);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn test_lookahead() {
        use super::Lookahead;
        use super::LookaheadExt;
        use super::LookaheadPos;

        let inputs_and_expected = vec![
            (
                vec![0, 1, 2],
                vec![
                    (LookaheadPos::First, &0),
                    (LookaheadPos::Middle, &1),
                    (LookaheadPos::Last, &2),
                ],
            ),
            (
                vec![0],
                vec![
                    (LookaheadPos::Only, &0),
                ],
            ),
            (
                vec![0, 1],
                vec![
                    (LookaheadPos::First, &0),
                    (LookaheadPos::Last, &1),
                ],
            ),
            (
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![
                    (LookaheadPos::First, &0),
                    (LookaheadPos::Middle, &1),
                    (LookaheadPos::Middle, &2),
                    (LookaheadPos::Middle, &3),
                    (LookaheadPos::Middle, &4),
                    (LookaheadPos::Middle, &5),
                    (LookaheadPos::Middle, &6),
                    (LookaheadPos::Middle, &7),
                    (LookaheadPos::Middle, &8),
                    (LookaheadPos::Last, &9),
                ],
            ),
            (
                vec![],
                vec![],
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced: Vec<_> = input.iter().lookahead().collect();
            assert_eq!(expected, produced);
        }
    }
}
