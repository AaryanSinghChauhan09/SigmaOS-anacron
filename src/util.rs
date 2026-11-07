//! Anacron model — catch-up job scheduler for non-24/7 systems.

#[derive(Debug, Clone)]
pub struct Job {
    pub name: String,
    pub period_days: u32,
    pub last_run: Option<u64>,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct Anacron {
    pub jobs: Vec<Job>,
    pub today: u64,
}

impl Default for Anacron {
    fn default() -> Self { Self { jobs: vec![], today: 0 } }
}

impl Anacron {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, name: &str, period: u32, cmd: &str) {
        self.jobs.push(Job { name: name.into(), period_days: period, last_run: None, command: cmd.into() });
    }
    pub fn due(&self) -> Vec<&Job> {
        self.jobs.iter().filter(|j| {
            j.last_run.map_or(true, |t| self.today - t >= j.period_days as u64)
        }).collect()
    }
    pub fn mark_run(&mut self, name: &str) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.name == name) { j.last_run = Some(self.today); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn due_run() {
        let mut a = Anacron::new();
        a.add("backup", 7, "backup.sh");
        assert_eq!(a.due().len(), 1);
        a.mark_run("backup");
        assert!(a.due().is_empty());
    }
}
