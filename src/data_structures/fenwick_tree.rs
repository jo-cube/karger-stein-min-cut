pub struct FenwickTree {
    n: usize,
    tree: Box<[usize]>,
}

impl FenwickTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n: n + 1,
            tree: vec![0; n + 1].into_boxed_slice(),
        }
    }

    pub fn from(arr: &[usize]) -> Self {
        let mut tree = vec![0; arr.len() + 1].into_boxed_slice();
        for i in 1..=arr.len() {
            tree[i] += arr[i - 1];
            let j = i as isize;
            let j = (j + (j & -j)) as usize;
            if j <= arr.len() {
                tree[j] += tree[i];
            }
        }
        FenwickTree {
            n: arr.len() + 1,
            tree,
        }
    }

    pub fn query(&self, id: usize) -> usize {
        let mut id = id as isize + 1;
        let mut sum = 0;
        while id > 0 {
            sum += self.tree[id as usize];
            id -= id & -id;
        }
        sum
    }

    pub fn update(&mut self, id: usize, val: usize, subtract: bool) {
        let mut id = id as isize + 1;
        while id < self.n as isize {
            if subtract {
                self.tree[id as usize] -= val;
            } else {
                self.tree[id as usize] += val;
            }
            id += id & -id;
        }
    }

    pub fn lower_entry(&self, val: usize) -> (usize, usize) {
        if val <= self.tree[1] {
            return (0, 0);
        }

        let (mut start, mut end) = (1, self.n - 2);
        while start < end {
            let mid = (start + end) >> 1;
            let mid_val = self.query(mid);
            if mid_val >= val {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        (start, self.query(start - 1))
    }

    #[allow(dead_code)]
    pub fn sum(&self) -> usize {
        self.query(self.n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::fenwick_tree::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let fenwick = FenwickTree::new(5);
        assert_eq!(fenwick.n, 6);
        assert_eq!(fenwick.sum(), 0);
        assert_eq!(fenwick.tree[..], [0, 0, 0, 0, 0, 0]);

        let mut fenwick = FenwickTree::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(fenwick.n, 11);
        assert_eq!(fenwick.sum(), 55);
        assert_eq!(fenwick.tree[..], [0, 1, 3, 3, 10, 5, 11, 7, 36, 9, 19]);

        for (i, val) in [1, 3, 6, 10, 15, 21, 28, 36, 45, 55].iter().enumerate() {
            assert_eq!(fenwick.query(i), *val);
        }

        fenwick.update(3, 6, false);
        assert_eq!(fenwick.sum(), 61);
        assert_eq!(fenwick.query(2), 6);
        assert_eq!(fenwick.query(3), 16);
        assert_eq!(fenwick.query(7), 42);

        fenwick.update(3, 6, true);
        for (i, val) in [1, 3, 6, 10, 15, 21, 28, 36, 45, 55].iter().enumerate() {
            assert_eq!(fenwick.query(i), *val);
        }

        assert_eq!(fenwick.lower_entry(1), (0, 0));
        assert_eq!(fenwick.lower_entry(2), (1, 1));
        assert_eq!(fenwick.lower_entry(10), (3, 6));
        assert_eq!(fenwick.lower_entry(12), (4, 10));
        assert_eq!(fenwick.lower_entry(15), (4, 10));
        assert_eq!(fenwick.lower_entry(53), (9, 45));
        assert_eq!(fenwick.lower_entry(55), (9, 45));
        assert_eq!(fenwick.lower_entry(100), (9, 45));

        let fenwick = FenwickTree::from(&[5, 5, 5, 0, 0, 0, 0, 5, 5, 5]);
        assert_eq!(fenwick.lower_entry(15), (2, 10));
        assert_eq!(fenwick.lower_entry(18), (7, 15));
    }
}
