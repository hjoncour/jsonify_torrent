use std::{fs::File, io::Read};

use clap::{App, Arg};
use torrent::open_torrent;

fn main() {
    let matches = App::new("JSONIFY TORRNET FILE")
    .version("0.1")
    .author("@hjoncour")
    .about("Small cli app to convert .torrent to .json")
    .arg(Arg::with_name("path"))
    .get_matches();

    let file_path = matches.value_of("path").unwrap();

    let mut file = match File::open(file_path) {
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

  println!("{:#?}", torrent);

}
