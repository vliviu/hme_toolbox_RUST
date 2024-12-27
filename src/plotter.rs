// src/plotter.rs
use plotters::prelude::*;

pub fn plot_clusters(clusters: Vec<Vec<(f64, f64)>>) {
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
