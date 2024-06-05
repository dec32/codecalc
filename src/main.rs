mod abbr;
mod code;

use std::{env, ffi::OsStr, fs::{self, File, OpenOptions}, io::{BufWriter, Write}, path::{Path, PathBuf}};


pub type Result<T> = anyhow::Result<T>;
pub type Text = &'static str; // 因爲是命令行工具所以請隨性一點

pub struct Dict {
    #[allow(unused)]
    header: Text,
    vocabs: Vec<Vocab>
}

#[derive(Clone, Copy, Debug)]
pub struct Vocab {
    text: Text,
    spell: Text,
    weight: u32,
    #[allow(unused)]
    ord: usize,
}


impl Dict {
    fn open(path: impl AsRef<Path>) -> Result<Dict> {
        let path = path.as_ref().injected();
        let content = fs::read_to_string(path)?.leak();

        // 這個搜索方法不是很嚴謹，懶的寫了
        let search = "...".as_bytes();
        let header_end = content.as_bytes()
            .windows(search.len())
            .take(1024)
            .position(|slice|slice == search)
            .map(|pos|pos + search.len())
            .unwrap_or(0);
        let header = &content[0..header_end];

        // 解析詞彙
        let mut ord = 0;
        let mut vocabs = Vec::new();
        for line in content[header_end..].lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let mut split = line.split("\t");
            let Some(text) =  split.next() else {
                println!("解析失敗：{line}");
                continue;
            };

            let spell = split.next().unwrap_or_default();
            let weight = split.next().and_then(|w|w.parse().ok()).unwrap_or_default();
            vocabs.push(Vocab{text, spell, weight, ord});
            ord += 1;
        }
        Ok(Dict {header, vocabs})
    }


    #[allow(unused)]
    fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().injected();
        let out = OpenOptions::new().create(true).write(true).open(path)?;
        let mut out = BufWriter::new(out);
        writeln!(&mut out, "{}", self.header)?;
        for vocab in self.vocabs.iter() {
            writeln!(&mut out, "{}\t{}\t{}", vocab.text, vocab.spell, vocab.weight)?;
        }
        Ok(())
    }


    pub fn header(name: &str, comment: &str) -> String {
        format!(include_str!("../res/header.yaml"), {comment}, {name})
    }
}







/// 方便操作
trait PathExt {
    fn injected(&self) -> PathBuf;
    fn writer(&self) -> std::io::Result<BufWriter<File>>;
}
impl PathExt for Path {
    fn injected(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for stem in self.into_iter() {
            let stem = stem.to_str()
                .and_then(|stem| {
                    if stem.starts_with('%') && stem.ends_with('%') {
                        let var_name = &stem[1..stem.len()-1];
                        env::var_os(var_name)
                    } else if stem.starts_with("rime:") {
                        let mut var = env::var_os("APPDATA").expect("找不到 %APPDATA% 變量，不能使用 rime: 路徑。");
                        var.push(OsStr::new("/Rime/"));
                        var.push(&stem["rime:".len()..]);
                        Some(var)
                    } else {
                        None
                    }
                })
                .unwrap_or(stem.to_os_string());
            buf = buf.join(stem);
        }
        buf
    }

    fn writer(&self) -> std::io::Result<BufWriter<File>> {
        let path = self.injected();
        let out = OpenOptions::new().create(true).write(true).open(path)?;
        Ok(BufWriter::new(out))
    }
}



fn handle() -> Result<()> {
    let char_dict = Dict::open("rime:moran.chars.dict.yaml")?;
    let word_dict = Dict::open("res/pinyin.txt")?;
    abbr::gen(char_dict, word_dict, "rime:moran.abbrev.dict.yaml")?;
    Ok(())
}

fn main() {
    handle().unwrap()
}
