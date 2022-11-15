use crate::eval::{self};
use crate::rule::{self, Hand, MapArray};

pub fn deep_think_all_ab(
    map: &MapArray,
    turn_player: isize,
    depth: isize,
    a: Option<isize>,
    b: Option<isize>,
    nearwin: bool,
    evalparam: [[isize; 6]; 9],
) -> (Option<Hand>, isize) {
    let mut best_score: isize = turn_player * 9999999 * -1;
    let mut besthand: Option<Hand> = None;
    let mut a = a;
    let mut b = b;
    if depth == 0 {
        best_score = eval::eval_map(map, nearwin, evalparam);
        return (None, best_score);
    }
    if a == None || b == None {
        a = Some(9999999 * turn_player * -1);
        b = Some(9999999 * turn_player);
    }

    let node_list = rule::get_node_map(map, turn_player);
    for i in 0..node_list.len() {
        let hand: Hand = node_list.get(i).unwrap().0;
        let map0 = &node_list.get(i).unwrap().1;
        let sc: isize;
        // 必勝
        let end = rule::is_end_x(map0, nearwin);
        if end == turn_player {
            return (Some(hand), 999999 * turn_player);
        }
        // 必敗
        if end == turn_player * -1 {
            if besthand == None {
                best_score = 999999 * turn_player * -1;
                besthand = Some(hand);
            }
            continue;
        }
        if rule::is_none_node(map0) {
            sc = 0;
        } else {
            sc = deep_think_all_ab(map0, turn_player * -1, depth - 1, b, a, nearwin, evalparam).1;
        }
        if besthand == None {
            best_score = sc;
            besthand = Some(hand);
        }
        if turn_player == 1 && sc > best_score {
            best_score = sc;
            besthand = Some(hand);
        } else if turn_player == -1 && sc < best_score {
            best_score = sc;
            besthand = Some(hand);
        }
        if turn_player == 1 && a.unwrap() < best_score
            || turn_player == -1 && a.unwrap() > best_score
        {
            a = Some(best_score);
        }
        if turn_player == 1 && b.unwrap() <= best_score
            || turn_player == -1 && b.unwrap() >= best_score
        {
            break;
        }
    }
    return (besthand, best_score);
}
