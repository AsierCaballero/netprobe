use std::collections::HashMap;
use crate::prober::Prober;
use crate::types::ProbeResult;

pub struct App {
    pub targets: Vec<String>,
    pub port: u16,
    pub count: u32,
    pub results: HashMap<String, Vec<ProbeResult>>,
}

impl App {
    pub fn new(targets: Vec<String>, port: u16, count: u32) -> Self {
        Self { targets, port, count, results: HashMap::new() }
    }

    pub async fn run_pass(&mut self) {
        for target in &self.targets {
            let r = Prober::probe(target, self.port, self.count).await;
            self.results.entry(target.clone()).or_default().push(r);
        }
    }
}
