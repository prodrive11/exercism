//raze("Pling";"Plang";"Plong";string n) {$[sum[x]>0;where x;3]} nn:not n mod 3,5,7

use std::collections::HashMap;

#[cfg(feature = "lambda_fold")]
pub fn raindrops(n: u32) -> String {
    let mut hm: HashMap<u32, &str> = HashMap::new();
    hm.insert(3, "Pling");
    hm.insert(5, "Plang");
    hm.insert(7, "Plong");
    let a = vec![3, 5, 7]
        .iter()
        .filter(|&i| n % i == 0)
        .fold("".to_string(), |mut o, i| {
            o.push_str(hm[i]);
            o
        });
    if a.len() > 0 {
        a
    } else {
        n.to_string()
    }
}

#[cfg(feature = "lambda_zip")]
pub fn raindrops(n: u32) -> String {
    let a = vec![3, 5, 7];
    let b = vec!["Pling", "Plang", "Plong"];

    let mut out: String = "".to_string();
    a.iter()
        .zip(&b)
        //.inspect(|(&x, &y)| println!("our:{:?}, y:{:?}", x, y))
        .map(|(&i, &v)| {
            if n % i == 0 {
                out.push_str(v)
            }
        })
        .for_each(drop);
    if out.len() > 0 {
        return out;
    } else {
        return n.to_string();
    }
}


#[cfg(feature = "match")]
pub fn raindrops(n: u32) -> String {
    let a = vec![3, 5, 7];
//    let out = match a.iter().map(|&x| (n%x==0) as u32).collect::<Vec<u32>>() {   //this doesn't
//    work
    let out = match ((n%3==0) as u32,(n%5==0) as u32,(n%7==0) as u32) {
        (0,0,0) => String::from(n.to_string()), 
        (1,0,0) => String::from("Pling"),
        (0,1,0) => String::from("Plang"),
        (0,0,1) => String::from("Plong"),
        (1,1,0) => String::from("PlingPlang"),
        (1,0,1) => String::from("PlingPlong"),
        (0,1,1) => String::from("PlangPlong"),
        (1,1,1) => String::from("PlingPlangPlong"),
        (_,_,_) => panic!(1),
    };
    out
}



#[cfg(all(not(feature = "match"), not(feature = "lambda_zip"), not(feature = "lambda_fold")))]
pub fn raindrops(n: u32) -> String {
    let mut hm: HashMap<u32, &str> = HashMap::new();
    hm.insert(3, "Pling");
    hm.insert(5, "Plang");
    hm.insert(7, "Plong");
    let mut out: String = "".to_string();

    for i in &[3, 5, 7] {
        if n % i == 0 {
            out.push_str(hm[&i]);
        };
    }
    if out.is_empty() {
        out.push_str(n.to_string().as_str());
    }
    out
}
