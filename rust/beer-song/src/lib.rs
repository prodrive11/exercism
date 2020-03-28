const VERSE0: &'static str = "No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n";

const VERSE1: &'static str = "1 bottle of beer on the wall, 1 bottle of beer.\n\
Take it down and pass it around, no more bottles of beer on the wall.\n";

const VERSE2: &'static str = "2 bottles of beer on the wall, 2 bottles of beer.\n\
Take one down and pass it around, 1 bottle of beer on the wall.\n";


const BOF: &'static str = " bottles of beer";
const OTW: &'static str = " on the wall";
const TOD: &'static str = "Take one down and pass it around, ";

pub fn verse(n: u32) -> String {
    match n {
        3..=99 => {
        let sn = n.to_string();
        let snn = (n-1).to_string();
        let v = vec![
                sn.as_str(), BOF, OTW, ", ", sn.as_str(), BOF, ".\n",
                TOD, snn.as_str(), BOF, OTW, ".\n",
            ];

             v.as_slice().join("")
        }, 
        2 => VERSE2.to_string(),
        1 => VERSE1.to_string(),
        0 => VERSE0.to_string(),
        _ => panic!(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(|i| verse(i)).collect::<Vec<String>>().join("\n")
}
