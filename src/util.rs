//! Dstat model — versatile resource statistics.

#[derive(Debug, Clone)]
pub struct StatSample {
    pub timestamp: String,
    pub cpu_user: f64,
    pub cpu_sys: f64,
    pub mem_used: u64,
    pub mem_total: u64,
    pub disk_read: u64,
    pub disk_write: u64,
    pub net_recv: u64,
    pub net_sent: u64,
}

#[derive(Debug, Clone)]
pub struct Dstat {
    pub interval: u64,
    pub count: usize,
    pub csv: bool,
    pub output_file: Option<String>,
}

impl Default for Dstat {
    fn default() -> Self { Self { interval: 1, count: 0, csv: false, output_file: None } }
}

impl Dstat {
    pub fn new() -> Self { Self::default() }

    pub fn sample(&self) -> StatSample {
        StatSample { timestamp: String::new(), cpu_user: 0.0, cpu_sys: 0.0, mem_used: 0, mem_total: 0, disk_read: 0, disk_write: 0, net_recv: 0, net_sent: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let d = Dstat::new();
        assert_eq!(d.sample().mem_used, 0);
    }
}
