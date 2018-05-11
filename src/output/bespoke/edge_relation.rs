use crate::facts::Region;
use crate::output::bespoke::SubsetRelation;
use fxhash::FxHashSet;
use matrix_relation::bitvec::SparseBitSet;
use relation::vec_family::StdVec;
use relation::Relation;
use std::collections::BTreeSet;

pub struct EdgeSubsetRelation {
    data: Relation<StdVec<Region>>,
}

impl Clone for EdgeSubsetRelation {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl SubsetRelation for EdgeSubsetRelation {
    fn empty(num_regions: usize) -> Self {
        Self {
            data: Relation::new(num_regions),
        }
    }

    fn kill_region(&mut self, _live_regions: impl Iterator<Item = Region>, dead_regions: &SparseBitSet<Region>) {
        for r in dead_regions.iter() {
            self.data.remove_edges(r);
        }
    }

    fn insert_one(&mut self, r1: Region, r2: Region) -> bool {
        self.data.add_edge(r1, r2)
    }

    fn insert_all(&mut self, other: &Self, live_regions: &BTreeSet<Region>) -> bool {
        let mut changed = false;
        for &r in live_regions {
            for succ_r in other.data.successors(r) {
                changed |= self.data.add_edge(r, succ_r);
            }
        }
        changed
    }

    fn for_each_reachable(&self, r1: Region, mut op: impl FnMut(Region)) {
        let mut stack = vec![r1];
        let mut visited = FxHashSet::default();
        visited.insert(r1);

        while let Some(p) = stack.pop() {
            op(p);
            for s in self.data.successors(p) {
                if visited.insert(s) {
                    stack.push(s);
                }
            }
        }
    }
}