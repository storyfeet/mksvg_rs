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
        t_sofar: 0,
        t_req: 0,
    }
}

pub struct SpreadIter<CIT: Iterator<Item = C>, C: Clone, F: Fn(&C) -> usize> {
    it: CIT,
    curr: Option<C>,
    count_f: F,
    t_sofar: usize,
    t_req: usize,
}

impl<CIT, C, F> Iterator for SpreadIter<CIT, C, F>
where
    CIT: Iterator<Item = C>,
    C: Clone,
    F: Fn(&C) -> usize,
{
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() || self.t_req <= self.t_sofar {
            self.curr = self.it.next();
            self.t_req = match self.curr {
                Some(ref c) => (self.count_f)(c),
                None => return None,
            };
            self.t_sofar = 0;
        }
        self.t_sofar += 1;
        match self.curr.as_ref() {
            Some(rv) => Some(rv.clone()),

            None => None,
        }
    }
}

pub fn spread_nc<I, IN, C, F>(i: IN, f: F) -> SpreadIterNC<I, C, F>
where
    I: Iterator<Item = C>,
    IN: IntoIterator<IntoIter = I, Item = C>,
    F: Fn(&C) -> usize,
    C: Clone,
{
    SpreadIterNC {
        it: i.into_iter(),
        curr: None,
        count_f: f,
        sofar: 0,
        t_sofar: 0,
        t_req: 0,
    }
}
pub struct SpreadIterNC<CIT: Iterator<Item = C>, C: Clone, F: Fn(&C) -> usize> {
    it: CIT,
    curr: Option<C>,
    count_f: F,
    sofar: usize,
    t_sofar: usize,
    t_req: usize,
}

impl<CIT, C, F> Iterator for SpreadIterNC<CIT, C, F>
where
    CIT: Iterator<Item = C>,
    C: Clone,
    F: Fn(&C) -> usize,
{
    type Item = (C, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() || self.t_req <= self.t_sofar {
            self.curr = self.it.next();
            self.t_req = match self.curr {
                Some(ref c) => (self.count_f)(c),
                None => return None,
            };
            self.t_sofar = 0;
        }
        self.t_sofar += 1;
        self.sofar += 1;
        match self.curr.as_ref() {
            Some(rv) => Some((rv.clone(), self.sofar - 1, self.t_sofar - 1)),

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
