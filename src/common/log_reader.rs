use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{result, usize};

#[derive(Debug)]
pub struct LogLine {
    pub ip: IpAddr,
    pub date: String,
    pub time: String,
    pub result: u16,
    pub request_type: String,
    pub user_agent: String,
    pub path: String,
}

#[derive(Debug)]
pub struct LogLines {
    from: String,
    lines: Vec<LogLine>,
    element_position: ElementPosition,
    time_between: usize,
    last_time_read: Duration,
}

#[derive(Debug)]
pub struct ElementPosition {
    ip: usize,
    date: usize,
    time: usize,
    result: usize,
    request_type: usize,
    user_agent: usize,
    path: usize,
}

impl LogLine {
    pub fn new(line: String, element_position: &ElementPosition) -> Self {
        let split: Vec<&str> = line.split(" ").collect();
        let ip: IpAddr;
        if element_position.ip == 99 {
        } else {
            ip = split[element_position.ip].parse().expect("Not found IP");
        }
        let date = split[element_position.date].to_string();
        let time = split[element_position.time].to_string();
        let result = split[element_position.result]
            .parse()
            .expect("Incorrect result");
        let request_type = split[element_position.request_type].to_string();
        let user_agent = split[element_position.user_agent].to_string();
        let path = split[element_position.path].to_string();
        Self {
            ip,
            date,
            time,
            result,
            request_type,
            user_agent,
            path,
        }
    }
}

impl LogLines {
    pub fn read_file(&mut self, file_path: String) {
        let file = File::open(file_path).expect("We can't open the file");
        let reader = BufReader::new(file);
        for result in reader.lines() {
            let line = result.expect("Error when reading line");
            self.lines.push(LogLine::new(line, &self.element_position));
        }
    }

    fn record(&mut self, line: LogLine) {
        if self.time_between == 0 {
            self.lines.push(line);
        } else {
            if self.last_time_read.as_secs()
                > line.time.parse().expect("Issue with time conversion")
            {
                self.lines.push(line)
            }
        }
    }

    pub fn get_lines_number(&self) -> usize {
        self.lines.len()
    }

    pub fn get_last_read_time(&self) -> Duration {
        self.last_time_read
    }
}

impl ElementPosition {
    fn new() -> Self {
        let ip = 0;
        let date = 2;
        let time = 3;
        let result = 4;
        let request_type = 7;
        let user_agent = 8;
        let path = 9;
        Self {
            ip,
            date,
            time,
            result,
            request_type,
            user_agent,
            path,
        }
    }
}
