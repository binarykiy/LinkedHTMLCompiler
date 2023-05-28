use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::util::{Lazy, VecDict};

pub struct Config {
    workspace: PathBuf,
    out: Lazy<PathBuf, BufWriter<File>>,
    src: VecDict<PathBuf, Rc<String>>,
}

impl Config {
    pub fn new(input: String) -> (Self, io::Result<Rc<String>>) {
        let workspace = Path::new(&input).parent()
            .expect("[FATAL] Failed to open working directory")
            .to_path_buf();
        let out = Lazy::new(|path| {
            BufWriter::new(OpenOptions::new()
            .create(true).truncate(true).write(true)
            .open(path)
            .expect("[FATAL] Failed to open the output file"))
        }, workspace.join("out.html"));
        let mut cfg = Self {
            workspace,
            out,
            src: VecDict::new(),
        };
        let source = cfg.read_absolute(PathBuf::from(input));
        (cfg, source)
    }
    pub fn write_all<S: AsRef<str>>(&mut self, text: S) {
        self.out.get_mut().write_all(text.as_ref().as_bytes())
                .expect("[FATAL] Failed to write text to the output file.");
    }
    pub fn read_relative<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Rc<String>> {
        self.read_absolute(self.workspace.join(path))
    }
    fn read_absolute(&mut self, path: PathBuf) -> io::Result<Rc<String>> {
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
