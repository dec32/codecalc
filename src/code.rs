use std::collections::HashMap;

use crate::Text;
use crate::Result;
use anyhow::Error;

// 全碼
#[derive(Clone, Copy, Debug)]
pub struct Code {
    pub conso: u8,
    pub vowel: u8, 
    pub head: u8, 
    pub tail: u8 
}

impl Code {
    /// 失敗時靜默返回
    pub fn infer(text: Text, char_codes: &HashMap<char, Code>) -> Option<Vec<Code>> {
        let mut codes = Vec::new();
        for char in text.chars() { 
            let Some(mut code) = char_codes.get(&char).cloned() else {
                println!("不能推斷「{text}」的全碼（缺「{char}」字）");
                return None;
            };
            // 強行鶴化
            code.fly();
            codes.push(code);
        }
        Some(codes)
    }

    pub fn infer_abbr(text: Text, char_codes: &HashMap<char, Code>) -> Option<Text> {
        let mut abbr = Vec::new();
        for char in text.chars() { 
            let Some(code) = char_codes.get(&char).cloned() else {
                println!("不能推斷「{text}」的略码（缺「{char}」字）");
                return None;
            };
            abbr.push(code.conso);
        }
        unsafe{ Some(String::from_utf8_unchecked(abbr).leak()) }
    }
}

impl TryFrom<Text> for Code {
    type Error = anyhow::Error;
    fn try_from(spell: Text) -> Result<Self> {
        if spell.len() != 5{
            return Err(Error::msg("格式有誤，不是全碼。"));
        }
        let spell = spell.as_bytes();
        let full = Code {
            conso: spell[0],
            vowel: spell[1],
            head: spell[3],
            tail: spell[4]
        };
        Ok(full)
    }
}


impl Code {
    fn fly(&mut self) {
        let bytes = [self.conso, self.vowel];
        let phone = unsafe {core::str::from_utf8_unchecked(&bytes)};
        let converted = match phone {
            "bz" => "bw",
            "dz" => "dw",
            "fz" => "fw",
            "gz" => "gw",
            "hz" => "hw",
            "kz" => "kw",
            "lz" => "lw",
            "mz" => "mw",
            "nz" => "nw",
            "pz" => "pw",
            "sz" => "sw",
            "uz" => "uw",
            "tz" => "tw",
            "wz" => "ww",
            "zz" => "zw",
            "vz" => "vw",
            "ip" => "iy",
            "cp" => "cy",
            "dp" => "dy",
            "gp" => "gy",
            "hp" => "hy",
            "jp" => "jy",
            "kp" => "ky",
            "lp" => "ly",
            "np" => "ny",
            "qp" => "qy",
            "rp" => "ry",
            "up" => "uy",
            "sp" => "sy",
            "tp" => "ty",
            "xp" => "xy",
            "yp" => "yy",
            "vp" => "vy",
            "zp" => "zy",
            "bx" => "bp",
            "dx" => "dp",
            "jx" => "jp",
            "lx" => "lp",
            "mx" => "mp",
            "nx" => "np",
            "px" => "pp",
            "qx" => "qp",
            "tx" => "tp",
            "xx" => "xp",
            "bl" => "bd",
            "cl" => "cd",
            "il" => "id",
            "dl" => "dd",
            "gl" => "gd",
            "hl" => "hd",
            "kl" => "kd",
            "ll" => "ld",
            "ml" => "md",
            "nl" => "nd",
            "pl" => "pd",
            "sl" => "sd",
            "ul" => "ud",
            "tl" => "td",
            "wl" => "wd",
            "zl" => "zd",
            "vl" => "vd",
            "by" => "bk",
            "dy" => "dk",
            "jy" => "jk",
            "ly" => "lk",
            "my" => "mk",
            "ny" => "nk",
            "py" => "pk",
            "qy" => "qk",
            "ty" => "tk",
            "xy" => "xk",
            "yy" => "yk",
            "iy" => "ik",
            "gy" => "gk",
            "hy" => "hk",
            "ky" => "kk",
            "uy" => "uk",
            "vy" => "vk",
            "dd" => "dl",
            "jd" => "jl",
            "ld" => "ll",
            "nd" => "nl",
            "qd" => "ql",
            "xd" => "xl",
            "id" => "il",
            "gd" => "gl",
            "hd" => "hl",
            "kd" => "kl",
            "ud" => "ul",
            "vd" => "vl",
            "ib" => "iz",
            "cb" => "cz",
            "db" => "dz",
            "fb" => "fz",
            "gb" => "gz",
            "hb" => "hz",
            "kb" => "kz",
            "lb" => "lz",
            "mb" => "mz",
            "nb" => "nz",
            "pb" => "pz",
            "rb" => "rz",
            "ub" => "uz",
            "sb" => "sz",
            "tb" => "tz",
            "yb" => "yz",
            "vb" => "vz",
            "zb" => "zz",
            "jw" => "jx",
            "lw" => "lx",
            "nw" => "nx",
            "qw" => "qx",
            "xw" => "xx",
            "iw" => "ix",
            "gw" => "gx",
            "hw" => "hx",
            "kw" => "kx",
            "uw" => "ux",
            "vw" => "vx",
            "bk" => "bc",
            "ck" => "cc",
            "ik" => "ic",
            "dk" => "dc",
            "gk" => "gc",
            "hk" => "hc",
            "kk" => "kc",
            "lk" => "lc",
            "mk" => "mc",
            "nk" => "nc",
            "pk" => "pc",
            "rk" => "rc",
            "sk" => "sc",
            "uk" => "uc",
            "tk" => "tc",
            "yk" => "yc",
            "zk" => "zc",
            "vk" => "vc",
            "bn" => "bb",
            "jn" => "jb",
            "ln" => "lb",
            "mn" => "mb",
            "nn" => "nb",
            "pn" => "pb",
            "qn" => "qb",
            "xn" => "xb",
            "yn" => "yb",
            "bc" => "bn",
            "dc" => "dn",
            "jc" => "jn",
            "lc" => "ln",
            "mc" => "mn",
            "nc" => "nn",
            "pc" => "pn",
            "qc" => "qn",
            "tc" => "tn",
            "xc" => "xn",
            other => other
        };
        self.conso = converted.as_bytes()[0];
        self.vowel = converted.as_bytes()[1];
    }
}