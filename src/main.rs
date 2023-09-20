use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader},
    str::from_utf8,
    sync::Arc,
};

// Representacion de las direcciones en la matriz
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Arrow {
    Diagonal,
    Horizontal,
    Vertical,
}

impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arrow::Diagonal => write!(f, "D"),
            Arrow::Horizontal => write!(f, "H"),
            Arrow::Vertical => write!(f, "V"),
        }
    }
}

// Representacion de la celda en la matriz
// Contiene el puntaje y las direcciones
#[derive(Clone, Debug)]
struct Cell {
    score: i32,
    arrow: Vec<Arrow>,
}

impl Cell {
    // Constructor de la celda vacia
    pub fn new() -> Cell {
        Cell {
            score: 0,
            arrow: Vec::new(),
        }
    }
}

// Representacion de la alineacion de secuencias
#[derive(Debug, Clone)]
struct SequenceAlignment {
    top_sequence: Arc<str>,
    side_sequence: Arc<str>,
}

impl SequenceAlignment {
    // Constructor de la alineacion de secuencias
    // Arc: Atomic Reference Counter: estructura mas eficiente par amaenjo de memoria
    pub fn new(top_sequence: &str, side_sequence: &str) -> SequenceAlignment {
        SequenceAlignment {
            top_sequence: Arc::from(top_sequence),
            side_sequence: Arc::from(side_sequence),
        }
    }
}


// Representacion de la matriz
// Contiene la matriz de celdas y la alineacion de secuencias
#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
    alignment: SequenceAlignment,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut scoring_string = String::new();
        scoring_string.push('\n');
        scoring_string.push_str("Scoring Grid: \n");
        for row in &self.grid {
            for cell in row {
                scoring_string.push_str(&format!("{:4}", cell.score));
            }
            scoring_string.push('\n');
        }

        let mut arrow_string = String::new();
        arrow_string.push('\n');
        arrow_string.push_str("Arrow Grid: \n");
        const CELL_WIDTH: usize = 4; // Define a fixed width for each cell
        for row in &self.grid {
            for cell in row {
                let mut cell_content = String::new();
                if !cell.arrow.is_empty() {
                    for arrow in &cell.arrow {
                        let arrow_display = match arrow {
                            Arrow::Diagonal => "D",
                            Arrow::Horizontal => "H",
                            Arrow::Vertical => "V",
                        };
                        cell_content.push_str(arrow_display);
                    }
                }

                while cell_content.len() < CELL_WIDTH {
                    cell_content.push(' ');
                }
                arrow_string.push_str(&cell_content);
            }
            arrow_string.push('\n');
        }
        write!(f, "{} {}", scoring_string, arrow_string)
    }
}

impl Grid {
    // Constructor de la matriz
    pub fn new(alignment: &SequenceAlignment) -> Grid {
        // Crear matriz con la dimension de secuencias + 2
        let mut grid = vec![
            vec![Cell::new(); alignment.side_sequence.len() + 1];
            alignment.top_sequence.len() + 1
        ];

        // Inicializar la primera fila y columna dejan la esquina superior vacia

        // Por cada fila y columna, asignar el puntaje y la direccion
        for i in 1..=alignment.top_sequence.len() {
            grid[i][0].score = -1 * (i as i32);
            grid[i][0].arrow.push(Arrow::Vertical); // Vertical arrow
        }
        for j in 1..=alignment.side_sequence.len() {
            grid[0][j].score = -1 * (j as i32);
            grid[0][j].arrow.push(Arrow::Horizontal); // Horizontal arrow
        }
        let alignment = SequenceAlignment {
            top_sequence: alignment.top_sequence.clone(),
            side_sequence: alignment.side_sequence.clone(),
        };
        Grid { grid, alignment }
    }

    // Calcula los puntajes de la matriz
    pub fn compute_scores(&mut self) {
        // Recorremos filas y columnas para modificar cada Celda
        for i in 1..=self.alignment.top_sequence.len() {
            for j in 1..=self.alignment.side_sequence.len() {
                // Calculamos el puntaje de match o mismatch
                let match_score = if self.alignment.top_sequence.chars().nth(i - 1)
                    == self.alignment.side_sequence.chars().nth(j - 1)
                {
                    1
                } else {
                    -1
                };

                let indel_score = -1;

                // Calculamos el puntaje de cada direccion
                let score_from_left = self.grid[i][j - 1].score + indel_score;
                let score_from_top = self.grid[i - 1][j].score + indel_score;
                let score_from_diagonal = self.grid[i - 1][j - 1].score + match_score;

                // Calculamos el puntaje maximo
                let max_score = score_from_left.max(score_from_top).max(score_from_diagonal);

                // Asignamos el puntaje maximo a la celda
                self.grid[i][j].score = max_score;
            }
        }
    }

