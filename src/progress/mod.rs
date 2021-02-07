use indicatif::ProgressStyle;

pub struct ProgressBar {
    pb: indicatif::ProgressBar,
    total_size: u64,
    finish_msg: String,
}

impl ProgressBar {
    pub fn new(total_size: u64, finish_msg: &str) -> Self {
        let pb = indicatif::ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
        Self {
            pb,
            total_size,
            finish_msg: finish_msg.to_string(),
        }
    }

    pub fn progress(&self, pos: u64) {
        let pos = pos.min(self.total_size);
        self.pb.set_position(pos);
        if pos == self.total_size {
            self.pb.finish_with_message(&self.finish_msg);
        }
    }
}
