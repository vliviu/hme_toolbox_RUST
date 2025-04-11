use plotters::prelude::*;
use std::fs::File;
use std::io::{self, BufReader}; // Ensure you have plotters dependency

use hme_toolbox::clustering::cluster_sequences;
use hme_toolbox::clustering::hme::HMECluster;
use hme_toolbox::parser::parse_fasta;
//use hme_toolbox::plotter::plot_clusters;

use hme_toolbox::plotter::plot_clusters as imported_plot_clusters;

#[test]
fn test_with_large_fasta() {
    let fasta_file = "tests/larger.fasta"; // Adjust the path to your file.
    let sequences = parse_fasta(fasta_file).expect("Failed to parse FASTA file");

    let clusters = cluster_sequences(&sequences);
    plot_clusters(clusters);

    assert!(
        std::path::Path::new("clusters.png").exists(),
        "Plot file was not generated."
    );
}

// This is now a regular utility function, not a test function.
fn plot_clusters(clusters: Vec<Vec<(f64, f64)>>) {
    let root = BitMapBackend::new("clusters.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Cluster Visualization", ("sans-serif", 40))
        .build_cartesian_2d(0.0..100.0, 0.0..100.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for cluster in clusters {
        let points: Vec<(f64, f64)> = cluster.into_iter().collect();
        chart
            .draw_series(PointSeries::of_element(
                points,
                5, // Marker size
                &RED,
                &|c, s, v| Circle::new(c, s, v.filled()),
            ))
            .unwrap();
    }
}

/* // Your test function that has no arguments.
#[test]
fn test_with_large_fasta() {
    let fasta_file = "tests/larger.fasta"; // Adjust the path to your file.
    let sequences = parse_fasta(fasta_file).expect("Failed to parse FASTA file");

    // Perform clustering (example function, adjust based on your actual code)
    let clusters = cluster_sequences(&sequences);

    // For now, just print the number of clusters as a simple check
    println!("Number of clusters: {}", clusters.len());

    // Visualize the clusters by calling plot_clusters
    plot_clusters(clusters);

    // Check if the plot file exists (optional)
    assert!(std::path::Path::new("clusters.png").exists(), "Plot file was not generated.");
} */
