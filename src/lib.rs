use std::collections::{ HashMap, HashSet};

pub type WordDict = HashMap<String, u32>;
pub type WordIdVec = Vec<u32>;
pub type ProcessedLine = (u32, usize, Vec<String>);

const HASH_BASE: u32 = 1779033703;
const HASH_MODULE: u32 = 2^31 - 1;
const FNV_PRIME: u32 = 16777619;
const FNV_OFFSET_BASIS: u32 = 2166136261;

pub fn process_line(line: &str) -> ProcessedLine {
    let line_string = line.to_owned();
    let line_split = line_string.split_whitespace();
    let mut id_string = String::new();
    let mut rest_words = Vec::new();
    let mut word_counter = 0;
    for word in line_split {
        if word_counter == 0 {
            id_string = word.to_owned();
        } else {
            rest_words.push(word.to_owned());
        }
        word_counter += 1;
    }
    let id = id_string.parse::<u32>().unwrap();
    (id, word_counter-1, rest_words)
}

fn fnv_hash(n: u32) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;
    for i in 0 .. 4 {
        let octet = 255 & (n >> (i * 4));
        hash ^= octet;
        hash = hash.wrapping_mul(FNV_PRIME);
        // hash += (hash << 1) + (hash << 4) + (hash << 7) + (hash << 8) + (hash << 24);
    }
    hash
}

fn fnv_hash_slice(slice: &[u32]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;
    for i in slice {
        hash ^= fnv_hash(*i);
    }
    hash
}

fn hash_slice(slice: &[u32]) -> u32 {
    let mut hash = HASH_BASE;
    for i in slice {
        hash = hash.wrapping_mul(HASH_BASE.wrapping_add(2*i));
    }
    return hash;
}

pub fn fnv_hash_pair(sentence_mapping: &Vec<u32>) -> (u32, u32) {
    let length = sentence_mapping.len();
    let hash_init = fnv_hash_slice(&sentence_mapping[0 .. 5]);
    let hash_tail = fnv_hash_slice(&sentence_mapping[(length - 5) .. length]);
    (hash_init, hash_tail)
}

pub fn hash_pair(sentence_mapping: &Vec<u32>) -> (u32, u32) {
    let length = sentence_mapping.len();
    /*
    let half = length /2;
    let second_half = if length % 2 == 1 { half + 1 } else { half };
    */
    let hash_init = hash_slice(&sentence_mapping[0 .. 5]);
    let hash_tail = hash_slice(&sentence_mapping[(length - 5) .. length]);
    (hash_init, hash_tail)
}

pub fn edit_distance_le_one(v1: &WordIdVec, v2: &WordIdVec) -> bool {
    let len1 = v1.len();
    let len2 = v2.len();
    if len1 > len2 {
        return different_len_edit_dist(v1, v2);
    } else if len2 > len1 {
        return different_len_edit_dist(v2, v1);
    } else {
        return same_len_edit_dist(v1, v2);
    }
}

fn different_len_edit_dist(v1: &WordIdVec, v2: &WordIdVec) -> bool {
    let mut local_v1 = v1.to_owned();
    let mut local_v2 = v2.to_owned();
    while local_v2.len() > 0 {
        let val1 = local_v1.pop();
        let val2 = local_v2.pop();
        if val1 != val2 {
            let v1_last = local_v1.pop();
            return v1_last == val2 && identical(&local_v1, &local_v2);
        }
    }
    local_v1.len() == 1
}

fn same_len_edit_dist(v1: &WordIdVec, v2: &WordIdVec) -> bool {
    let mut local_v1 = v1.to_owned();
    let mut local_v2 = v2.to_owned();
    while local_v1.len() > 0 {
        let val1 = local_v1.pop();
        let val2 = local_v2.pop();
        if val1 != val2 {
            return identical(&local_v1, &local_v2);
        }
    }
    true
}

fn identical(v1: &WordIdVec, v2: &WordIdVec) -> bool {
    v1.len() == v2.len() && (0 .. v1.len()).all (|i| v1.get(i) == v2.get(i))
}

fn map_word_to_id(word: String, next_word_id: u32, word_map: &mut WordDict) -> (u32, u32) {
    if word_map.is_empty() {
        word_map.insert(word,0);
        return(0,0);
    }
    let word_id = word_map.entry(word).or_insert(next_word_id + 1);
    let mut next_id = next_word_id;
    if word_id <= &mut next_id {
        (*word_id, next_word_id)
    } else {
        (*word_id, next_word_id + 1)
    }
}

pub fn map_words_to_vec_u32(words: Vec<String>, next_word_id: u32, word_map: &mut WordDict) -> (WordIdVec, u32) {
    let mut word_ids = Vec::new();
    let mut next_id = next_word_id;
    for word in words {
        let (current_word_id, id) = map_word_to_id(word, next_id, word_map);
        next_id = id;
        word_ids.push(current_word_id);
    }
    (word_ids, next_id)
}

pub fn jaccard(list1: &Vec<usize>, list2: &Vec<usize>) -> f64 {
    let set1 = from_vec(list1);
    let set2 = from_vec(list2);
    let union = set1.union(&set2).count() as f64;
    let intersection = set1.intersection(&set2).count() as f64;
    intersection / union
}

