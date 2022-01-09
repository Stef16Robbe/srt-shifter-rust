use clap::{Arg, App};
use strum_macros::EnumString;
use std::{io::{BufReader, BufRead, Write}, fs::File, num::ParseIntError, str::FromStr, fmt::Display};

#[derive(Debug, EnumString)]
enum Operation {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus
}

#[derive(Debug)]
struct TimeStamp {
    hours: u32,
    minutes: u32,
    seconds: u32,
    miliseconds: u32
}

impl FromStr for TimeStamp {
    // 0:00:04,280
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://www.dotnetperls.com/substring-rust
        let hours = s[..1].parse().unwrap();
        let minutes = s[3..4].parse().unwrap();
        let seconds = s[6..7].parse().unwrap();
        let miliseconds = s[s.len()-3..].parse().unwrap();

        Ok(TimeStamp { hours, minutes, seconds, miliseconds })
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://stackoverflow.com/q/41820818/10503012
        write!(f, "{:01}:{:02}:{:02},{:03}", self.hours, self.minutes, self.seconds, self.miliseconds)
    }
}

fn main() {
    // (\d+):(\d\d):(\d\d)[,:]?(\d*){0,3}

    // https://stackoverflow.com/a/60458834/10503012
    let matches = App::new("SRT Shifter")
                                    .version("0.1")
                                    .author("Stef16Robbe <stef.robbe@gmail.com>")
                                    .about("Shift SRT subtitle timings")
                                    .arg(Arg::new("file")
                                        .short('f')
                                        .long("file")
                                        .value_name("FILENAME")
                                        .help("Define the .srt file to be edited")
                                        .required(true)
                                        .takes_value(true))
                                    .arg(Arg::new("seconds")
                                        .short('s')
                                        .long("seconds")
                                        .value_name("SECONDS")
                                        .help("Set the seconds to shift each time by")
                                        .required(true)
                                        .takes_value(true))
                                    .arg(Arg::new("mode")
                                        .short('m')
                                        .long("mode")
                                        .value_name("MODE")
                                        .help("Set operation mode to add or subtract the defined seconds to/from the timestamps `+/-`")
                                        .required(true)
                                        .takes_value(true))
                                    .arg(Arg::new("output filename")
                                        .short('o')
                                        .long("out")
                                        .value_name("OUTPUT FILE")
                                        .help("Set the name of the output file after editing")
                                        .required(false)
                                        .default_value("out.srt")
                                        .takes_value(true))
                                    .get_matches();
    
    let time_split = " --> ".to_string();
    let path = matches.value_of("file").unwrap();
    let time_shift_seconds = matches.value_of("seconds").unwrap().parse::<u32>().unwrap();
    let add_subtract: Operation = Operation::from_str(matches.value_of("mode").unwrap()).unwrap();
    let out = matches.value_of("output filename").unwrap();

    let file = BufReader::new(File::open(path).unwrap());
    let mut output: String = "".to_string();

    for line in file.lines() {
        let l = line.unwrap();
        let mut split = l.split(&time_split);
        
        if l.contains(&time_split) {
            let mut first: TimeStamp = TimeStamp::from_str(split.nth(0).unwrap()).unwrap();
            let mut second: TimeStamp = TimeStamp::from_str(split.nth(0).unwrap()).unwrap();

            match add_subtract {
                Operation::Plus => {
                    first.seconds += time_shift_seconds;
                    second.seconds += time_shift_seconds;
                },
                Operation::Minus => {
                    first.seconds -= time_shift_seconds;
                    second.seconds -= time_shift_seconds;
                }
            };

            output += &(first.to_string() + &time_split + &second.to_string() + "\n");
        } else {
            output += &l;
            output += "\n";
        }
    };

    let mut write_file = File::create(out).unwrap();
    write_file.write_all(output.as_bytes()).expect("Failed to write times to srt file");
}
