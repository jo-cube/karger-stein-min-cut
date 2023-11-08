use super::graph_util::MergeUtil;

pub struct UnionFind {
    pub count: usize,
    pub(crate) ids: Box<[usize]>,
    sizes: Box<[usize]>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            count: n,
            ids: (0..n).collect::<Box<[usize]>>(),
            sizes: vec![1; n].into_boxed_slice(),
        }
    }

    pub fn root(&mut self, p: usize) -> usize {
        let mut p = p;
        while self.ids[p] != p {
            self.ids[p] = self.ids[self.ids[p]];
            p = self.ids[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) -> usize {
        let (i, j) = (self.root(p), self.root(q));
        if i == j {
            return i;
        }
        self.count -= 1;
        let (max, min) = if self.sizes[i] >= self.sizes[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.ids[min] = max;
        self.sizes[max] += self.sizes[min];
        max
    }

    #[allow(dead_code)]
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn condense(mut self, merge_util: &mut MergeUtil) -> Box<[usize]> {
        let mut stack_size = 0;
        let MergeUtil { stack, merge_proxy } = merge_util;

        for id in 0..self.ids.len() {
            let root = self.root(id);
            if id == root && merge_proxy[root] == 0 {
                stack[stack_size] = root;
                stack_size += 1;
                merge_proxy[root] = stack_size;
            }
        }

        for i in 0..self.ids.len() {
            self.ids[i] = self.root(self.ids[i]);
        }

        for i in 0..self.ids.len() {
            self.ids[i] = merge_proxy[self.ids[i]] - 1;
        }

        for i in 0..stack_size {
            merge_proxy[stack[i]] = 0;
        }

        self.ids
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::graph_util::MergeUtil;
    use crate::data_structures::union_find::UnionFind;

    #[test]
    fn test_union_find() {
        let mut union_find = UnionFind::new(5);
        assert_eq!(union_find.ids[..], [0, 1, 2, 3, 4]);

        union_find.union(1, 3);
        union_find.union(3, 4);
        assert_eq!(union_find.ids[..], [0, 1, 2, 1, 1]);

        assert_eq!(union_find.count, 3);
        assert_eq!(union_find.root(4), 1);
        assert!(union_find.connected(1, 4));
        assert!(!union_find.connected(0, 4));

        union_find.union(3, 4);
        assert_eq!(union_find.count, 3);
        assert_eq!(union_find.root(4), 1);

        let mut union_find = UnionFind::new(6);
        union_find.union(3, 4);
        union_find.union(0, 3);
        union_find.union(1, 2);
        assert_eq!(union_find.ids[..], [3, 1, 1, 3, 3, 5]);

        let mut merge_util = MergeUtil::new(6);
        let ids = union_find.condense(&mut merge_util);
        assert_eq!(ids[..], [1, 0, 0, 1, 1, 2]);
        assert_eq!(merge_util.merge_proxy[..], [0, 0, 0, 0, 0, 0]);

        let mut union_find = UnionFind::new(8);
        union_find.union(3, 0);
        union_find.union(4, 7);
        union_find.union(1, 2);
        union_find.union(0, 4);
        assert_eq!(union_find.ids[..], [3, 1, 1, 3, 3, 5, 6, 4]);

        let mut merge_util = MergeUtil::new(8);
        let ids = union_find.condense(&mut merge_util);
        assert_eq!(ids[..], [1, 0, 0, 1, 1, 2, 3, 1]);
        assert_eq!(merge_util.merge_proxy[..], [0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
