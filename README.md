# AT command writer and reader

## How to build
```
$ cross build --target armv7-unknown-linux-musleabihf --release
```
After build, it is better to strip the binary.

## How to use

```text
read-at 0.1.0
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

Some device may require tune timing parameters with `-w` or `-t`.
For example, you may need to add like `-w20` which means wait 20ms for response after sent an AT command request.

## Exit code

code | description
-- | --
0 | Success
1 | Failed to open the serial port
2 | Failed to write to the serial port
3 | Failed to read from the serial port
4 | Found "ERROR" in response strings

## Examples

#### with echo
```
root@armadillo:~# read-at AT+GMR
AT+GMR
Revision:1951B01SIM7080G-JC
OK
```

#### without echo
```
root@armadillo:~# read-at ATE0+GMR
Revision:1951B01SIM7080G-JC
OK
```

#### without result string
```
root@armadillo:~# read-at -n ATE0+GMR
Revision:1951B01SIM7080G-JC
```

#### read CCLK as date command style
```
root@armadillo:~# read-at --cclk ATE0+CCLK?
2021/04/15 11:13:30
```