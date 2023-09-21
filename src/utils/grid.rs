use std::fmt::Display;
use crate::utils::models::{Arrow, Cell, SequenceAlignment};
// Representacion de la matriz
// Contiene la matriz de celdas y la alineacion de secuencias
#[derive(Debug)]
pub struct Grid {
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



