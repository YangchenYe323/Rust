use std::convert::TryInto;
/// This implementation of segment tree is built on
/// an array of i32, and supports the following operation:
/// 1. given an index i, j, query the sum of array in range [i, j]
/// 2. given a range [x, y] and a diff, update all values of the array
/// in range by diff
/// Any range sum query [i, j] can be easily answered by calling query(j) - query(i)
///
///
pub struct RangeSumSegmentTree {
    // store total range [1, len]
    len: usize,
    // representation of the tree, where child of arr[p]
    // is child arr[p * 2] and arr[p * 2 + 1]
    arr: Vec<i32>,
    // implement lazy propagation
    // mark[p] stores the diffs that have been applied
    // to node p but has not been propagated to its child node
    mark: Vec<i32>,
}

impl RangeSumSegmentTree {
    /// build tree from an array of values
    pub fn from_vec(values: &[i32]) -> Self {
        let n = values.len();
        // our arr is 1-indexed
        let length = calculate_length(n);
        let mut tree = Self {
            len: n,
            arr: vec![0; length],
            mark: vec![0; length],
        };

        tree.build_rec(values, 1, n, 1);

        tree
    }

    fn build_rec(&mut self, values: &[i32], left: usize, right: usize, p: usize) {
        if left == right {
            self.arr[p] = values[left - 1];
            return;
        }
        let mid = left + (right - left) / 2;
        self.build_rec(values, left, mid, p * 2);
        self.build_rec(values, mid + 1, right, p * 2 + 1);
        self.arr[p] = self.arr[p * 2] + self.arr[p * 2 + 1];
    }

    /// add diff to all element in range [i, j]
    pub fn update(&mut self, i: usize, j: usize, diff: i32) {
        self.update_rec(i, j, 1, self.len, 1, diff)
    }

    fn update_rec(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize, diff: i32) {
        // no intersection of current segment and target segment
        if cl > r || cr < l {
            return;
        }

        // current segment is contained in target segment
        if cl >= l && cr <= r {
            self.arr[p] += diff * (cr - cl + 1) as i32;
            if l < r {
                self.mark[p] += diff;
            }
            return;
        }

        self.push_down(p, (cr - cl + 1) as i32);

        let mid = cl + (cr - cl) / 2;
        self.update_rec(l, r, cl, mid, p * 2, diff);
        self.update_rec(l, r, mid + 1, cr, p * 2 + 1, diff);

        self.arr[p] = self.arr[p * 2] + self.arr[p * 2 + 1];
    }

    fn push_down(&mut self, p: usize, length: i32) {
        self.mark[p * 2] += self.mark[p];
        self.mark[p * 2 + 1] += self.mark[p];
        self.arr[p * 2] += self.mark[p] * ((length + 1) / 2);
        self.arr[p * 2 + 1] += self.mark[p] * (length / 2);
        self.mark[p] = 0;
    }

    /// return the range sum of array[i]..array[j] inclusive
    pub fn query(&mut self, i: usize, j: usize) -> i32 {
        self.query_rec(i, j, 1, self.len, 1)
    }

    fn query_rec(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize) -> i32 {
        // no intersection of current segment and target segment
        if cl > r || cr < l {
            return 0;
        }
        // current segment is contained in target segment
        if cl >= l && cr <= r {
            return self.arr[p];
        }
        // push down
        self.push_down(p, (cr - cl + 1) as i32);
        let mid = cl + (cr - cl) / 2;
        self.query_rec(l, r, cl, mid, p * 2) + self.query_rec(l, r, mid + 1, cr, p * 2 + 1)
    }
}

// calculate the length needed for
// a segmentree covering range [1, n]
fn calculate_length(n: usize) -> usize {
    let mut h = 1;
    let mut cur = n;
    while cur != 1 {
        cur = (cur + 1) / 2;
        h += 1;
    }
    2usize.pow(h.try_into().unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_query() {
        let values = [1, 2, 3, 4, 5, 6];
        let mut seg_tree = RangeSumSegmentTree::from_vec(&values);
        assert_eq!(21, seg_tree.query(1, 6));
        assert_eq!(5, seg_tree.query(2, 3));
        assert_eq!(4, seg_tree.query(4, 4));
        assert_eq!(12, seg_tree.query(3, 5));
    }

    #[test]
    fn test_update() {
        let values = [2, 4, 1, 3, 5, 7];
        let mut seg_tree = RangeSumSegmentTree::from_vec(&values);

        assert_eq!(8, seg_tree.query(2, 4));

        // update
        seg_tree.update(2, 4, 1);
        // new values should be [2, 5, 2, 4, 5, 7]
        assert_eq!(11, seg_tree.query(2, 4));
        assert_eq!(5, seg_tree.query(2, 2));
        assert_eq!(2, seg_tree.query(3, 3));
        assert_eq!(4, seg_tree.query(4, 4));
        assert_eq!(9, seg_tree.query(1, 3));

        seg_tree.update(3, 6, -2);
        // new values should be [2, 5, 0, 2, 3, 5]
        assert_eq!(8, seg_tree.query(5, 6));
        assert_eq!(17, seg_tree.query(0, 6));
    }

    #[test]
    fn test_build() {
        for length in 10..10000 {
            let values = vec![2; length];
            let _seg_tree = RangeSumSegmentTree::from_vec(&values[..]);
        }
    }
}
