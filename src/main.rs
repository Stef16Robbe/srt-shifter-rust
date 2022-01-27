use clap::{Arg, App};
use strum_macros::EnumString;
use chrono::{NaiveTime, Duration};
use std::{io::{BufReader, BufRead, Write}, fs::File, str::FromStr, ops::{AddAssign, SubAssign}};

#[derive(Debug, EnumString)]
enum Operation {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus
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
    
    let fmt = "%H:%M:%S,%3f";
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
            let mut first = NaiveTime::parse_from_str(split.nth(0).unwrap(), &fmt).unwrap();
            let mut second = NaiveTime::parse_from_str(split.nth(0).unwrap(), &fmt).unwrap();

            match add_subtract {
                Operation::Plus => {
                    first.add_assign(Duration::seconds(time_shift_seconds.into()));
                    second.add_assign(Duration::seconds(time_shift_seconds.into()));
                },
                Operation::Minus => {
                    first.sub_assign(Duration::seconds(time_shift_seconds.into()));
                    second.sub_assign(Duration::seconds(time_shift_seconds.into()));
                }
            };

            output += &(first.format("%-H:%M:%S,%3f").to_string() + &time_split + &second.format("%-H:%M:%S,%3f").to_string() + "\n");
        } else {
            output += &l;
            output += "\n";
        }
    };

    let mut write_file = File::create(out).unwrap();
    write_file.write_all(output.as_bytes()).expect("Failed to write times to srt file");
}
