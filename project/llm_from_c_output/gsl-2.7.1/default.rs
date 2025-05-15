use std::env;
use std::error::Error;
use std::fmt;
use std::process;

#[derive(Debug)]
struct GslError {
    message: String,
    code: i32,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for GslError {}

const GSL_EINVAL: i32 = 1;

struct RngType {
    name: &'static str,
}

static GSL_RNG_MT19937: RngType = RngType {
    name: "mt19937",
};

struct RngEnv {
    rng_type: &'static RngType,
    seed: u64,
}

static mut GSL_RNG_DEFAULT: Option<&'static RngType> = None;
static mut GSL_RNG_DEFAULT_SEED: u64 = 0;

fn gsl_rng_types_setup() -> Vec<&'static RngType> {
    vec![&GSL_RNG_MT19937]
}

fn gsl_rng_env_setup() -> Result<&'static RngType, Box<dyn Error>> {
    let mut seed = 0u64;
    let rng_type = match env::var("GSL_RNG_TYPE") {
        Ok(p) => {
            let t0 = gsl_rng_types_setup();
            let mut default = None;

            for t in &t0 {
                if p == t.name {
                    default = Some(*t);
                    break;
                }
            }

            match default {
                Some(t) => {
                    eprintln!("GSL_RNG_TYPE={}", t.name);
                    t
                }
                None => {
                    eprintln!("GSL_RNG_TYPE={} not recognized", p);
                    eprintln!("Valid generator types are:");
                    
                    let mut i = 0;
                    for t in &t0 {
                        eprint!(" {:18}", t.name);
                        i += 1;
                        if i % 4 == 0 {
                            eprintln!();
                        }
                    }
                    eprintln!();
                    
                    return Err(Box::new(GslError {
                        message: "unknown generator".to_string(),
                        code: GSL_EINVAL,
                    }));
                }
            }
        }
        Err(_) => {
            &GSL_RNG_MT19937
        }
    };

    if let Ok(p) = env::var("GSL_RNG_SEED") {
        seed = p.parse::<u64>().unwrap_or(0);
        eprintln!("GSL_RNG_SEED={}", seed);
    }

    unsafe {
        GSL_RNG_DEFAULT = Some(rng_type);
        GSL_RNG_DEFAULT_SEED = seed;
    }

    Ok(rng_type)
}

fn main() {
    match gsl_rng_env_setup() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}