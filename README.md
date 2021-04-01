## How to build
```
$ cross build --target armv7-unknown-linux-musleabihf --release
```

## How to use
```
read-at 0.0.1
ADVALY SYSTEM Inc.
AT command writer and reader

USAGE:
    read-at [FLAGS] [OPTIONS] <AT command>

FLAGS:
        --cclk        Parse CCLK result and show datetime with format 'yyyy/mm/dd HH:MM:SS'
    -h, --help        Prints help information
    -e, --no-error    Do not show 'ERROR'
    -n, --no-ok       Do not show 'OK'
    -V, --version     Prints version information

OPTIONS:
    -b <baud rate>            Serial baud rate [default: 115200]
    -d <device>               Serial port [default: /dev/ttyUSB2]
    -w <response wait>        Wait time in milli-seconds for response [default: 5]
    -t <timeout>              Timeout in milli-seconds for serial port access [default: 1]

ARGS:
    <AT command>    AT command
```