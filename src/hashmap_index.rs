use std::fs;
use std::collections::{ HashMap };

pub type HashMapIndexType = HashMap<String, Vec<usize>>;

#[allow(dead_code)]
pub fn build_hash_map_index (s: &str) -> HashMapIndexType {
    // let hash: BTreeMap<_,_> = s.split_whitespace().into_iter().enumerate().map(|v| (v.1, v.0)).collect();
    // let mut hash: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut word = String::with_capacity(256);
    let mut hash_map: HashMapIndexType = HashMap::new();

    for (index, char) in s.chars().enumerate() {
        if !(char.is_whitespace() || char.is_ascii_punctuation()) {
            word.push(char);
            continue;
        }

        if word.len() > 2 {
            hash_map.entry(word.clone())
                .and_modify(|v| v.push(index - word.len()))
                .or_insert(vec![index - word.len()]);
        }

        word.clear();
    }

    hash_map
}


#[allow(dead_code)]
pub fn build_hash_map_index_from_file(path: &str) -> std::io::Result<HashMapIndexType> {
    let data = fs::read_to_string(path)?;
    Ok(build_hash_map_index(&data))
}
