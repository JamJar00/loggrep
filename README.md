# loggrep
Grep, but for log files of various shapes and sizes

This project was created in slightly more than an evening with the intention that I'll add more log formats to it as I go along and feel like they would be useful. There's no guarantee I'll actually do that though.

## Installation
Download the latest release from the [releases page](https://github.com/JamJar00/loggrep/releases) and add it somewhere on your path (e.g. `~/.bin/local`).

Windows users will currently need to build from source using `cargo build --release`, sorry!

There is also a docker image available on [Docker Hub](https://hub.docker.com/r/jamoyjamie/loggrep)

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
cat logfile.txt | loggrep remote_addr "^66.249.65.159$"
```
Or to filter for GET requests:
```bash
cat logfile.txt | loggrep request "^GET"
```

With docker:
```bash
cat logfile.txt | docker run -i jamoyjamie/loggrep:<version> request "^GET"
```

## Supported Formats
### nginx
The default format of nginx logs. The following fields map to the nginx variables of the same name.

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

### Python
Default log format for Python as per [the source](https://github.com/python/cpython/blob/main/Lib/logging/__init__.py#LL538C19-L538C19)

Fields:
- `levelname`
- `name`
- `message`

### update-alternatives
Simple format for the update-alternatives log in `/var/log/alternatives.log`.

Fields:
- `timestamp`
- `message`

### dpkg
Format for dpkg logs in `/var/log/dpkg.log` as per the [man page](https://man7.org/linux/man-pages/man1/dpkg.1.html). Note that this log format has three different forms which makes it tricky to parse. Due to internal limitations, currently some fields need to be repeated with a numeric on the end.

Fields:
- `timestamp`
- `type`
- `command`
- `state`
- `pkg`
- `installed_version`
- `action`
- `pkg_2`
- `installed_version_2`
- `available_version`
- `filename`
- `decision`

## TODO
- More formats
- Add argument to draw logs from file
- Add colour to matched field if outputting to a terminal
- Support matching on multiple fields? (You can always just pipe loggrep into loggrep to do this...)
- Tests, always tests
- Support custom log formats from config file
