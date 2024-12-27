use rayon::prelude::*;
use super::hme::HierarchicalCluster;

impl HierarchicalCluster {
    pub fn parallel_update(&mut self) {
        self.subclusters.par_iter_mut().for_each(|sub| {
            sub.parallel_update();
        });
        self.cluster.update_centroid();
    }
}
