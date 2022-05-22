use std::time::Instant;

pub struct Reporter {
    time_stamp: Instant,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter {
            time_stamp: Instant::now(),
        }
    }

    pub fn report_time(&mut self, msg: &str) {
        let time = self.time_stamp.elapsed();
        println!("{} {:?}", msg, time);
        self.time_stamp = Instant::now();
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}
