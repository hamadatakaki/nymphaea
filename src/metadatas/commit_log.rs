use crate::manage_file::*;

use std::path::Path;

pub struct CommitLog {
    pub hash: String,
    message: String,
    unixtime: usize 
}

impl CommitLog {
    pub fn new(hash: &str, message: &str, unixtime: usize) -> Self {
        Self {
            hash: hash.to_string(),
            message: message.to_string(),
            unixtime: unixtime
        }
    }

    fn from_line(line: &str) -> Self {
        let mut splited: Vec<&str> = line.splitn(3, ' ').collect();

        let message = match splited.pop() {
            Some(m) => m,
            None => panic!("bad commit log!\n")
        };
        let unixtime = match splited.pop() {
            Some(ut) => ut.parse::<usize>().unwrap(),
            None => panic!("bad commit log!\n")
        };
        let hash = match splited.pop() {
            Some(h) => h,
            None => panic!("bad commit log!\n")
        };
        Self::new(hash, message, unixtime)
    }

    pub fn commit_logs(branch: &str) -> std::io::Result<Vec<Self>> {
        let log_path = format!(".nymphaea/commit_logs/{}", branch);
        let log_path = Path::new(&log_path);
        let lines = read_every_line(log_path)?;
        let logs: Vec<Self> = lines.iter().map(|s| Self::from_line(s)).collect();
        Ok(logs)
    }
}

pub fn latest_commit_log(branch: &str) -> std::io::Result<Option<CommitLog>> {
    let log_path = format!(".nymphaea/commit_logs/{}", branch);
    let log_path = Path::new(&log_path);
    let line = read_last_line(log_path)?;
    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(CommitLog::from_line(&line)))
    }
}

#[cfg(test)]
mod tests {
    use super::CommitLog;

    #[test]
    fn test_commit_log_from_line() {
        let c = CommitLog::from_line("1234567890abcdef1234 1573746918 :innocent: Delete debugging print");
        assert_eq!(c.hash, "1234567890abcdef1234".to_string());
        assert_eq!(c.message, ":innocent: Delete debugging print".to_string());
        assert_eq!(c.unixtime, 1573746918);
    }
}
