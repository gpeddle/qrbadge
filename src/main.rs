use std::fs;

extern crate minidom;
use structopt::StructOpt;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the config file
    #[structopt( long = "spec", parse(from_os_str))]
    spec_file:  std::path::PathBuf,

    /// The path to the file of data
    #[structopt( long = "input", parse(from_os_str))]
    input_file: std::path::PathBuf,

    /// The path to output files
    #[structopt( long = "output", parse(from_os_str))]
    output_dir: std::path::PathBuf,
}


const _QRSPEC_NS: &'static str = "qrspec";


#[derive(Debug)]
pub struct QrSpec {

    file_pattern: String,

    title: String,
    body: String,
}

#[derive(Debug)]
pub enum Unit {
    Inches,
    Millimeters
}

#[derive(Debug)]
pub struct QrDocument {

    units: Unit,
    width: f32,
    height: f32,
    margin: f32
}





fn main() {

    
    let args = Cli::from_args();

 

    let contents = fs::read_to_string(args.spec_file)
        .expect("Error while reading the spec file");
    

    let root: minidom::Element = contents.parse().unwrap();
    println!("{:#?}", root);
    //let input_file = std::env::args().nth(2).expect("no input data file provided");
    //let output_dir = std::env::args().nth(3).expect("no output dir provided");

    //let qq: QrDocument = new_qr_document();

    let _data = fs::read_to_string(args.input_file)
    .expect("Error while reading the input file");

    let _xx = args.output_dir;

    
}   