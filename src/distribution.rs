#[derive(Debug)]
pub struct EmpiricalDistribution(pub Vec<f64>);

impl EmpiricalDistribution {
    pub fn get_cdf(&self) -> Vec<f64>{
        /// Takes the empirical distribution and creates a CDF
        let norm_factor: f64 = self.0.iter().sum(); // how much to normalize empirical values
        let mut cdf_point: f64 = 0.0; // keep track of previous value; to be incremented like a cdf
        self.0
            .iter()
            .map(|value| {cdf_point += value; cdf_point/norm_factor})
            .collect()
    }
}


pub fn calc_wasserstein_distance_1d(cdf_a: Vec<f64>, cdf_b: Vec<f64>) {
    let norm_distance: u32 = if cdf_a.len() == cdf_b.len() { // the length of vectors must be the same
        cdf_a.len()
    } else {
        1 // THIS IS JUST A PLACEHOLDER
    }
    let distance: f64 = 0;
}