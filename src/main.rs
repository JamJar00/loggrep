use std::io;
use regex::Regex;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'F', long)]
    format: String,

    #[arg(value_parser)]
    field: String,

    #[arg(value_parser)]
    regex: String
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let extract_re = match args.format.as_str() {
        // Nginx (default format)
        // $remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent"
        // E.g. 66.249.65.159 - - [06/Nov/2014:19:10:38 +0600] "GET /news/53f8d72920ba2744fe873ebc.html HTTP/1.1" 404 177 "-" "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5376e Safari/8536.25 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        "nginx" => Regex::new(r#"^(?<remote_addr>\S+) - (?<remote_user>.+) \[.+] "(?<request>.+)" (?<status>\d+) (?<body_bytes_sent>\d+) "(?<http_referer>.+)" "(?<user_agent>.+)"$"#).unwrap(),

        // Syslog original format
        // https://www.ietf.org/rfc/rfc3164.txt
        // <priority>timestamp hostname: message
        // E.g. <34>Oct 11 22:14:15 mymachine su: 'su root' failed for lonvick on /dev/pts/8
        "syslog-bsd" => Regex::new(r"^<(?<priority>\d{1,3}+)>(?<timestamp>\w\w\w [\d ]\d \d\d:\d\d:\d\d) (?<hostname>\S+) (?<message>.+)$").unwrap(),

        // TODO Syslog IETF format
        // TODO Syslog Extended IETF format
        // https://datatracker.ietf.org/doc/html/rfc5424

        _ => panic!("Invalid format")
    };

    let match_re = Regex::new(args.regex.as_str()).unwrap();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let captures = extract_re.captures(line.as_str());
        match captures {
            Some(captures) => {
                let match_field = captures.name(args.field.as_str()).unwrap().as_str();
                if match_re.is_match(match_field) {
                    println!("{}", line)
                }
            }
            None => eprintln!("Line could not be decoded into the expected format")
        }
    }
    Ok(())
}
