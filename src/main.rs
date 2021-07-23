fn get_version() -> u16 {
    1000
}

fn usage() {
    println!("tinymd, a markdown compiler written by Ryan Mercadante");
    println!("Version {}", get_version());
}

fn main() {
    usage();
}
