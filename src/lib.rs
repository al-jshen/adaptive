use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
pub struct Subdiv {
    left: f64,
    right: f64,
    f_left: f64,
    f_right: f64,
    loss: f64,
}

impl PartialEq for Subdiv {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.right == other.right
            && self.f_left == other.f_left
            && self.f_right == other.f_right
            && self.loss == other.loss
    }
}

impl Eq for Subdiv {}

impl PartialOrd for Subdiv {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.loss.partial_cmp(&other.loss)
    }
}

impl Ord for Subdiv {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.loss.partial_cmp(&other.loss).unwrap()
    }
}

pub trait SubdivLoss {
    fn loss(&self, x1: f64, x2: f64, y1: f64, y2: f64) -> f64;
}

pub struct EuclideanLoss {}

impl SubdivLoss for EuclideanLoss {
    fn loss(&self, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
        (y2 - y1).powi(2) + (x2 - x1).powi(2)
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveSampler<F, L>
where
    F: Fn(f64) -> f64,
    L: SubdivLoss,
{
    f: F,
    l: L,
    pq: BinaryHeap<Subdiv>,
}

impl<F, L> AdaptiveSampler<F, L>
where
    F: Fn(f64) -> f64,
    L: SubdivLoss,
{
    pub fn eval(&self, x: f64) -> f64 {
        (self.f)(x)
    }

    pub fn new(f: F, l: L, lower: f64, upper: f64) -> Self {
        let mut pq = BinaryHeap::new();

        let f_lower = f(lower);
        let f_upper = f(upper);
        let sd = Subdiv {
            left: lower,
            right: upper,
            f_left: f_lower,
            f_right: f_upper,
            loss: 1.,
        };
        pq.push(sd);

        AdaptiveSampler { f, l, pq }
    }

    pub fn next(&mut self) -> (f64, f64) {
        let max = self.pq.pop().unwrap();

        let mid = (max.left + max.right) / 2.;
        let f_mid = self.eval(mid);
        let loss_left = self.l.loss(max.left, mid, max.f_left, f_mid);
        let loss_right = self.l.loss(mid, max.right, f_mid, max.f_right);

        let left = Subdiv {
            left: max.left,
            right: mid,
            f_left: max.f_left,
            f_right: f_mid,
            loss: loss_left,
        };

        let right = Subdiv {
            left: mid,
            right: max.right,
            f_left: f_mid,
            f_right: max.f_right,
            loss: loss_right,
        };

        self.pq.push(left);
        self.pq.push(right);

        (mid, f_mid)
    }

    pub fn until(&mut self, tol: f64) -> Vec<(f64, f64)> {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut loss = f64::MAX;
        while loss > tol {
            let (x, y) = self.next();
            xs.push(x);
            ys.push(y);
            loss = self.pq.peek().unwrap().loss;
        }
        xs.into_iter().zip(ys).collect()
    }
}
