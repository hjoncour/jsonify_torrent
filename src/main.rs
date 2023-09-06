use std::{fs::File, io::Read};

use clap::{App, Arg};
use torrent::open_torrent;
use torrent::encode::save_to_json_file;

fn main() {
    let matches = App::new("TORRENTIFY")
        .version("0.1")
        .author("@hjoncour")
        .about("Small cli app to convert .torrent to .json")
        .arg(Arg::with_name("input_path")
        .required(true)
        .takes_value(true)
        .index(1)
        .help("Input path for the .torrent file"))
    .arg(Arg::with_name("output_path")
        .required(false)
        .takes_value(true)
        .index(2)
        .help("Output path for the file"))
    .arg(Arg::with_name("output_type")
        .required(false)
        .takes_value(true)
        .index(3)
        .help("Type of the output"))
    .get_matches();

    let input_path = matches.value_of("input_path").unwrap();
    let output_path: Option<&str> = matches.value_of("output_path");
    let output_type = matches.value_of("output_type");

    let mut file = match File::open(input_path) {
      Ok(file) => file,
      Err(err) => {
          eprintln!("Error opening file: {}", err);
          return;
      }
  };

  let mut file_contents = Vec::new();
  if let Err(err) = file.read_to_end(&mut file_contents) {
      eprintln!("Error reading file: {}", err);
      return;
  }

  let torrent = open_torrent(&file_contents);
  let name = save_to_json_file(torrent.unwrap(), output_path, output_type);
  println!("here0");
  println!("name: {}", name.unwrap());

}
