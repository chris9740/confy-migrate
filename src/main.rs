mod config;

fn main() {
    let config = config::get_config().unwrap();

    println!("{config:#?}");
}
