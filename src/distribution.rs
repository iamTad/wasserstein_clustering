use std::process;

#[derive(Debug)]
pub struct EmpiricalDistribution(pub Vec<f64>);

impl EmpiricalDistribution {
    /// Takes the empirical distribution and creates a CDF
    pub fn get_cdf(&self) -> Vec<f64>{
        let norm_factor: f64 = self.0.iter().sum(); // how much to normalize empirical values
        let mut cdf_point: f64 = 0.0; // keep track of previous value; to be incremented like a cdf
        self.0
            .iter()
            .map(|value| {cdf_point += value; cdf_point/norm_factor})
            .collect()
    }
}

/// This function takes 2 cdfs and returns a noremalized wasserstein distance.
/// The distance is the sum of |distances| between all corresponding points
/// between 2 cumulative distributions.
/// The normalization is done by dividing by the number of points in the distribution.
/// This allows one to compare distro distances between different data sources, though
/// I am unsure about whether that's appropriate to do in the first place (e.g. compare
/// distance between different gene coverage patterns)
pub fn calc_wasserstein_distance_1d(cdf_a: Vec<f64>, cdf_b: Vec<f64>) -> f64 {
    // initialize values 
    let norm_distance: f64 = if cdf_a.len() == cdf_b.len() { // the length of vectors must be the same
        cdf_a.len() as f64
    } else {
        println!("Not all vectors are the same size.");
        process::exit(1)
    };
    let mut distance: f64 = 0.0;
    // calculate distances
    for (a,b) in cdf_a.iter().zip(cdf_b.iter()) {
        distance += f64::abs(a-b);
    }
    // return normalized distance
    distance/norm_distance
}