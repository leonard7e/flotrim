use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = "0.1.0",
    about = "Calculate new length of a flute to approach the desired pitch",
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Calculate the trimmed length for a given pitch adjustment
    Trim(TrimArgs),
    /// Generate a table of flute lengths for chromatic semitone offsets
    Table(TableArgs),
}

#[derive(Parser, Debug)]
struct TrimArgs {
    /// The length of the flute in millimeters
    pub length: f64,
    /// The pitch difference in Cents
    pub tune: f64,
}

impl TrimArgs {
    pub fn validate(&self) -> Result<(), String> {
        Some(self.length)
            .filter(|&l| l > 0.0)
            .ok_or_else(|| "Length must be greater than zero.".to_string())?;

        Some(self.tune)
            .filter(|&t| t >= -2400.0 && t <= 2400.0)
            .ok_or_else(|| {
                "Tune adjustment must be between -2400.0 and 2400.0 cents (max +-2 octaves)."
                    .to_string()
            })?;

        Ok(())
    }
}

#[derive(Parser, Debug)]
struct TableArgs {
    /// The reference flute length in millimeters
    pub length: f64,
    /// Number of semitones above and below the reference
    #[arg(short, long, default_value_t = 6)]
    pub semitones: u32,
}

impl TableArgs {
    pub fn validate(&self) -> Result<(), String> {
        Some(self.length)
            .filter(|&l| l > 0.0)
            .ok_or_else(|| "Length must be greater than zero.".to_string())?;

        if self.semitones == 0 {
            return Err("Semitones must be at least 1.".to_string());
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Trim(args) => cmd_trim(args),
        Command::Table(args) => cmd_table(args),
    }
}

fn cmd_trim(args: TrimArgs) {
    if let Err(err) = args.validate() {
        use clap::CommandFactory;
        let mut cmd = TrimArgs::command();
        cmd.error(clap::error::ErrorKind::ValueValidation, err)
            .exit();
    }

    let new_length = args.length * (2.0f64.powf(-args.tune / 1200.0));
    println!("Trim your flute up to {:.2}mm.", new_length);
}

fn cmd_table(args: TableArgs) {
    if let Err(err) = args.validate() {
        use clap::CommandFactory;
        let mut cmd = TableArgs::command();
        cmd.error(clap::error::ErrorKind::ValueValidation, err)
            .exit();
    }

    let n = args.semitones as i32;
    println!("{:<12} {:<12}", "Semitone", "Length (mm)");
    println!("{:-<12} {:-<12}", "", "");

    for s in -n..=n {
        let cents = s as f64 * 100.0;
        let length = args.length * (2.0f64.powf(-cents / 1200.0));
        // let diff = length - args.length;
        // println!("{:<10} {:<12.0} {:<12.2} {:>+10.2}", s, cents, length);
        println!("{:<12} {:<12.2}", s, length);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_valid_args() {
        let args = TrimArgs {
            length: 300.0,
            tune: 100.0,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_trim_invalid_length() {
        let args = TrimArgs {
            length: 0.0,
            tune: 100.0,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Length must be greater than zero."
        );

        let args = TrimArgs {
            length: -100.0,
            tune: 100.0,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Length must be greater than zero."
        );
    }

    #[test]
    fn test_trim_invalid_tune() {
        let args = TrimArgs {
            length: 300.0,
            tune: 2500.0,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Tune adjustment must be between -2400.0 and 2400.0 cents (max +-2 octaves)."
        );

        let args = TrimArgs {
            length: 300.0,
            tune: -2500.0,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Tune adjustment must be between -2400.0 and 2400.0 cents (max +-2 octaves)."
        );
    }

    #[test]
    fn test_table_valid_args() {
        let args = TableArgs {
            length: 300.0,
            semitones: 12,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_table_invalid_length() {
        let args = TableArgs {
            length: 0.0,
            semitones: 12,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Length must be greater than zero."
        );
    }

    #[test]
    fn test_table_invalid_semitones() {
        let args = TableArgs {
            length: 300.0,
            semitones: 0,
        };
        assert_eq!(
            args.validate().unwrap_err(),
            "Semitones must be at least 1."
        );
    }
}
