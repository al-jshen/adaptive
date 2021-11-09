use adaptive::{AdaptiveSampler, EuclideanLoss};

fn main() {
    let f = |x: f64| x + 0.01 * 0.01 / (0.01 * 0.01 + x.powi(2));
    let mut sampler = AdaptiveSampler::new(f, EuclideanLoss {}, -1., 1.);
    for (x, y) in sampler.until(0.01) {
        println!("{},{}", x, y);
    }
}
