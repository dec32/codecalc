mod code;
mod sort;
use std::{collections::{BTreeMap, HashSet}, env, ffi::OsStr, fmt::Display, fs::{self, File, OpenOptions}, io::{BufWriter, Write}, path::{Path, PathBuf}};

pub type Result<T> = anyhow::Result<T>;


// 因爲是命令行工具所以請隨性一點
pub type Text = &'static str; 

// 拼写，排序時先比較長度再比較内容
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Spell(Text);
impl Ord for Spell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len())
            .then(self.0.cmp(&other.0))
    }
}
impl PartialOrd for Spell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<Text> for Spell {
    fn from(value: Text) -> Self {
        Spell(value)
    }
}
impl Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


// 詞条，保留了順序信息
#[derive(Clone, Copy, Debug)]
pub struct Vocab {
    text: Text,
    spell: Spell,
    weight: u32,
    ord: usize,
}


pub struct Dict {
    header: Text,
    vocabs: BTreeMap<Spell, Vec<Vocab>>
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
        let mut vocabs = BTreeMap::new();
        for line in content[header_end..].lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let mut split = line.split("\t");
            let Some(text) =  split.next() else {
                println!("解析失敗：{line}");
                continue;
            };

            let spell = split.next().unwrap_or_default().into();
            let weight = split.next().and_then(|w|w.parse().ok()).unwrap_or_default();
            vocabs.entry(spell).or_insert(Vec::new()).push(Vocab{ text, spell, weight, ord });
            ord += 1;
        }
        Ok(Dict {header, vocabs})
    }


    fn patch(&mut self, patch: Dict) {
        for (spell, vocabs) in patch.vocabs {
            self.vocabs.insert(spell, vocabs);
        }
    }


    #[allow(unused)]
    fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().injected();
        let out = OpenOptions::new().create(true).write(true).open(path)?;
        let mut out = BufWriter::new(out);
        writeln!(&mut out, "{}", self.header)?;
        for (spell, vocabs) in self.vocabs.iter() {
            for vocab in vocabs {
                writeln!(&mut out, "{}\t{}\t{}", vocab.text, spell, vocab.weight)?;
            }
        }
        Ok(())
    }

    #[allow(unused)]
    fn save_weightless(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().injected();
        let out = OpenOptions::new().create(true).write(true).open(path)?;
        let mut out = BufWriter::new(out);
        writeln!(&mut out, "{}", self.header)?;
        for (spell, vocabs) in self.vocabs.iter() {
            for vocab in vocabs {
                writeln!(&mut out, "{}\t{}", vocab.text, spell)?;
            }
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



#[test]
fn empty_slots() -> Result<()> {
    let slots = Dict::open("rime:moran.chars.dict.yaml")?.vocabs.keys()
        .map(|spell|unsafe{ String::from_utf8_unchecked(spell.0.as_bytes()[..2].to_vec()) })
        .collect::<HashSet<_>>();

    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            let mut slot = String::new();
            slot.push(c1);
            slot.push(c2);
            if slots.contains(&slot) {
                continue;
            }
            println!("{slot}")
        }
    }
    Ok(())
}

#[test]
fn patch() -> Result<()> {
    let mut dict = Dict::open("rime:moran_fixed.dict.yaml")?;
    let preset = Dict::open("rime:moran_fixed.preset.dict.yaml")?;
    dict.patch(preset);
    dict.save_weightless("target.txt")
}

fn main() {
    // // handle().unwrap()
    // sort().unwrap();
    // filter().unwrap();
    // empty_slots().unwrap();
}
