pub struct Permutations<T> where T : Clone + Copy + std::fmt::Debug {
    data: Vec<T>,
    stack: Vec<usize>,
    popped: bool,
}

impl<T> Permutations<T> where T : Clone + Copy + std::fmt::Debug {
    pub fn count(&mut self) -> usize {
        let mut count = 0;

        while let Some(_) = self.next() {
            count += 1;
        }

        count
    }

    pub fn next(&mut self) -> Option<&[T]> {
        if self.stack.len() == 0 {
            return None
        }

        let from = self.stack.len() - 1;
        let to = self.data.len();
        let mut i = *self.stack.last().unwrap();

        if self.popped {
            self.data.swap(from, i);
            *self.stack.last_mut().unwrap() += 1;
            i += 1;
        }
        self.popped = false;

        if self.stack.len() == self.data.len() {
            self.stack.pop();
            self.popped = true;

            Some(&self.data)
        } else {
            if i == to {
                self.stack.pop();
                self.popped = true;

                self.next()
            } else {
                self.data.swap(from, i);
                self.stack.push(from + 1);

                self.next()
            }
        }
    }

    pub fn new(items: &[T]) -> Permutations<T> where T: Clone + Copy + std::fmt::Debug {
        if items.len() < 2 {
            panic!("permutations of too short list.")
        }

        let mut stack = Vec::with_capacity(items.len());
        stack.push(0);

        Permutations{
            data: Vec::from(items),
            popped: false,
            stack,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutations_count() {
        assert_eq!(Permutations::new(&[1, 2, 3]).count(), 6);
        assert_eq!(Permutations::new(&[1, 2, 3, 4]).count(), 24);
        assert_eq!(Permutations::new(&[1, 2, 3, 4, 5]).count(), 120);
        assert_eq!(Permutations::new(&[1, 2, 3, 4, 5, 6]).count(), 720);
    }

    #[test]
    fn test_permutations_all() {
        let mut hs: HashSet<Vec<i32>> = HashSet::with_capacity(320);
        let mut perm = Permutations::new(&[1, 2, 3]);

        while let Some(v) = perm.next() {
            println!("{:?}", v);

            if hs.contains(v) {
                panic!("repeated permutation: {:?}", v);
            }
            hs.insert(v.to_vec());
        }
    }
}