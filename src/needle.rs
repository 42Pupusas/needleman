use needleman::utils::grid::Grid;
use needleman::utils::helpers::{build_alignment_from_path, read_and_clip_fasta};
use needleman::utils::models::SequenceAlignment;
use std::thread;
use std::time::Instant;

// Funcion principal
fn main() {
    let start = Instant::now();

    //let string1 = "GCATGCG";
    //let string2 = "GATTACA";
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let fasta1 = read_and_clip_fasta(&format!("{}/src/utils/seqs/org1.fasta", path), 30).unwrap();
    let fasta2 = read_and_clip_fasta(&format!("{}/src//utils/seqs/org2.fasta", path), 30).unwrap();
    let duration = start.elapsed();
    println!("Fasta: {:?}", duration);
    // println!("Fasta 1: {}", fasta1);
    // println!("Fasta 2: {}", fasta2);
    let alignment = SequenceAlignment::new(&fasta1, &fasta2);
    // let alignment = SequenceAlignment::new(string1, string2);
    let mut grid = Grid::new(&alignment);
    let duration = start.elapsed();
    println!("Grid: {:?}", duration);
    grid.compute_scores();
    let duration = start.elapsed();
    println!("Scores: {:?}", duration);

    grid.compute_arrows();
    let duration = start.elapsed();
    println!("Arrows: {:?}", duration);
    println!("Grid Struct: {}", grid);

    let paths = grid.find_paths();
    let path_lenght = paths.len();
    let duration = start.elapsed();
    println!("Paths: {:?}", duration);
    println!("Alignments: {}", path_lenght);
        let thread_start = Instant::now();
        for path in paths {
            let top_sequence = alignment.top_sequence.clone();
            let side_sequence = alignment.side_sequence.clone();
            let handle = 
                thread::spawn(move || {
            let potential_alignment =
                build_alignment_from_path(&*top_sequence, &*side_sequence, &path);
            let duration = thread_start.elapsed();
            println!("Alignment: {:?}", duration);
            println!("{}\n{}", potential_alignment.top_sequence, potential_alignment.side_sequence);
            });
            handle.join().unwrap();
        }
    let duration = start.elapsed();
    println!("Finished Alignments: {:?}", duration);
}
