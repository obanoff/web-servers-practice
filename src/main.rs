
fn mode(m: &str, threads: usize) {
    match m {
        "multi threaded" => {
            use web_server::multi_threaded;

            multi_threaded::run(threads);
        },
        "async" => {
            println!("Missing implementation");
        },
        _ => (),
    }
}

macro_rules! mode {
    ($x: expr) => {
        mode($x, 4) // in a case of only the first argument provided, set the second argument to 4 (default parameter)
    };
    ($x: expr, $y: expr) => {
        mode($x, $y) // a case of both arguments provided
    };
}

fn main() {
    mode!("multi threaded");
}

























