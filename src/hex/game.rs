use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
enum Cell {
    White,
    Black,
    Empty
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    size: u8,
    rows: Vec<Vec<Cell>>,
}

impl Board {

    // todo: use this
    #[allow(dead_code)]
    pub fn new(size: u8) -> Self {
        Self { size, rows: vec![vec![Cell::Empty; size as usize]; size as usize] }
    }
}
//
// impl From<T> for Board{
//
//     // todo: finish this
//     fn from(source: impl Read) -> Self {
//         // for line in source.lines(){
//         //
//         // }
//
//         Board::new(3)
//     }
// }

// impl Display for Board{
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "board")
//     }
// }


#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_board_initialization() {
        let _b = Board::new(3);
    }

    #[test]
    fn test_board_serialization() {
        let b = Board::new(3);

        let r = serde_json::to_string(&b).unwrap();
        println!("{:?}", r);
    }


    // #[test]
    // fn test_board_display() {
    //     let b = Board::new(3);
    //
    //     assert_eq!(b.to_string(), "a b c")
    // }

}