pub fn from_vec(list: &Vec<usize>) -> HashSet<usize> {
    let mut set = HashSet::with_capacity(list.len());
    for u in list {
        set.insert(*u);
    }
    set
}

#[test]
fn test_from_vec() {
    assert_eq!(3, from_vec(&vec![1,1,1,2,2,2,3,3,3]).len());
}

#[test]
fn test_jaccard() {
    let doc1 = vec![2,3,4,2];
    let doc2 = vec![1,5,4,2];
    let doc3 = vec![1];
    assert_eq!(0.4, jaccard(&doc1, &doc2));
    assert_eq!(1.0, jaccard(&doc1, &doc1));
    assert_eq!(0.0, jaccard(&doc1, &doc3));
}

#[test]
fn test_process_line() {
    let s = "42 is the answer";
    assert_eq!((42, 3, vec!["is".to_owned(),"the".to_owned(),"answer".to_owned()]), process_line(&s));
}

#[test]
fn test_map_word_id() {
    let mut map = HashMap::<String, u32>::new();
    map.insert("Doctor".to_owned(), 0);
    map.insert("Who".to_owned(), 1);
    map.insert("TARDIS".to_owned(), 2);
    let next_word_id = 2;

    let doctor = "Doctor".to_owned();
    let clara = "Clara".to_owned();
    assert_eq!((0,2), map_word_to_id(doctor, next_word_id, &mut map));
    assert_eq!((3,3), map_word_to_id(clara, next_word_id, &mut map));
    assert!(map.contains_key(&"Clara".to_owned()));
}

#[test]
fn test_map_words_to_vec_u32() {
    let words = vec!["I'm".to_owned(), "the".to_owned(), "doctor".to_owned()];
    let next_word_id = 0;
    let mut map = HashMap::new();
    let (id_vec, next_id) = map_words_to_vec_u32(words, next_word_id, &mut map);
    assert_eq!(2, next_id);
    assert_eq!(vec![0,1,2], id_vec);
    assert_eq!(1, *map.get(&"the".to_owned()).unwrap());
}

#[test]
fn test_hash_slice() {
    let v = vec![1000,212234,3000,4000000,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,1234567];
    assert!(1 != hash_slice(&v[1 .. 5]));
}

#[test]
fn test_edit_distance_le_one() {
    let v1 = vec![0,0,0,1,0,0];
    let v2 = vec![0,0,0,1,0,1];
    let v3 = vec![0,0,0,1,0,1,2];
    assert!(!edit_distance_le_one(&v1, &Vec::new()));
    assert!(edit_distance_le_one(&v1,&v2));
    assert_eq!(6, v1.len());
    assert!(!edit_distance_le_one(&v1,&v3));
    assert_eq!(6, v1.len());
    assert!(edit_distance_le_one(&v2,&v3));
    assert_eq!(6, v2.len());
    assert!(!edit_distance_le_one(&v3,&v1));
    assert_eq!(6, v2.len());
}

#[test]
fn test_different_len_edit_dist() {
    let v1 = vec![1,2,3,4,5,5];
    let v2 = vec![1,2,3,4,5];
    let v3 = vec![1,2,3,4];
    let v4 = vec![1,1,2,3,4];
    let v5 = vec![7,2,3,4,5,5];

    assert!(different_len_edit_dist(&v1, &v2));
    assert!(different_len_edit_dist(&v2, &v3));
    assert!(different_len_edit_dist(&v4, &v3));
    assert!(! different_len_edit_dist(&v1, &v4));
    assert!(! different_len_edit_dist(&v1, &v3));
}

#[test]
fn test_same_len_edit_dist() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![1,2,3,4,2];
    let v3 = vec![2,2,3,4,2];
    assert!(same_len_edit_dist(&v1, &v2));
    assert!(! same_len_edit_dist(&v1, &v3));
    assert!(same_len_edit_dist(&v2, &v3));
}

#[test]
fn test_identical() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![1,2,3,4,2];
    assert! (identical(&v1, &v1));
    assert! (! identical(&v1, &v2));
    assert! (identical(&vec![], &vec![]));
}

#[test]
fn test_hash_pair() {
    let v = vec![1,2,3,4,5,11,12,13,14,15];
    let v_init = vec![1,2,3,4,5];
    let v_tail = vec![11,12,13,14,15];
    let hash_v = hash_pair(&v);
    let hash_init = hash_slice(&v_init[0..5]);
    let hash_tail = hash_slice(&v_tail[0..5]);

    assert_eq!((hash_init, hash_tail), hash_v);
}

#[test]
fn test_fnv_hash() {
    assert!(1 != fnv_hash(1));
}

#[test]
fn test_fnv_hash_slice() {
    let v = vec![1,2,3,4,5,11,12,13,14,15];
    let v_init = vec![1,2,3,4,5];
    let v_tail = vec![11,12,13,14,15];
    let (hash_init, hash_tail) = fnv_hash_pair(&v);
    assert_eq!(fnv_hash_slice(&v[5..10]), fnv_hash_slice(&v_tail[0..5]));
    assert_eq!(fnv_hash_slice(&v[0..5]), fnv_hash_slice(&v_init[0..5]));
    assert_eq!(fnv_hash_slice(&v_init[0..5]), hash_init);
    assert_eq!(fnv_hash_slice(&v_tail[0..5]), hash_tail);
}
