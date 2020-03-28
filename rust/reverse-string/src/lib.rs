#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
   input.bytes().rev().map(|b| b as char).collect::<String>()
}


#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    input.graphemes(true).collect::<Vec<&str>>().iter().rev().map(|s|*s).collect::<Vec<&str>>().join("")
}



//---------
pub fn reverse_utf16(input: &str) -> String {
   let z:Vec<u16> = input.encode_utf16().collect::<Vec<u16>>().iter().rev().map(|s|*s).collect();
   String::from_utf16_lossy(z.as_slice())
}



/*
// very suboptimal solutions

use std::collections::VecDeque;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    let ns: Vec<u16> = input.encode_utf16().collect();
    //println!("new str {:?}, len:{}", ns, ns.len());
    if ns.len() == 0 {
        return "".to_string();
    } else {
        let mut v: VecDeque<u16> = VecDeque::with_capacity(ns.len());
        for i in ns {
            v.push_front(i)
        }
        let a: Vec<u16> = v.iter().map(|i| *i as u16).collect();
        // println!("reversed {:?}, len:{}", a, a.len());
        return String::from_utf16_lossy(a.as_slice());
    };
}

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    let ns = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    if ns.len() == 0 {
        return "".to_string();
    } else {
        let mut v: VecDeque<&str> = VecDeque::with_capacity(ns.len());
        for i in ns {
            v.push_front(i)
        }
        let a: Vec<&str> = v.iter().map(|i| *i).collect();
        return a.join("");
    };
}
*/
