use rand::prelude::{thread_rng, Distribution};
type Torus = u32;

pub fn f2torus(src: f64) -> Torus {
    //f64 -> i32
    let mut seisuu: f64 = src - src.round();
    if seisuu < 0.0 {
        seisuu += 1.0;
    }
    (seisuu * (u32::MAX as f64)) as u32
}

pub fn d_ta(std_dev: f64) -> Torus {
    //choose from modular normal distribution
    let mut rng = thread_rng();
    let dist = rand_distr::Normal::<f64>::new(0.0, std_dev).unwrap();
    f2torus(dist.sample(&mut rng))
}
