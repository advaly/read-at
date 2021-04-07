use serialport;
use std::time::Duration;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use clap::{App, Arg};
use std::process::exit;

fn main() {
    let app = App::new("read-at")
        .version("0.1.0")
        .about("AT command writer and reader")
        .author("ADVALY SYSTEM Inc.")
        .arg(Arg::with_name("AT command")
            .help("AT command")
            .required(true))
        .arg(Arg::with_name("device")
            .help("Serial port")
            .short("d")
            .default_value("/dev/ttyUSB2"))
        .arg(Arg::with_name("cclk")
            .help("Parse CCLK result and show datetime with format 'yyyy/mm/dd HH:MM:SS'")
            .long("cclk"))
        .arg(Arg::with_name("no ok")
            .help("Do not show 'OK'")
            .short("n").long("no-ok"))
        .arg(Arg::with_name("no error")
            .help("Do not show 'ERROR'")
            .short("e").long("no-error"))
        .arg(Arg::with_name("response wait")
            .help("Wait time in milli-seconds for response")
            .short("w")
            .default_value("5"))
        .arg(Arg::with_name("timeout")
            .help("Timeout in milli-seconds for serial port access")
            .short("t")
            .default_value("1"))
        .arg(Arg::with_name("baud rate")
            .help("Serial baud rate")
            .short("b")
            .default_value("115200"))
        .get_matches();

    let device = app.value_of("device").unwrap();
    let timeout = app.value_of("timeout").unwrap().parse::<u64>().unwrap();
    let response_wait = app.value_of("response wait").unwrap().parse::<u64>().unwrap();
    let baudrate = app.value_of("baud rate").unwrap().parse::<u32>().unwrap();
    let command = app.value_of("AT command").unwrap();
    let is_cclk = app.is_present("cclk");

    // Open serial port
    let mut port = match serialport::new(device, baudrate) 
        .timeout(Duration::from_millis(timeout))
        .open() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error while open port {}: {}", device, e);
                exit(1);
            },
    };

    // Write a command text
    if let Err(e) = port.write(format!("{}\r", command).as_bytes()) {
        eprintln!("Error while write to port: {}", e);
        exit(2);
    }

    // Wait for response
    sleep(Duration::from_millis(response_wait));

    let mut reader = BufReader::new(port);
    let mut vbuf: Vec<String> = Vec::new();

    // Read all response
    for _i in 0..32 {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_len) => {
                let s = line.trim();
                if s.len() > 0 {
                    vbuf.push(s.to_string());
                }
            },
            Err(_e) => break,
        }
    }

    // Set return code
    //   Some AT command returns response after OK,
    //   so here we try to find OK in all lines not only the last line.
    let retcode = match vbuf.clone().into_iter().find(|s| s == "ERROR") {
        Some(_s) => 3,
        None => 0,
    };

    // Show datetime if cclk mode
    if is_cclk {
        for s in vbuf {
            if let Some(_n) = s.find("CCLK:") {
                let year = if s[8..10].parse::<i32>().unwrap() < 70 {"20"} else {"19"};
                // use writeln to avoid broken pipe error
                writeln!(io::stdout(), "{}{} {}", year, &s[8..16], &s[17..25]).unwrap_or_else(|_e| exit(3));
            }
        }
    }

    // Show results
    else {
        for s in vbuf {
            if app.is_present("no ok") && s == "OK" {
                continue;
            }
            if app.is_present("no error") && s == "ERROR" {
                continue;
            }
            // use writeln to avoid broken pipe error
            writeln!(io::stdout(), "{}", s).unwrap_or_else(|_e| exit(3));
        }
    }

    exit(retcode);
}
