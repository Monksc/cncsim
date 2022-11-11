mod utils;

use std::io;

use std::process::Command;

use clap::Parser;
use std::fs;

use rand::random;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   input: String,
   #[clap(short, long, value_parser)]
   output: String,
   #[clap(long, value_parser)]
   blockwidth: f64,
   #[clap(long, value_parser)]
   blockheight: f64,
   #[clap(long, value_parser)]
   imgwidth: u64,
   #[clap(long, value_parser)]
   imgheight: u64,
   #[clap(long, value_parser)]
   fnvalue: u64,
}

fn random_string() -> String {
    let characters: Vec<char> = "0123456789ABCDEF".chars().collect();
    let mut s = String::new();
    for _ in 0..16 {
        let r: usize = random();
        s.push(characters[r % characters.len()]);
    }

    return s;
}

fn get_temp_file(extension: &str) ->
    std::io::Result<(std::fs::File, std::path::PathBuf)> {
    let path = std::env::temp_dir().join(random_string() + extension);
    let file = std::fs::File::create(&path)
        .expect("Could not create a file");

    Ok((file, path))
}

fn main() -> io::Result<()> {

    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let (mut file, path) = get_temp_file(".scad")
        .expect("Could not create temporary file.");

    utils::tostl::to_scad(
        args.fnvalue,
        args.blockwidth,
        args.blockheight,
        vec![
            utils::cncrouter::Tool {
                radius: 0.25/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.125/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.0625/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.02/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.02/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.005/2.0,
                length: 1.0,
            },
            utils::cncrouter::Tool {
                radius: 0.005/2.0,
                length: 1.0,
            },
        ], &mut contents.chars(), &mut file)
        .expect("Could not read template or write to temporary file.");

    let file_path = path.into_os_string()
        .into_string()
        .expect("Path could not be found.");

    println!("STL FILE: {}", file_path);

    let output = Command::new("/usr/bin/openscad")
        .arg("-o")
        .arg(args.output)
        // .arg("--autocenter")
        // .arg("--viewall")
        .arg(format!("--imgsize={},{}", args.imgwidth, args.imgheight))
        // .arg("--imgsize=16384,16384")
        .arg(&file_path)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Could not get stdout.");
    let stderr = String::from_utf8(output.stderr).expect("Could not get stderr.");
    println!("STDOUT:\n{}", stdout);
    print!("STDERR:\n{}", stderr);

    Ok(())
}
