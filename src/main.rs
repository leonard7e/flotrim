use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = "0.1.0",
    about = "Calculate new length of a flute to approach the desired pitch",
    long_about = None)]
struct Args {
    /// The length of the flute in millimeters
    pub length: f64,
    /// The pitch difference in Cents
    pub tune: f64,
}

fn main() {
    let Args { length, tune } = Args::parse();
    let new_length = length * (2.0f64.powf(-tune / 1200.0));
    println!("Trim your flute up to {}mm.", new_length);
}
