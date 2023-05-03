// #[warn(unused_imports)]
use std::collections::*;
use std::fs;
use std::net::IpAddr;
use shellexpand;

mod hashmap_index;

#[allow(unused_imports)]
use hashmap_index::{ build_hash_map_index, build_hash_map_index_from_file };

#[warn(unused_variables)]
fn main() -> std::io::Result<()> {
    let _s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let _substr = "elit";

    // search_in_string(s, substr);

    // transmute(s);

    // build_hash_map_index(s);

    let hash_map = build_hash_map_index_from_file(
        &shellexpand::full("~/Downloads/words/en").unwrap()
    )?;

    for word in "hello world how is your day going".split_whitespace() {
        hash_map
            .get_key_value(word)
            .and_then(|tuple| {
                println!("Found \"{}\" on indexes: {:?}", tuple.0, tuple.1);
                Some(tuple)
            }).expect(&format!("Not found: {}", word));
    }

    /* let mut total = 0;
    for (key, value) in hash_map.iter() {
        total += key.capacity();
        total += value.capacity();
    }

    println!("total: {}", total);
    println!("keys: {}; values: {}", hash_map.keys().len(), hash_map.values().len()); */

    Ok(())
}

#[allow(dead_code)]
fn test_collections () {
    let keys = vec!['a', 'b', 'c', 'd', 'e'];
    let values = vec![1, 2, 3, 4];

    let hash: HashMap<_,_> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", hash);
    for (key, value) in hash {
        println!("{}: {}", key, value);
    }

    println!("-------------------\n");

    let bthash: BTreeMap<_,_> = keys.iter().zip(values.iter()).collect();
    for (key, value) in bthash {
        println!("{}: {}", key, value);
    }

    println!("-------------------\n"); 
}

#[allow(dead_code)]
fn test_result () {
    let _r: Result<bool, &str> = Ok(true);
    let _x: Result<bool, &str> = Ok(true); // Err("Error test");
    let r: Result<bool, &str> = Err("Error test");
    println!("{}", r.unwrap());
    println!("{}", r.unwrap_or_else(|err: &str| -> bool { println!("{}", err); false })); 
    match r {
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    }
}

#[allow(dead_code)]
fn test_threads () {
    let mut x = 5;
    let r = std::thread::spawn(move || {
        x *= 2;
        return x;
    }).join().unwrap();
    println!("{}, {}", x, r);
}

#[allow(dead_code)]
fn test_shellexpand () -> std::io::Result<()> {
    let path = shellexpand::full("~/mc.sh").unwrap();
    // let file = fs::File::open(path.into_owned())?;
    // println!("{:?}", file);
    let contents = fs::read_to_string(path.into_owned())?;
    println!("{}", contents);
    Ok(())
}

#[allow(dead_code)]
fn ipaddress () {
    let ip_addr: IpAddr = "192.168.1.1".parse().unwrap();
    println!("{:?}", ip_addr);
}

#[allow(dead_code)]
fn search_in_string(s: &str, substr: &str) {
    let sublen = substr.len();
    let mut subindex: usize = 0;

    for i in 0..s.len() {
        if s[i..=i] != substr[subindex..=subindex] {
            subindex = 0;
            continue;
        }
        subindex += 1;
        if subindex == sublen {
            println!("Found in index: {}, {}", i + 1 - sublen, s[i + 1 - sublen..=i].to_string());
            break;
        }
    }
}

#[allow(dead_code)]
fn transmute(s: &str) {
    type Type = u64;
    const SIZE_OF: usize = std::mem::size_of::<Type>();

    let mut tmp: Type;
    let mut p: [u8; SIZE_OF] = [0; SIZE_OF];

    for part in s.as_bytes().chunks_exact(SIZE_OF) {
        p.copy_from_slice(part);

        unsafe {
            tmp = std::mem::transmute::<[u8; SIZE_OF], Type>(p);
        }

        println!("{:X}", tmp);
    }
}
