use std::collections::VecDeque;

#[derive(Default)]
pub struct LogBook {
    entries: VecDeque<String>,
}

impl LogBook {
    pub(super) fn add_entry(&mut self, entry: &str) {
        self.entries.push_front(entry.to_string());
        if self.entries.len() > 10 {
            self.entries.pop_back();
        }
    }
    pub(super) fn get_entries(&self) -> Vec<String> {
        self.entries.clone().into()
    }
}
