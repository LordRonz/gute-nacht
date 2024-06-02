use base64::prelude::*;
use rand::{seq::SliceRandom, Rng};

use super::emojis::get_emojis; // 0.7.2

pub fn get_description() -> String {
    let probability_vec = [vec![0; 35], vec![1; 30], vec![2; 20], vec![3; 10], vec![4; 5]].concat();
    let ps_vec_raw = [
        "QWlzaGl0ZXJ1",
        "UmVtaW5kZXI6IEFrdSBzYXlhbmdnZyBrYW11IDwz",
        "SSB3aWxsIGFsd2F5cyBsb3ZlIHlvdSBhbmQgYmUgYnkgeW91ciBzaWRl",
    ];

    let ps_vec = ps_vec_raw
        .map(|x| BASE64_STANDARD.decode(x).unwrap())
        .map(|x| String::from_utf8(x).unwrap());

    let tsuki = probability_vec.choose(&mut rand::thread_rng()).unwrap();

    // [0, 1, 2]
    let pps = rand::thread_rng().gen_range(0..3) < 2;

    let mut description_list: Vec<String> = vec![];

    for _ in 0..3 {
        let emoji = get_emojis();
        let animated = if emoji.animated { "a" } else { "" };
        description_list.push(format!("<{animated}:{name}:{id}>", name=emoji.name, id=emoji.id, animated=animated))
    }
    let mut description = description_list.join(" ");

    if *tsuki > 0 {
        description += format!("\nP.S. : ||{ps_template}||", ps_template=ps_vec[tsuki - 1]).as_str();
        if pps {
            let pps_desc = String::from_utf8(BASE64_STANDARD.decode("SSByZWFsbHkgbWVhbiBpdA==").unwrap()).unwrap();
            description += format!("\nP.P.S. : ||{pps_template}||", pps_template=pps_desc).as_str();
        }
    }

    return description;
}
