use hme_toolbox::clustering::hme::HMECluster;
use hme_toolbox::clustering::hme::HierarchicalCluster;

#[test]
fn test_hme_cluster() {
    let mut cluster = HMECluster::new(vec![0.0, 0.0]);
    cluster.add_point(vec![1.0, 1.0]);
    cluster.add_point(vec![2.0, 2.0]);

    cluster.update_centroid();
    assert_eq!(cluster.centroid, vec![1.5, 1.5]);
}

#[test]
fn test_hierarchical_clustering() {
    let mut cluster = HierarchicalCluster::new(vec![0.0, 0.0], 2);
    cluster.add_point(vec![1.0, 1.0], 0);
    cluster.add_point(vec![2.0, 2.0], 0);
    cluster.add_point(vec![5.0, 5.0], 0); // Should create sub-cluster

    cluster.update_centroids();
    assert!(cluster.subclusters.len() > 0);
}
