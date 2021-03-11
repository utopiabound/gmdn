use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gmdn", about = "Geothmetic Meandian.")]
struct Opt {
    /// Iterations
    #[structopt(short, long, default_value = "1")]
    iterations: u32,

    /// Enable debug
    #[structopt(short, long)]
    debug: bool,

    /// Floating point numbers run progression on
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
    v.iter().fold(1.0, |acc, x| acc * x).powf(1.0/n)
}

fn meadian(v: &Vec<f64>) -> f64 {
    let n = v.len();
    assert!(n > 0);
    let h = (n + n % 2) / 2;
    let mut v2 = v.clone();
    v2.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    *v2.get(h-1).unwrap()
}

fn f(v: &Vec<f64>) -> Vec<f64> {
    vec![amean(v), gmean(v), meadian(v)]
}

fn main() {
    let opt = Opt::from_args();
    let mut v = opt.numbers.clone();
    
    for n in 1..=opt.iterations {
        v = f(&v);
        if opt.debug {
            println!("{}:Gmdn() = {:?}", n, v);
        }
    }
    println!("Gmdn({:?}) = {:?}", opt.numbers, v);
}
