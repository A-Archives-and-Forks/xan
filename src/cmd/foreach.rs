use csv;
use regex::bytes::Regex;
use std::env;
use std::ffi::OsStr;
use std::io::BufReader;
use std::os::unix::ffi::OsStrExt;
use std::process::{Command, Stdio};

use config::{Config, Delimiter};
use select::SelectColumns;
use util;
use CliResult;

static USAGE: &str = "
Execute a bash command once per line in given CSV file.

Deleting all files whose filenames are listed in a column:

  $ xsv foreach filename 'rm {}' assets.csv

Executing a command that outputs CSV once per line without repeating headers:

  $ xsv foreach query 'search --year 2020 {}' queries.csv > results.csv

Same as above but with an additional column containing the current value:

  $ xsv foreach query -c from_query 'search {}' queries.csv > results.csv

Usage:
    xsv foreach [options] <column> <command> [<input>]
    xsv foreach --help

foreach options:
    -u, --unify              If the output of execute command is CSV, will
                             unify the result by skipping headers on each
                             subsequent command.
    -c, --new-column <name>  If unifying, add a new column with given name
                             and copying the value of the current input file line.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the file will be considered to have no
                           headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. [default: ,]
";

#[derive(Deserialize)]
struct Args {
    arg_column: SelectColumns,
    arg_command: String,
    arg_input: Option<String>,
    flag_unify: bool,
    flag_new_column: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&None).writer()?;

    let template_pattern = Regex::new(r"\{\}")?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let column_index = *sel.iter().next().unwrap();

    let mut record = csv::ByteRecord::new();
    let mut output_headers_written = false;

    while rdr.read_byte_record(&mut record)? {
        let current_value = &record[column_index];

        let templated_command =
            template_pattern.replace_all(args.arg_command.as_bytes(), current_value);

        let command = OsStr::from_bytes(&templated_command);

        let shell = match env::var("SHELL") {
            Ok(val) => val,
            Err(_err) => return fail!("No shell found"),
        };

        if !args.flag_unify {
            let mut cmd = Command::new(shell)
                .arg("-c")
                .arg(command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();

            cmd.wait().unwrap();
        } else {
            let mut cmd = Command::new(shell)
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();

            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);
                // let stdout_lines = stdout_reader.lines();

                let mut stdout_rdr = csv::ReaderBuilder::new()
                    .delimiter(match &args.flag_delimiter {
                        Some(delimiter) => delimiter.as_byte(),
                        None => b',',
                    })
                    .has_headers(true)
                    .from_reader(stdout_reader);

                let mut output_record = csv::ByteRecord::new();

                if !output_headers_written {
                    let mut headers = stdout_rdr.byte_headers()?.clone();

                    if let Some(name) = &args.flag_new_column {
                        headers.push_field(name.as_bytes());
                    }

                    wtr.write_byte_record(&headers)?;
                    output_headers_written = true;
                }

                while stdout_rdr.read_byte_record(&mut output_record)? {
                    if args.flag_new_column.is_some() {
                        output_record.push_field(current_value);
                    }

                    wtr.write_byte_record(&output_record)?;
                }
            }

            cmd.wait().unwrap();
        }
    }

    Ok(())
}
