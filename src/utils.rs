use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{process::Command, time::{Duration, Instant}};
use std::thread::{self, sleep};
use ngx::{core::NgxStr, http::Request, ngx_log_debug_http};

use crate::mem::THREAD_CONTROL;
use crate::ModuleConfig;

pub const PATH_BASH: &str =  "/bin/bash";
pub const PATH_KILL: &str =  "/bin/kill";
pub const HARDKILL_TIMEOUT: u64 =  60;

pub fn find_free_port() -> Result<u16, std::io::Error> {
    // Bind to port 0; the OS will assign a free port
    let listener = TcpListener::bind("127.0.0.1:0")?;

    // Retrieve the assigned port
    match listener.local_addr()? {
        SocketAddr::V4(addr) => Ok(addr.port()),
        SocketAddr::V6(addr) => Ok(addr.port()),
    }
}

pub fn get_host(request: &Request) -> Option<&NgxStr> {
    if !request.get_inner().headers_in.user_agent.is_null() {
        unsafe {
            Some(NgxStr::from_ngx_str(
                (*request.get_inner().headers_in.host).value,
            ))
        }
    } else {
        None
    }
}

pub fn spawn_process(co: &ModuleConfig, port: u16, request: &Request) -> usize {
    // Command to run should be adjusted according to your actual command.
    let mut childp = Command::new(PATH_BASH);

    let fcmd = format!("source /etc/profile ; source ~/.profile ; {}", co.command);

    childp
        .args(&["-c", &fcmd])
        .env("HOME", format!("/home/{}", co.user))
        .env("PORT", port.to_string());

    ngx_log_debug_http!(request, "env cmd: {:#?}", fcmd);

    let child = childp.spawn().expect("Failed to spawn process");
    let pid = usize::try_from(child.id()).unwrap();
    pid
    }


pub fn wait_for_connection(port: u16, request: &Request, host: &str) {
    let addr = format!("127.0.0.1:{}", port)
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .next()
        .expect("Unable to resolve address");

    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();

    loop {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            Ok(_) => {
                ngx_log_debug_http!(request, "Successfully connected to {}:{}", host, port);
                break;
            }
            Err(e) => {
                if start_time.elapsed() > timeout {
                    println!("Failed to connect within {:?}: {}", timeout, e);
                    break;
                }
                sleep(Duration::from_millis(500));
            }
        }
    }
}

pub fn now_timestamp() -> u64 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
}

pub fn spawn_once<F: Fn() + Send + 'static>(f: F) {
    let mut tc = THREAD_CONTROL.lock().unwrap();

    // Check if there's an active thread and if it has finished
    let should_spawn = match &*tc {
        Some(handle) => handle.is_finished(),
        None => true,
    };

    if should_spawn {
        // Spawn the thread if necessary
        *tc = Some(thread::spawn(f));
    }
}

pub fn parse_duration_to_seconds(input: &str) -> Result<u64, &'static str> {
    let len = input.len();
    if len < 2 {
        return Err("Input too short");
    }

    let (num_part, unit) = input.split_at(len - 1);
    let number: u64 = match num_part.parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number"),
    };

    match unit {
        "s" => Ok(number),
        "m" => Ok(number * 60),
        "h" => Ok(number * 3600),
        "d" => Ok(number * 3600 * 24),
        _ => Err("Unsupported unit"),
    }
}

pub fn parse_on_off_to_bool(input: &str) -> Result<bool, &'static str> {
    match input {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err("Invalid input"),
    }
}

