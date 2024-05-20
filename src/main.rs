use wasserstein_clustering::distribution;

fn main() {
    let v = vec![1.1, 2.2, 3.3];
    let distro = distribution::EmpiricalDistribution(v);
    let cdf = distro.get_cdf();
    println!("{:?}", cdf);
}
