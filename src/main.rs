
fn main() {
    mode("multi threaded");
}


fn mode(m: &str) {
    match m {
        "multi threaded" => {
            use web_server::multi_threaded;

            multi_threaded::run(4);
        },
        "async" => {
            println!("Missing implementation");
        },
        _ => (),
    }
}

























