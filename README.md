# loggrep
Grep, but for log files of various shapes and sizes

This project was created in a single evening with the intention that I'll add more log formats to it as I go along and feel like they would be useful. There's no guarantee I'll actually do that though.
## Installation
Sorry, currently you'll have to compile from source:
```bash
cargo build --release
```

## Usage
```
Grep, but for log files of various shapes and sizes

Usage: loggrep --format <FORMAT> <FIELD> <REGEX>

Arguments:
  <FIELD>
  <REGEX>

Options:
  -F, --format <FORMAT>
  -h, --help             Print help
  -V, --version          Print version
```

For example, for filter for nginx requests from `66.249.65.159`, you could do:
```bash
cat logfile.txt | loggrep -F nginx remote_addr "^66.249.65.159$"
```
Or to filter for GET requests:
```bash
cat logfile.txt | loggrep -F nginx request "^GET"
```

## Supported Formats
### nginx
The default format of nginx logs is supported. You can query against the following fields which map to the nginx variables of the same name.
Fields:
- `remote_addr`
- `remote_user`
- `request`
- `status`
- `body_bytes_sent`
- `http_referer`
- `user_agent`

### syslog-bsd
Original BSD syslog format defined in [RFC-3164](https://www.ietf.org/rfc/rfc3164.txt)
Fields:
- `priority`
- `timestamp`
- `hostname`
- `message`

## TODO
- More formats
- Add argument to draw logs from file
- Autodetect format based on what rules are able to parse it and the file being read
- Add colour to matched fieled if outputting to a terminal
- Support matching on multiple fields? (You can always just pipe loggrep into loggrep to do this...)
- Tests, always tests
- Support custom log formats from config file
