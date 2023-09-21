use std::{
    fs::File,
    io::{self, BufReader},
    str::from_utf8,
    sync::Arc,
};

use super::models::{SequenceAlignment, Arrow};

// Lee el archivo FASTA y lo corta a un largo especifico
// ESTO SI ES DE PAQUETE ME DIO WEBA HACERLO
// Solo modifique a retornar Arc<str> en vez de Strings
pub fn read_and_clip_fasta(filename: &str, clip_length: usize) -> Result<Arc<str>, io::Error> {
    let reader = BufReader::new(File::open(filename)?);
    let mut sequences = seq_io::fasta::Reader::new(reader);

    if let Some(result) = sequences.records().next() {
        let record = result.unwrap();
        let sequence = record.seq;
        if sequence.len() > clip_length {
            return Ok(Arc::from(from_utf8(&sequence[0..clip_length]).unwrap()));
        } else {
            return Ok(Arc::from(from_utf8(&sequence).unwrap()));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "FASTA file is empty or has no valid records",
    ))
}

// Construye la alineacion de secuencias a partir de la direccion de la matriz
pub fn build_alignment_from_path(
    top_sequence: &str,
    side_sequence: &str,
    path: &Vec<Arrow>,
) -> SequenceAlignment {
    // Creamos dos vectores vacios para almacenar las secuencias
    let mut top_sequence_chars = Vec::new();
    let mut side_sequence_chars = Vec::new();

    // Inicializamos los indices de las secuencias
    let mut top_index = top_sequence.len() - 1;
    let mut side_index = side_sequence.len() - 1;

    // Recorremos el path de la matriz en forma reversa
    // El match asigna la funcion correspondiente a cada direccion
    for &arrow in path.iter().rev() {
        match arrow {
            Arrow::Diagonal => {
                // Agregamos los caracteres de ambas seceuncias porque hay alineacion
                top_sequence_chars.push(top_sequence.chars().nth(top_index).unwrap());
                side_sequence_chars.push(side_sequence.chars().nth(side_index).unwrap());

                // Decrementamos los indices
                if top_index > 0 {
                    top_index -= 1;
                }
                if side_index > 0 {
                    side_index -= 1;
                }
            }
            Arrow::Horizontal => {
                // Agregamos el caracter de la secuencia top y un guion al de la secuencia side
                top_sequence_chars.push(top_sequence.chars().nth(top_index).unwrap());
                side_sequence_chars.push('-');

                // Decrementamos el indice de la secuencia top
                if top_index > 0 {
                    top_index -= 1;
                }
            }
            Arrow::Vertical => {
                // Agregamos el caracter de la secuencia side y un guion al de la secuencia top
                top_sequence_chars.push('-');
                side_sequence_chars.push(side_sequence.chars().nth(side_index).unwrap());
                // Decrementamos el indice de la secuencia side
                if side_index > 0 {
                    side_index -= 1;
                }
            }
        }
    }

    // Construimos las secuencias a partir de los vectores de caracteres en reversa
    let top_sequence = Arc::from(top_sequence_chars.iter().rev().collect::<String>());
    let side_sequence = Arc::from(side_sequence_chars.iter().rev().collect::<String>());

    SequenceAlignment {
        top_sequence,
        side_sequence,
    }
}
