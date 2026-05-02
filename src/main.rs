use catr::get_args;

fn main() {
    if let Err(e) = catr::run(get_args().unwrap()) {
        eprintln!("{}", e);
    }
}
