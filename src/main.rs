use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_title() -> String {
  let mut the_title = String::from(env!("CARGO_PKG_NAME"));
  the_title.push_str(" (v");
  the_title.push_str(env!("CARGO_PKG_VERSION"));
  the_title.push_str("), ");
  the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
  the_title
}

fn parse_markdown_file(_filename: &str) {
  print_short_banner();
  println!("[ INFO ] Starting parser!");

  // Create a path variable from the filename
  let input_filename = Path::new(_filename);

  // Try to open the file
  let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

  // HTML elements
  let mut _ptag: bool = false;
  let mut _htag: bool = false;

  // Create a place to store all our tokens
  let mut tokens: Vec<String> = Vec::new();

  // Read the file line-by-line
  let reader = BufReader::new(file);

  // Loop through the reader lines
  for line in reader.lines() {
    // For each line, unwrap it
    let line_contents = line.unwrap();

    let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
    let mut output_line = String::new();

    match first_char.pop() {
      // The first character is #
      Some('#') => {
        if _ptag {
          _ptag = false;
          output_line.push_str("</p>\n"); // Adding \n for instructional clarity
        }

        if _htag {
          _htag = false;
          output_line.push_str("</h1>\n"); // Close it if we're already open
        }

        _htag = true;
        output_line.push_str("<h1>");
        output_line.push_str(&line_contents[2..]); // Get all but the first two characters
      }
      // The first character is not #
      _ => {
        if !_ptag {
          _ptag = true;
          output_line.push_str("<p>");
        }

        output_line.push_str(&line_contents);
      }
    }

    if _ptag {
      _ptag = false;
      output_line.push_str("</p>\n");
    }

    if _htag {
      _htag = false;
      output_line.push_str("</h1>\n");
    }

    if output_line != "<p></p>\n" {
      tokens.push(output_line)
    }
  } // end of "for line in reader.lines()" block
}

fn print_short_banner() {
  println!("{}", get_title());
}

fn print_long_banner() {
  print_short_banner();
  println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
  println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
  println!("Usage: tinymd <somefile.md>");
}

fn usage() {
  print_long_banner();
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  match args.len() {
    2 => parse_markdown_file(&args[1]),
    _ => {
      println!("[ ERROR ] You forgot to specify the markdown file to parse!");
      usage();
    }
  }
}
