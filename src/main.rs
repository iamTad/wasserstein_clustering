use wasserstein_clustering::distribution;

fn main() {
    let v = vec![1.1, 2.2, 3.3];
    let distro = distribution::EmpiricalDistribution(v);
    let cdf = distro.get_cdf();

    // test calculation
    //let u = vec![5.0, 5.0, 0.5];
    //let d = distribution::EmpiricalDistribution(u);
    //let c = d.get_cdf();
    //let answer: f64 = distribution::calc_wasserstein_distance_1d(cdf, c);
    //println!("{:?}", answer);
}
