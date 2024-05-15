pub struct EmpiricalDistribution(Vec<f64>);

impl EmpiricalDistribution {
    fn get_cdf(&self) -> Vec<f64>{
        let mut cdf: Vec<f64> = Vec::new();
        let norm_factor: f64 = self.0.iter().sum(); // how much to normalize empirical values
        let mut cdf_point: f64 = 0.0; // keep track of previous value; to be incremented like a cdf
        self.0
            .iter()
            .map(|value| {cdf_point += value; cdf_point/norm_factor})
            .collect()
    }
}