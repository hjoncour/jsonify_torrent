use clap::{App, Arg};

fn main() {
    let matches = App::new("JSONIFY TORRNET FILE")
    .version("0.1")
    .author("@hjoncour")
    .about("Small cli app to convert .torrent to .json")
    .arg(Arg::with_name("path"))
    .get_matches();

println!("{}: ", &matches.value_of("path").unwrap());
}