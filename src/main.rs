// Copyright (c) 2021 Nathaniel Clark

use clap::Parser;

/// Geothmetic Meandian
///
/// [Arithmetic Mean, Geometric Mean, Median]
#[derive(Debug, Parser)]
#[clap(name = "gmdn", about = "Geothmetic Meandian")]
struct Opt {
    /// Maximum Iterations
    #[clap(short, long)]
    iterations: Option<u32>,

    /// Matching decimal places
    #[clap(short, long, default_value = "4")]
    precision: u8,

    /// Enable debug
    #[clap(short, long)]
    debug: bool,

    /// Print info about final answer
    #[clap(short, long)]
    verbose: bool,

    /// Floating point numbers run progression on
    #[clap(required = true)]
    numbers: Vec<f64>,
}

fn amean(v: &Vec<f64>) -> f64 {
    let n = v.len() as u32;
    assert!(n > 0);
    v.iter().fold(0.0, |acc, x| acc + *x) / (n as f64)
}

fn gmean(v: &Vec<f64>) -> f64 {
    let n = v.len() as f64;
    assert!(n > 0.0);
    v.iter().fold(1.0, |acc, x| acc * x).powf(1.0 / n)
}

fn meadian(v: &Vec<f64>) -> f64 {
    let n = v.len();
    assert!(n > 0);
    let h = (n + n % 2) / 2;
    let mut v2 = v.clone();
    v2.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    *v2.get(h - 1).unwrap()
}

fn f(v: &Vec<f64>) -> Vec<f64> {
    vec![amean(v), gmean(v), meadian(v)]
}

fn main() {
    let opt = Opt::parse();
    let mut v = opt.numbers.clone();

    let mut n = 0;
    let prec = (10.0_f64).powi(opt.precision as i32);
    loop {
        n += 1;
        v = f(&v);
        if opt.debug {
            println!("{}:Gmdn() = {:?}", n, v);
        }
        if opt.iterations.map(|x| x <= n).unwrap_or_default()
            || v.iter()
                .map(|x| (x * prec).trunc())
                .collect::<Vec<_>>()
                .windows(2)
                .all(|w| w[0] == w[1])
        {
            break;
        }
    }
    if opt.verbose {
        println!("Gmdn({:?}) iterations:{n} -> {:?}", opt.numbers, v);
    }
    println!("{:.*}", opt.precision as usize, v[0]);
}
