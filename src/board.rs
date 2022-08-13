use crate::types::BoardInfo;
use crate::{piece_type, win_type};

const ROW: usize = 5;
const COL: usize = 5;

const DIRS: [[[usize; 2]; 5]; 4] = [
    [[!1, 0], [!0, 0], [0, 0], [1, 0], [2, 0]],
    [[0, !1], [0, !0], [0, 0], [0, 1], [0, 2]],
    [[!1, !1], [!0, !0], [0, 0], [1, 1], [2, 2]],
    [[2, !1], [1, !0], [0, 0], [!0, 1], [!1, 2]],
];

// pure function
fn logic(bin: BoardInfo, index: usize) -> i32 {
    let mut lines = 0;
    let (p0, p1) = (index / COL, index % COL);
    for dir in DIRS {
        let mut line = 0;
        for dp in dir {
            let (dp0, dp1) = (p0 + dp[0], p1 + dp[1]);
            if dp0 < ROW && dp1 < COL {
                if bin.Board[dp0 * COL + dp1] == bin.Turn {
                    line += 1;
                    if line >= 3 {
                        break;
                    }
                } else {
                    line = 0
                }
            }
        }
        if line >= 3 {
            lines += 1;
            if lines >= 2 {
                break;
            }
        }
    }
    if lines >= 2 {
        if bin.Turn == piece_type::BLACK {
            return win_type::BLACK;
        } else {
            return win_type::WHITE;
        }
    } else if lines == 1 {
        if bin.Turn == piece_type::BLACK {
            return win_type::WHITE;
        } else {
            return win_type::BLACK;
        }
    } else {
        let mut full = true;
        for v in bin.Board {
            if v == piece_type::NULL {
                full = false;
                break;
            }
        }
        if full {
            return win_type::TIE;
        }
    }
    return win_type::NULL;
}
