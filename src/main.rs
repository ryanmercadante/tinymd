use std::fs::File;
use std::io::Write;
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
  let mut _h2tag: bool = false;
  let mut _emtag: bool = false;

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

        if _h2tag {
          _h2tag = false;
          output_line.push_str("</h2>\n"); // Close it if we're already open
        }

        if line_contents[1..2].eq("#") {
          _h2tag = true;
          output_line.push_str("<h2>");
          let trimmed_heading = &line_contents[2..].trim_start();
          output_line.push_str(&trimmed_heading);
        } else {
          _htag = true;
          output_line.push_str("<h1>");
          let trimmed_heading = &line_contents[1..].trim_start();
          output_line.push_str(&trimmed_heading);
        }
      }
      Some('*') => {
        if _emtag {
          _emtag = false;
          output_line.push_str("</em>\n")
        }

        let second_char = &line_contents[1..2];
        let last_two_char = String::from(&line_contents[&line_contents.len() - 2..]);
        if second_char.ne(" ") {
          let last = &last_two_char[1..];
          let second_last = &last_two_char[..1];
          if last.eq("*") && second_last.ne(" ") {
            _emtag = true;
            output_line.push_str("<em>");
            output_line.push_str(&line_contents[1..&line_contents.len() - 1]);
          }
        }
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

    if _emtag {
      _emtag = false;
      output_line.push_str("</em>\n");
    }

    if _ptag {
      _ptag = false;
      output_line.push_str("</p>\n");
    }

    if _h2tag {
      _h2tag = false;
      output_line.push_str("</h2>\n");
    }

    if _htag {
      _htag = false;
      output_line.push_str("</h1>\n");
    }

    if output_line != "<p></p>\n" {
      tokens.push(output_line)
    }
  } // end of "for line in reader.lines()" block

  // Create output filename from input file and add .html extension
  let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
  output_filename.push_str(".html");

  let mut outfile = File::create(output_filename).expect("[ ERROR ] Could not create output file!");

  // Loop through tokens and write each line as bytes to outfile
  for line in &tokens {
    outfile
      .write_all(line.as_bytes())
      .expect("[ ERROR ] Could not write to output file!");
  }

  println!("[ INFO ] Parsing complete!");
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
