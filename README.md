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
