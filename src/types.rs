struct RoomInfo {
    pub Player1: String,
    pub Player2: String,
    pub State: i32,
}

struct RoomDetail {
    pub Player1: String,
    pub Player2: String,
    pub P1Ready: bool,
    pub P2Ready: bool,
}

pub(crate) struct BoardInfo {
    pub Board: Vec<i32>,
    pub Turn: i32,
    pub Result: i32,
}
