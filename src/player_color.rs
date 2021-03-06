use chessbik_board::PieceColor;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerColor {
    WHITE,
    BLACK
}

impl PlayerColor {
    pub fn eq_piece_color(&self, c: PieceColor) -> bool {
        match self {
            PlayerColor::WHITE => c == PieceColor::WHITE,
            PlayerColor::BLACK => c == PieceColor::BLACK,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            PlayerColor::WHITE => PlayerColor::BLACK,
            PlayerColor::BLACK => PlayerColor::WHITE ,
        }
    }

    pub fn piece(&self) -> PieceColor {
        match self {
            PlayerColor::WHITE => PieceColor::WHITE,
            PlayerColor::BLACK => PieceColor::BLACK ,
        }
    }
}