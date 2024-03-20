use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WPath {
    pub path: Vec<u8>,
    pub prob: f64,
}

impl WPath {
    pub fn new(path: Vec<u8>, prob: f64) -> WPath {
        WPath { path, prob }
    }
}

impl fmt::Display for WPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert the path Vec<u8> to a string for easier display
        let path_str: String = self.path.iter().map(|b| format!("{}", b)).collect::<Vec<String>>().join(", ");

        // Write the formatted string into the formatter
        write!(f, "Path: [{}], Probability: {}", path_str, self.prob)
    }
}