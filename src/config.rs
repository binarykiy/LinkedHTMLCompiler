use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::util::VecDict;

pub struct Config {
    workspace: PathBuf,
    out: BufWriter<File>,
    src: VecDict<PathBuf, Rc<String>>,
}

impl Config {
    pub fn init(input: String) -> Self {
        let workspace = Path::new(&input).parent()
            .expect("[FATAL] Failed to open working directory")
            .to_path_buf();
        let out = BufWriter::new(OpenOptions::new()
            .create(true).truncate(true).write(true)
            .open(workspace.join("out.html"))
            .expect("[FATAL] Failed to open the output file"));
        Self {
            workspace,
            out,
            src: VecDict::new(),
        }
    }
    pub fn relative_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.workspace.join(path)
    }
    pub fn write_all<S: AsRef<str>>(&mut self, text: S) {
        self.out.write_all(text.as_ref().as_bytes())
            .expect("[FATAL] Failed to write text to the output file.");
    }
    pub fn read_relative<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Rc<String>> {
        self.read_absolute(self.workspace.join(path))
    }
    pub fn read_absolute(&mut self, path: PathBuf) -> io::Result<Rc<String>> {
        if self.src.get(&path) == None {
            let mut file = BufReader::new(
                OpenOptions::new().read(true).open(&path)?);
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            self.src.push_unique(path.clone(), Rc::new(buf));
        }
        Ok(Rc::clone(self.src.get(&path).unwrap()))
    }
}
