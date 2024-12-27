pub fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}

pub fn cosine_similarity(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let norm_b = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    dot_product / (norm_a * norm_b)
}
