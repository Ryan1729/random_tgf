extern crate rand;
use rand::{SeedableRng, StdRng, Rng};

#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("random_tgf")
    .version(crate_version!())
    .arg(
        Arg::with_name("seed")
            .short("s")
            .takes_value(true)
            .help("the seed for the pseudo-random generator"),
    )
    .get_matches();
    
    let seed: Vec<usize> = if let Some(passed_seed) = matches.value_of("seed") {
        passed_seed.as_bytes().iter().map(|&b| b as usize).collect()
    } else {
        if cfg!(debug_assertions) {
            vec![42]
        } else {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|dur| dur.as_secs())
                .unwrap_or(42);
            vec![timestamp as usize]
        }
    };
    
    let mut rng: StdRng = SeedableRng::from_seed(seed.as_slice());
    
    let mut tgf = String::new();

    let mut edges = HashSet::new();
    let mut offset = 0;
    
    let mut cluster_sizes = Vec::new();
    for _ in 0..rng.gen_range(4, 6) {
        cluster_sizes.push(rng.gen_range(4, 6));
    }
    
    let node_count = cluster_sizes.iter().sum();
    
    for cluster_size in cluster_sizes {
        let (mut current_cluster, new_offset) = cluster(&mut rng, offset, cluster_size);
        
        if new_offset < node_count {
            edges.insert((offset, rng.gen_range(new_offset, node_count)));
        }
        
        offset = new_offset;
        
        edges.extend(current_cluster.drain());
    }
    
    for i in 0..node_count {
        tgf.push_str(&format!("{}\n", i));
    }

    tgf.push_str("#\n");
    
    for &(s, t) in edges.iter().filter(|&&(s,t)| s != t) {
        if s < node_count && t < node_count {
            tgf.push_str(&format!("{} {}\n", s, t));
        }
    }
    
    println!("{}", tgf);
}

use std::collections::HashSet;

fn cluster(rng : &mut StdRng, offset: u8, nodes: u8) -> (HashSet<(u8, u8)>, u8) {
    let mut edges = HashSet::new();
    
    for i in 0..nodes {
        for j in i..nodes {
            edges.insert((i,j));
        }
    }
    
    for i in 0..nodes {
        edges.remove(&(i, i + rng.gen_range(0, nodes) % nodes));
    }
    
    ( edges.iter().map(|&(s, t)| (s + offset, t + offset)).collect(), offset + nodes)
}
