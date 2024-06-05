use std::{collections::{BTreeMap, HashMap}, io::Write, path::Path};
use crate::{code::Code, Dict, PathExt, Result, Text};


/// 生成略碼
#[allow(unused)]
pub fn gen(char_dict: Dict, word_dict: Dict, target: impl AsRef<Path>) -> Result<()> {
    // 建立單字全碼索引
    let mut char_codes = HashMap::new();
    for vocab in char_dict.vocabs.iter() {
        let char = vocab.text.chars().next().unwrap();
        let Ok(code) = Code::try_from(vocab.spell) else {
            println!("無法解析單字「{}」的全碼 {:?}。", vocab.text, vocab.spell);
            continue;
        };
        char_codes.insert(char, code);
    }

    // 建立通碼映射表
    let mut common_tab = BTreeMap::new();
    for vocab in char_dict.vocabs.iter().chain(word_dict.vocabs.iter()) {
        // 逐字翻譯成全碼 TODO：尽可能使用 vocab 自身的数据
        let Some(codes) = Code::infer(vocab.text, &char_codes) else {
            continue;
        };
        match codes.as_slice() {
            [a] => {
                // 單字有兩種通碼：聲韻、聲韻首
                common_tab.entry(text([a.conso, a.vowel])).or_insert(Vec::new()).push(vocab.clone());
                common_tab.entry(text([a.conso, a.vowel, a.head])).or_insert(Vec::new()).push(vocab.clone());
                
            }
            [a, b] => {
                // 雙字詞只有一種通碼：聲韻聲韻
                common_tab.entry(text([a.conso, a.vowel, b.vowel, b.conso])).or_insert(Vec::new()).push(vocab.clone());
            }
            _ => {
                continue;
            }
        };
    }
    // 排序，最高位近似反映了每個碼位的頻度
    for vocabs in common_tab.values_mut() {
        vocabs.sort_by(|a, b|a.weight.cmp(&b.weight).reverse());
    }

    
    /****** 以上內容可以抽出去給其他功能用 ******/ 


    // 開始構建略碼
    let mut abbr_tab = BTreeMap::new();
    for mut vocab in word_dict.vocabs {
        let count = vocab.text.chars().count();
        // 過濾低頻詞
        let min_weight = match count {
            0 | 1 => u32::MAX,
            2 => 3_0000,
            3 => 500,
            4 => 200,
            _ => u32::MAX,
        };
        if vocab.weight < min_weight {
            continue;
        }
        // 取略码
        let Some(abbr) = Code::infer_abbr(vocab.text, &char_codes) else {
            continue;
        };
        abbr_tab.entry(abbr).or_insert(Vec::new()).push(vocab);
    }

    // 把略碼按词频排序
    for (abbr, vocabs) in abbr_tab.iter_mut() {
        vocabs.sort_by(|a, b|a.weight.cmp(&b.weight).reverse());

        // 只保留一個二簡，且如果保留了，就強制置頂
        if abbr.len() == 2 {
            vocabs.truncate(1);
            // 避讓高頻字
            if let Some(most) = common_tab.get(abbr).map(|vocabs|vocabs[0]) {
                if most.weight >= 5_0000 {
                    vocabs.clear();
                } else {
                    // TODO：調頻是不可靠的，最有有效的辦法應當是把二簡置頂寫入方案的邏輯中
                    vocabs[0].weight = u32::MAX;
                    println!("{}：採納「{}」，下調「{}」", abbr, vocabs[0].text, most.text)
                }
            };
        }

        // TODO：避免三簡影响造句
    }


    // 輸出略碼
    let mut buf = target.as_ref().writer()?;
    writeln!(&mut buf, "{}", Dict::header("moran.abbrev", ""))?;
    while let Some((abbr, mut vocabs)) = abbr_tab.pop_first() {
        // TODO: 分音节
        for vocab in vocabs {
            writeln!(&mut buf, "{}\t{}\t{}", vocab.text, abbr, vocab.weight)?;
        }
    }

    Ok(())
}


fn text<const N:usize>(bytes: [u8;N]) -> Text {
    String::from_utf8(bytes.into_iter().collect()).unwrap().leak()
}


