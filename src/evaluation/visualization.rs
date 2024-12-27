use plotters::prelude::*;

pub fn plot_clusters(clusters: &Vec<Vec<Vec<f64>>>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output, (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Cluster Visualization", ("sans-serif", 20))
        .build_cartesian_2d(-10.0..10.0, -10.0..10.0)?;

    chart.configure_mesh().draw()?;

    for cluster in clusters {
        let points: Vec<(f64, f64)> = cluster.iter().map(|v| (v[0], v[1])).collect();
        chart.draw_series(points.iter().map(|p| Circle::new(*p, 3, BLUE.filled())))?;
    }

    Ok(())
}
