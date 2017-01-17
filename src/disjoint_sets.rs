type DisjointSet = Vec<Option<usize>>;

// TODO what about a bounds check for find?
// otherwise these are "unsafe" and can panic.
// With that change we could return an Option<usize>
// that would represent if our find succeeded
// (similar in how vector `get` is implemented),
// or maybe a Result if we plan to be very explicit
// about the error.

trait UnionFind {
    fn union(&self, i: usize, j: usize) -> DisjointSet;
    fn find(&self, i: usize) -> usize;

    fn compressed_find(&mut self, i: usize) -> usize;
    fn compressed_find_depth(&mut self, i: usize, depth: usize) -> (usize, usize);
    fn compressed_union(&mut self, i: usize, j: usize);
}

impl UnionFind for DisjointSet {
    // Return the name of the set that contains i.
    // In general one should use the path compressed
    // version `compressed_find`.
    fn find(&self, i: usize) -> usize {
        println!("-- find: {:?}", i);
        if let Some(u) = self[i] {
            return self.find(u);
        } else {
            return i;
        }
    }

    // Assuming i and j belong to different sets, return the
    // union of the two sets. In general one should use the
    // path compressed and better linking `compressed_union`.
    fn union(&self, i: usize, j: usize) -> DisjointSet {
        println!("union: {:?} {:?}", i, j);
        let x = self.find(i);
        let y = self.find(j);

        let mut v = self.clone();

        if x != y {
            v[x] = Some(y);
        }

        return v;
    }

    // If we're operating on a mutable disjoint set,
    // we can use path compression to make the find
    // operation faster with repeated use.
    fn compressed_find(&mut self, i: usize) -> usize {
        let (parent, _) = self.compressed_find_depth(i, 0);
        parent
    }

    // Actual implementation of the path compressed find,
    // adds an additional argument for tracking depth of
    // the find operation.
    fn compressed_find_depth(&mut self, i: usize, depth: usize) -> (usize, usize) {
        if let Some(u) = self[i] {
            let (parent, depth) = self.compressed_find_depth(u, depth + 1);
            self[i] = Some(parent);
            return (parent, depth);
        } else {
            return (i, depth);
        }
    }

    // In addition to using path compressed find, we can keep
    // tree size minimal by always linking the smaller to the
    // larger tree
    fn compressed_union(&mut self, i: usize, j: usize) {
        let (x, x_depth) = self.compressed_find_depth(i, 0);
        let (y, y_depth) = self.compressed_find_depth(j, 0);

        if x != y {
            if x_depth > y_depth {
                self[y] = Some(x);
            } else {
                self[x] = Some(y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use disjoint_sets::{UnionFind, DisjointSet};

    #[test]
    fn find_works() {
        let v: DisjointSet = vec![None, None, Some(0)];
        assert_eq!(v.find(0), 0);
        assert_eq!(v.find(1), 1);
        assert_eq!(v.find(2), 0);

        let v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None, Some(5),
                                  Some(0), Some(11), Some(8), Some(11), Some(6), Some(6), Some(2),
                                  None, Some(5)];
        assert_eq!(v.find(10), 5);
    }

    #[test]
    fn union_works() {
        let v: DisjointSet = vec![None, None, Some(0)];
        assert_eq!(v.union(1, 2), vec![None, Some(0), Some(0)]);

        let v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None, Some(5),
                                  Some(0), Some(11), Some(8), Some(11), Some(6), Some(6), Some(2),
                                  None, Some(5)];
        assert_eq!(v.union(7, 4),
                   vec![Some(10), Some(2), Some(14), Some(1), Some(3), Some(14), Some(5),
                        Some(0), Some(11), Some(8), Some(11), Some(6), Some(6), Some(2), None,
                        Some(5)]);
    }

    #[test]
    fn compressed_find_works() {
        let mut v: DisjointSet = vec![None, None, Some(0)];
        assert_eq!(v.compressed_find(0), 0);
        assert_eq!(v, vec![None, None, Some(0)]);
        assert_eq!(v.compressed_find(1), 1);
        assert_eq!(v, vec![None, None, Some(0)]);
        assert_eq!(v.compressed_find(2), 0);
        assert_eq!(v, vec![None, None, Some(0)]);

        let mut v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None,
                                      Some(5), Some(0), Some(11), Some(8), Some(11), Some(6),
                                      Some(6), Some(2), None, Some(5)];
        assert_eq!(v.compressed_find(10), 5);
        assert_eq!(v,
                   vec![Some(10), Some(2), Some(14), Some(1), Some(3), None, Some(5), Some(0),
                        Some(11), Some(8), Some(5), Some(5), Some(6), Some(2), None, Some(5)]);
        assert_eq!(v.compressed_find(10), 5);
        assert_eq!(v,
                   vec![Some(10), Some(2), Some(14), Some(1), Some(3), None, Some(5), Some(0),
                        Some(11), Some(8), Some(5), Some(5), Some(6), Some(2), None, Some(5)]);

        let mut v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None,
                                      Some(5), Some(0), Some(11), Some(8), Some(11), Some(6),
                                      Some(6), Some(2), None, Some(5)];
        let same_v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None,
                                       Some(5), Some(0), Some(11), Some(8), Some(11), Some(6),
                                       Some(6), Some(2), None, Some(5)];
        // Assert compressed find is the same for all
        // i in the set.
        for i in 0..15 {
            assert_eq!(v.compressed_find(i), same_v.find(i));
        }

    }

    #[test]
    fn compressed_union_works() {
        let mut v: DisjointSet = vec![None, None, Some(0)];
        v.compressed_union(1, 2);
        assert_eq!(v, vec![None, Some(0), Some(0)]);

        let mut v: DisjointSet = vec![Some(10), Some(2), Some(14), Some(1), Some(3), None,
                                      Some(5), Some(0), Some(11), Some(8), Some(11), Some(6),
                                      Some(6), Some(2), None, Some(5)];
        v.compressed_union(7, 4);
        assert_eq!(v,
                   vec![Some(5), Some(14), Some(14), Some(14), Some(14), None, Some(5), Some(5),
                        Some(11), Some(8), Some(5), Some(5), Some(6), Some(2), Some(5), Some(5)]);
    }

}
