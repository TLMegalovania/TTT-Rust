use crate::room_state;

pub(crate) struct RoomInfo {
    pub Player1: String,
    pub Player2: String,
    pub State: i32,
}

pub(crate) struct RoomDetail {
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

pub(crate) fn roomDetailsToInfo(rd: &[RoomDetail]) -> Vec<RoomInfo> {
    let mut values = Vec::with_capacity(rd.len());
    for info in rd {
        let state;
        if info.P1Ready && info.P2Ready {
            state = room_state::IN_GAME
        } else if info.Player1.is_empty() || info.Player2.is_empty() {
            state = room_state::AVAILABLE
        } else {
            state = room_state::FULL
        }
        values.push(RoomInfo {
            Player1: info.Player1.clone(),
            Player2: info.Player2.clone(),
            State: state,
        })
    }
    return values;
}