    // Calcula las direcciones de la matriz
    pub fn compute_arrows(&mut self) {
        // Recorremos filas y columnas para modificar cada Celda
        for i in 1..=self.alignment.top_sequence.len() {
            for j in 1..=self.alignment.side_sequence.len() {
                // Calculamos el puntaje de cada direccion
                let score_from_left = self.grid[i][j - 1].score;
                let score_from_top = self.grid[i - 1][j].score;
                let score_from_diagonal = self.grid[i - 1][j - 1].score;

                // Calculamos el puntaje maximo
                let max_score = score_from_left.max(score_from_top).max(score_from_diagonal);
                let mut arrows = Vec::new(); // Vector de direcciones por si necesitamos guardar
                                             // varias direcciones

                // Revisamos si el puntaje maximo es igual al puntaje de cada direccion
                if score_from_diagonal == max_score {
                    arrows.push(Arrow::Diagonal);
                }
                // Si dos o mas celdas tienen el mismo puntaje, se agregan todas las direcciones
                if score_from_left == max_score {
                    arrows.push(Arrow::Horizontal);
                }
                // Por eso no usamos else en estos condicionales
                if score_from_top == max_score {
                    arrows.push(Arrow::Vertical);
                }

                // Guardamos las direcciones en la celda
                self.grid[i][j].arrow = arrows;
            }
        }
    }

    // Encuentra los caminos de la matriz
    pub fn find_paths(&self) -> Vec<Vec<Arrow>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();

        // Funcion recursiva para encontrar los caminos
        fn backtrack(
            grid: &Grid,
            row: usize,
            col: usize,
            current_path: &mut Vec<Arrow>,
            paths: &mut Vec<Vec<Arrow>>,
        ) {
            // Si llegamos a la esquina superior izquierda, agregamos el camino a la lista de caminos
            if row == 0 && col == 0 {
                paths.push(current_path.clone());
                return;
            }

            // Recorremos las direcciones de la celda
            for &arrow in &grid.grid[row][col].arrow {
                // Agregamos la direccion al camino y llamamos a la funcion recursiva
                current_path.push(arrow);
                // Dependiendo de la direccion, cambiamos la fila y columna
                match arrow {
                    Arrow::Diagonal => backtrack(grid, row - 1, col - 1, current_path, paths),
                    Arrow::Vertical => backtrack(grid, row - 1, col, current_path, paths),
                    Arrow::Horizontal => backtrack(grid, row, col - 1, current_path, paths),
                }
                // Removemos la direccion del camino
                current_path.pop();
            }
        }

        // Llamamos a la funcion recursiva
        let last_row = self.grid.len() - 1;
        let last_col = self.grid[0].len() - 1;
        backtrack(&self, last_row, last_col, &mut current_path, &mut paths);

        // Retornamos los caminos
        paths
    }
}

// Lee el archivo FASTA y lo corta a un largo especifico
// ESTO SI ES DE PAQUETE ME DIO WEBA HACERLO
// Solo modifique a retornar Arc<str> en vez de Strings
fn read_and_clip_fasta(filename: &str, clip_length: usize) -> Result<Arc<str>, io::Error> {
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
fn build_alignment_from_path(
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

// Funcion principal
fn main() {
    // let string1 = "GCATGCG";
    // let string2 = "GATTACA";
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let fasta1 = read_and_clip_fasta(&format!("{}/src/org1.fasta", path), 30).unwrap();
    let fasta2 = read_and_clip_fasta(&format!("{}/src/org2.fasta", path), 30).unwrap();
    // println!("Fasta 1: {}", fasta1);
    // println!("Fasta 2: {}", fasta2);
    let alignment = SequenceAlignment::new(&fasta1, &fasta2);
    // let alignment = SequenceAlignment::new(string1, string2);
    let mut grid = Grid::new(&alignment);
    grid.compute_scores();

    grid.compute_arrows();

    let paths = grid.find_paths();
    for path in paths {
        let top_sequence = alignment.top_sequence.clone();
        let side_sequence = alignment.side_sequence.clone();
        std::thread::spawn(move || {
            let potential_alignment =
                build_alignment_from_path(&*top_sequence, &*side_sequence, &path);
            println!("Alignment Candidate:");
            println!("Top Sequence: {:?}", potential_alignment.top_sequence);
            println!("Side Sequence: {:?}", potential_alignment.side_sequence);
        });
    }
    println!("Grid Struct: {}", grid);
}
