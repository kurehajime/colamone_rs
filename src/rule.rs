pub static PIECES: [[usize; 9]; 17] = [
    [
        0, 0, 0, // -8
        0, 0, 0, //
        0, 1, 0,
    ],
    [
        0, 1, 0, //-7
        0, 0, 0, //
        0, 1, 0,
    ],
    [
        0, 1, 0, //-6
        0, 0, 0, //
        1, 0, 1,
    ],
    [
        1, 0, 1, // -5
        0, 0, 0, //
        1, 0, 1,
    ],
    [
        1, 0, 1, // -4
        0, 0, 0, //
        1, 1, 1,
    ],
    [
        1, 1, 1, // -3
        0, 0, 0, //
        1, 1, 1,
    ],
    [
        1, 0, 1, //-2
        1, 0, 1, //
        1, 1, 1,
    ],
    [
        1, 1, 1, //-1
        1, 0, 1, //
        1, 1, 1,
    ],
    [
        0, 0, 0, // 0
        0, 0, 0, //
        0, 0, 0,
    ],
    [
        1, 1, 1, // 1
        1, 0, 1, //
        1, 1, 1,
    ],
    [
        1, 1, 1, // 2
        1, 0, 1, //
        1, 0, 1,
    ],
    [
        1, 1, 1, // 3
        0, 0, 0, //
        1, 1, 1,
    ],
    [
        1, 1, 1, // 4
        0, 0, 0, //
        1, 0, 1,
    ],
    [
        1, 0, 1, // 5
        0, 0, 0, //
        1, 0, 1,
    ],
    [
        1, 0, 1, // 6
        0, 0, 0, //
        0, 1, 0,
    ],
    [
        0, 1, 0, // 7
        0, 0, 0, //
        0, 1, 0,
    ],
    [
        0, 1, 0, // 8
        0, 0, 0, //
        0, 0, 0,
    ],
];
pub const NUMBERS: [usize; 36] = [
    0, 1, 2, 3, 4, 5, //
    10, 11, 12, 13, 14, 15, //
    20, 21, 22, 23, 24, 25, //
    30, 31, 32, 33, 34, 35, //
    40, 41, 42, 43, 44, 45, //
    50, 51, 52, 53, 54, 55,
];
pub type MapArray = Vec<isize>;
pub type Hand = (usize, usize);
pub type HandNode = (Hand, MapArray);

pub fn get_can_move_panel_x(panel_num: usize, map: &MapArray) -> Vec<usize> {
    let mut can_move: Vec<usize> = vec![];
    let number: isize = map[panel_num];
    let panel_num = panel_num as isize;
    let x = panel_num / 10;
    let y = panel_num % 10;
    // アガリのコマは動かしたらダメ。何も無いマスも動かしようがない。
    if (number > 0 && y == 0) || (number < 0 && y == 5) || number == 0 {
        return can_move;
    }
    for i in 0..9 {
        let ii = i as isize;
        if PIECES[(number + 8) as usize][i] == 0 {
            continue;
        }
        let target_x = x + (ii % 3) - 1;
        let target_y = y + (ii / 3) - 1;
        if target_y < 0 || target_y > 5 || target_x > 5 || target_x < 0 {
            continue;
        }

        let idx = (target_x * 10 + target_y) as usize;
        let target_number = map[idx];

        // 自コマとアガリのコマはとったらダメ。
        if (target_number * number > 0)
            || (target_number > 0 && target_y == 0)
            || (target_number < 0 && target_y == 5)
        {
            continue;
        }
        can_move.push(idx);
    }
    can_move
}

pub fn get_node_map(map: &MapArray, turn_player: isize) -> Vec<HandNode> {
    let mut node_list: Vec<HandNode> = vec![];
    for i in 0..36 {
        let panel_num: usize = NUMBERS[i];
        if map[panel_num] * turn_player <= 0 || map[panel_num] == 0 {
            continue;
        }
        let can_move = get_can_move_panel_x(panel_num, map);
        for num in 0..can_move.len() {
            let mut node_map: MapArray = vec![];
            node_map[can_move[num]] = node_map[panel_num];
            node_map[panel_num] = 0;
            let hand = (panel_num, can_move[num]);
            let map_array = node_map;
            node_list.push((hand, map_array));
        }
    }
    return node_list;
}
