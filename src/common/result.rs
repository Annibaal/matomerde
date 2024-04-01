use crate::common::log_reader::LogLines;
use std::{time::Duration, usize};
#[derive(Debug)]
struct Resulte {
    total_request: usize,
    date: Duration,
    total_request_ok: usize,
    total_request_ko: usize,
    request_by_path: Vec<RequestPath>,
}

#[derive(Debug)]
struct RequestType {
    number_ok: usize,
    number_error: usize,
    request_type: String,
}

#[derive(Debug)]
pub struct RequestPath {
    number_ok: usize,
    number_ko: usize,
    request_type: String,
    path: String,
    error_type: Vec<Requests>,
}

#[derive(Debug)]
struct Requests {
    request_type: String,
    http_code: String,
    number: usize,
}

//pub fn create_result(log_lines: LogLines) -> Resulte {}
