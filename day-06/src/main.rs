
fn main() {
    let times: Vec<f64> = vec![60808676f64];
    let distances: Vec<f64> = vec![601116315591300f64];

    let mut ret : f64= 1.0;

    for i in 0..1 {
        let t_squared = times[i] * times[i];
        let four_d = 4.0 * distances[i];

        let diff = t_squared - four_d;
        let mut lower = 0.5 * (times[i] - diff.sqrt()) + 1.0;
        lower = lower.floor();
        let upper = times[i] - lower;
        let num = upper - lower + 1.0;

        ret = ret * num;
    }
    print!("{ret}")
}
