#[cfg(unix)]
pub mod unix;

#[derive(Debug)]
struct ProcessInfo {
    parent_pid: Option<u32>,
    command: String,
}
