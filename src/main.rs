use std::path::Path;
use std::{fs::File, io::Read};

use clap::{App, Arg};
use torrent::open_torrent;
use torrent::encode::save_to_json_file;

fn main() {
  let matches = App::new("TORRENTIFY")
  .version("0.1")
  .author("@hjoncour")
  .about("Small cli app to parse and convert .torrent files")
  .arg(Arg::with_name("input_path")
      .required(true)
      .takes_value(true)
      .index(1)
      .help("Input path for the .torrent file"))
  .arg(Arg::with_name("output_path")
      .long("output")
      .short("o")
      .index(2)
      .takes_value(true)
      .required(false) // Make output_path optional
      .help("Output path for the file"))
  .arg(Arg::with_name("output_type")
      .long("type")
      .short("t")
      .index(3)
      .takes_value(true)
      .required(false) // Make output_type optional
      .help("Type of the output"))
  .get_matches();

    let input_path: &str = matches.value_of("input_path").unwrap();
    let output_path: Option<&str> = matches.value_of("output_path");
    let output_type: Option<&str> = matches.value_of("output_type");
    let output_name: String = Path::new(input_path).file_name().unwrap().to_str().unwrap().to_string();

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
  let _name = save_to_json_file(torrent.unwrap(), output_name, output_path, output_type);

}
