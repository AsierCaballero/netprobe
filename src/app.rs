use crate::prober::Prober;
use crate::types::ProbeResult;
use std::collections::HashMap;

pub struct App {
    pub targets: Vec<String>,
    pub port: u16,
    pub count: u32,
    pub current_pass: u64,
    pub results: HashMap<String, Vec<ProbeResult>>,
}

impl App {
    pub fn new(targets: Vec<String>, port: u16, count: u32) -> Self {
        Self {
            targets,
            port,
            count,
            current_pass: 0,
            results: HashMap::new(),
        }
    }

    pub async fn run_pass(&mut self) {
        self.current_pass += 1;
        for target in &self.targets {
            let r = Prober::probe(target, self.port, self.count, 5).await;
            self.results.entry(target.clone()).or_default().push(r);
        }
    }
}
