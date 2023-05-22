use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub struct Config {
    workspace: PathBuf,
    out: BufWriter<File>,
}

impl Config {
    pub fn init(input: String) -> Self {
        let workspace = Path::new(&input).parent()
            .expect("[FATAL] Failed to open working directory")
            .to_path_buf();
        let out = BufWriter::new(OpenOptions::new()
            .create(true).truncate(true).write(true)
            .open(workspace.join("input"))
            .expect("[FATAL] Failed to open the output file"));
        Self {
            workspace,
            out,
        }
    }
    pub fn relative_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.workspace.join(path)
    }
    pub fn write_all<S: AsRef<str>>(&mut self, text: S) {
        self.out.write_all(text.as_ref().as_bytes())
            .expect("[FATAL] Failed to write text to the output file.");
    }
}
