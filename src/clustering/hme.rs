pub struct HMECluster {
    pub centroid: Vec<f64>,
    pub points: Vec<Vec<f64>>,
}

impl HMECluster {
    pub fn new(centroid: Vec<f64>) -> Self {
        HMECluster {
            centroid,
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: Vec<f64>) {
        self.points.push(point);
    }

    pub fn update_centroid(&mut self) {
        let dimension = self.centroid.len();
        let mut new_centroid = vec![0.0; dimension];

        for point in &self.points {
            for (i, &val) in point.iter().enumerate() {
                new_centroid[i] += val;
            }
        }

        if !self.points.is_empty() {
            for i in 0..dimension {
                new_centroid[i] /= self.points.len() as f64;
            }
        }

        self.centroid = new_centroid;
    }
}
// We extend the existing HMECluster to support recursive sub-clusters.
pub struct HierarchicalCluster {
    pub cluster: HMECluster,
    pub subclusters: Vec<HierarchicalCluster>,
    pub max_depth: usize,
}

impl HierarchicalCluster {
    pub fn new(centroid: Vec<f64>, max_depth: usize) -> Self {
        HierarchicalCluster {
            cluster: HMECluster::new(centroid),
            subclusters: Vec::new(),
            max_depth,
        }
    }

    pub fn add_point(&mut self, point: Vec<f64>, depth: usize) {
        if depth < self.max_depth {
            // Assign point to closest subcluster or create new subcluster
            if let Some(sub) = self.subclusters.iter_mut().min_by(|a, b| {
                let da = euclidean_distance(&a.cluster.centroid, &point);
                let db = euclidean_distance(&b.cluster.centroid, &point);
                da.partial_cmp(&db).unwrap()
            }) {
                sub.add_point(point, depth + 1);
            } else {
                let mut new_cluster = HierarchicalCluster::new(point.clone(), self.max_depth);
                new_cluster.cluster.add_point(point);
                self.subclusters.push(new_cluster);
            }
        } else {
            self.cluster.add_point(point);
        }
    }

    pub fn update_centroids(&mut self) {
        for sub in &mut self.subclusters {
            sub.update_centroids();
        }
        self.cluster.update_centroid();
    }
}

fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}
