use std::fmt::Display;
use std::sync::Arc;

// Representacion de las direcciones en la matriz
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Arrow {
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
pub struct Cell {
    pub score: i32,
    pub arrow: Vec<Arrow>,
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
pub struct SequenceAlignment {
    pub top_sequence: Arc<str>,
    pub side_sequence: Arc<str>,
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
