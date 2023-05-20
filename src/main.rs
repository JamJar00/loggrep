use std::io;
use regex::Regex;
use std::io::BufRead;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'F', long, help = "Specify the expected format of the logs")]
    format: Option<String>,

    #[arg(short = 'I', long, help = "Prints info about the log format instead of filtering")]
    info: bool,

    #[arg(short = 'i', long, help = "Ignore case")]
    ignore_case: bool,

    #[arg(short = 'v', long, help = "Invert the sense of matching, to select non-matching lines")]
    invert_match: bool,

    #[arg(value_parser, help = "Field to filter on")]
    field: Option<String>,

    #[arg(value_parser, help = "Regex to filter the field on")]
    regex: Option<String>
}

fn main() -> io::Result<()> {
    let regexes = HashMap::from([
        // Nginx (default format)
        // $remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent"
        // E.g. 66.249.65.159 - - [06/Nov/2014:19:10:38 +0600] "GET /news/53f8d72920ba2744fe873ebc.html HTTP/1.1" 404 177 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5376e Safari/8536.25 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        ("nginx", r#"^(?<remote_addr>\S+) - (?<remote_user>.+) \[.+] "(?<request>.+)" (?<status>\d+) (?<body_bytes_sent>\d+) "(?<http_referer>.+)" "(?<user_agent>.+)"$"#),

        // Syslog original format
        // https://www.ietf.org/rfc/rfc3164.txt
        // <priority>timestamp hostname: message
        // E.g. <34>Oct 11 22:14:15 mymachine su: 'su root' failed for lonvick on /dev/pts/8
        ("syslog-bsd", r"^<(?<priority>\d{1,3}+)>(?<timestamp>\w\w\w [\d ]\d \d\d:\d\d:\d\d) (?<hostname>\S+) (?<message>.+)$"),

        // TODO Syslog IETF format
        // TODO Syslog Extended IETF format
        // https://datatracker.ietf.org/doc/html/rfc5424

        // Python default format
        // %(levelname)s:%(name)s:%(message)s
        // https://github.com/python/cpython/blob/main/Lib/logging/__init__.py#LL538C19-L538C19
        // E.g. ERROR:root:Some error
        // E.g. WARNING:root:This is a log line!
        ("python", r"^(?<levelname>\w+):(?<name>[\w.]+):(?<message>.+)$")
    ]);

    let args = Cli::parse();
    let mut lines = io::stdin().lock().lines();

    if args.info {
        let (format_name, extract_re) = match &args.format {
            Some(format_arg) => (format_arg.as_str(), Regex::new(regexes[format_arg.as_str()]).unwrap()),
            None => {
                let first_line = lines.next().unwrap().unwrap();
                let (format_name, regex) = autodetect_format(regexes, first_line.as_str());
                (format_name, regex)
            }
        };

        println!("Format:           {}", format_name);
        println!("Regex:            {}", extract_re.as_str());
        // FIXME use intersperse/collect when released from nightly rust builds
        println!("Available Fields: {}", extract_re.capture_names().flatten().collect::<Vec<&str>>().join(", "));
    } else {
        if args.field == None || args.regex == None {
            panic!("No field/regex specified to filter with");
        }

        let field = args.field.unwrap();
        let regex = if args.ignore_case {
            "(?i)".to_string() + args.regex.unwrap().as_str()
        } else {
            args.regex.unwrap()
        };
        let match_re = Regex::new(regex.as_str()).unwrap();

        let extract_re = match &args.format {
            Some(format_arg) => Regex::new(regexes[format_arg.as_str()]).unwrap(),
            None => {
                let first_line = lines.next().unwrap().unwrap();
                let (_, regex) = autodetect_format(regexes, first_line.as_str());
                process_line(&regex, &match_re, field.as_str(), first_line.as_str(), args.invert_match);
                regex
            }
        };

        for line in lines {
            let line = line.unwrap();
            process_line(&extract_re, &match_re, field.as_str(), line.as_str(), args.invert_match);
        }
    }
    Ok(())
}

fn autodetect_format<'a>(regexes: HashMap<&'a str, &str>, line: &str) -> (&'a str, Regex) {
    for (name, regex) in regexes {
        let regex = Regex::new(regex).unwrap();
        if regex.is_match(line.trim()) {
            return (name, regex);
        }
    }
    panic!("Could not detect log format")
}


fn process_line(extract_re: &Regex, match_re: &Regex, field: &str, line: &str, invert_match: bool) {
    let line = line.trim();
    if line.is_empty() {
        return;
    }

    let captures = extract_re.captures(line);
    match captures {
        Some(captures) => {
            let match_field = captures.name(field).unwrap().as_str();
            if match_re.is_match(match_field) != invert_match {
                println!("{}", line)
            }
        }
        None => eprintln!("Line could not be decoded into the expected format")
    }
}
