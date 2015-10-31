extern crate time;
extern crate utillib;

use std::collections::{ HashMap, HashSet };
use std::io::prelude::*;
use time::PreciseTime;
use utillib::*;

fn main() {
    let start = PreciseTime::now();
    let mut mapping = HashMap::<String, u32>::new();
    let mut buckets = HashMap::<u32, Vec<u32>>::new();
    let mut all_sentences = HashMap::<u32, Vec<u32>>::new();
    let mut next_word_id: u32 = 0;
    let mut similar_pairs = HashSet::new();
    //
    // setyp all the data structures
    let stdin = std::io::stdin();
    for maybe_line in stdin.lock().lines() {
        let line = maybe_line.unwrap();
        
        let (id, _, word_vec) = process_line(&line);
        
        let (ref word_id_vec, next_id) = map_words_to_vec_u32(
            word_vec, next_word_id, &mut mapping);

        all_sentences.insert(id, word_id_vec.to_owned());

        next_word_id = next_id;
        
        let (hash1, hash2) = hash_pair(&word_id_vec);
        
        // store twice
        buckets.entry(hash1).or_insert(Vec::new()).push(id);
        buckets.entry(hash2).or_insert(Vec::new()).push(id);
    }
    let buckets_created = PreciseTime::now();
    println!("Finished processing file. Created {} buckets in {:2} seconds.", 
             buckets.len(), start.to(buckets_created));

    //
    // everything is in place, start comparisons
    for bucket in buckets.keys() {
        let ref id_vecs = buckets[bucket];
        let bucket_size = id_vecs.len();
        for i in 0 .. bucket_size {
            let &ith_id = id_vecs.get(i).unwrap();
            let ref ith_vec = all_sentences[&ith_id];
            for j in i .. bucket_size {
                let &jth_id = id_vecs.get(j).unwrap();
                if ith_id == jth_id {
                    continue;
                }
                let ref jth_vec = all_sentences[&jth_id];
                if edit_distance_le_one(&ith_vec, &jth_vec) {
                    similar_pairs.insert((ith_id, jth_id));
                }
            }
        }
    }
    let finished = PreciseTime::now();

    println!("Finished comparisons in {:2} seconds.", buckets_created.to(finished));
    println!("Total duration was {:2} seconds.", start.to(finished));
    println!("number of pairs with edit distance <= 1: {}", similar_pairs.len());
    if similar_pairs.len() < 30 {
        for (id1, id2) in similar_pairs {
            println!("\t{} -> {}", id1, id2);
        }
    }
}
