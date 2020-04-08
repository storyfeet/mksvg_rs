pub fn spread<I, IN, C, F>(i: IN, f: F) -> SpreadIter<I, C, F>
where
    I: Iterator<Item = C>,
    IN: IntoIterator<IntoIter = I, Item = C>,
    F: Fn(&C) -> usize,
    C: Clone,
{
    SpreadIter {
        it: i.into_iter(),
        curr: None,
        count_f: f,
        rem: 0,
    }
}

pub struct SpreadIter<CIT: Iterator<Item = C>, C: Clone, F: Fn(&C) -> usize> {
    it: CIT,
    curr: Option<C>,
    count_f: F,
    rem: usize,
}

impl<CIT, C, F> Iterator for SpreadIter<CIT, C, F>
where
    CIT: Iterator<Item = C>,
    C: Clone,
    F: Fn(&C) -> usize,
{
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            self.curr = self.it.next();
            if let Some(ref c) = self.curr {
                self.rem = (self.count_f)(c);
            }
        };
        match self.curr.as_ref() {
            Some(_) => {
                if self.rem <= 0 {
                    self.curr = self.it.next();
                    if let Some(ref c) = self.curr {
                        self.rem = (self.count_f)(c) - 1;
                    }
                    self.curr.clone()
                } else {
                    self.rem -= 1;
                    self.curr.clone()
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[derive(Clone, Debug)]
    pub struct Counter(usize);

    #[test]
    pub fn test_spread_iter_gets_all_the_counts() {
        let v = vec![Counter(3), Counter(4), Counter(1)];
        let si = spread(v, |c| c.0);
        let nums: Vec<usize> = si.map(|v| v.0).collect();
        assert_eq!(nums, vec![3, 3, 3, 4, 4, 4, 4, 1]);
    }
}
