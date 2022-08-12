enum PlayerType {
  None,
  Host,
  Guest,
  Observer,
}

enum RoomState {
  Available,
  Full,
  InGame,
}

enum PieceType {
  Null,
  Black,
  White,
}

enum WinType {
  Null,
  Tie,
  Black,
  White,
  Flee,
}

struct RoomInfo {
  Player1: String,
  Player2: String,
  State: RoomState
}

struct RoomDetail  {
  Player1: String,
  Player2: String,
  P1Ready: bool,
  P2Ready: bool,
}

struct BoardInfo {
  Board: Vec<PieceType>,
  Turn: PieceType,
  Result: WinType
}
