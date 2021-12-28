
use std::fs::File;
use std::fmt;

use serde::Deserialize;
use serde_json::{from_reader, from_str, Result, Error};

extern crate proc_macro;
use proc_macro::{TokenStream};


#[derive(Deserialize, Debug)]
struct Bone {
    id: u32,
    pos: [f64; 2]
}

fn get_bones(path: String) -> Result<Vec<Bone>> {
    // let mut file = File::open(path);
    let test_s = r#"[
        {
            "id": 0,
            "pos": [0, 0]
        }
    ]"#;

    // match file {
    //     Ok(o) => return from_reader(o),
    //     Err(e) => panic!("{}", e)
    // }
    let bones: Vec<Bone> = from_str(test_s)?;
    Ok(bones)
}

#[proc_macro]
pub fn gen_from_anim(ts: TokenStream) -> TokenStream {
    let bones = get_bones(ts.to_string()).expect("msg: &str");
    format!("{}", bones[0].id).parse().unwrap()
}
