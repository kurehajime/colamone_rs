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
type MapArray = Vec<isize>;

pub fn get_can_move_panel_x(panel_num: isize, map: MapArray) -> Vec<usize> {
    let mut can_move: Vec<usize> = vec![];
    let number: isize = map[panel_num as usize];
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    fn conv_map(map: &HashMap<isize, isize>) -> MapArray {
        let mut rtn: MapArray = vec![0; 54];
        for (key, value) in map {
            rtn[*key as usize] = *value as isize;
        }
        return rtn;
    }

    #[test]
    fn test_get_can_move_panel_x() {
        let mut map = HashMap::new();
        map.insert(11, 1);
        map.insert(31, 2);
        map.insert(13, 3);
        map.insert(33, 4);

        //1の動きテスト
        let can_move = get_can_move_panel_x(11, conv_map(&map));
        assert_eq!(can_move, vec![0, 10, 20, 1, 21, 2, 12, 22]);
        //2の動きテスト
        let can_move = get_can_move_panel_x(31, conv_map(&map));
        assert_eq!(can_move, vec![20, 30, 40, 21, 41, 22, 42]);
        //3の動きテスト
        let can_move = get_can_move_panel_x(13, conv_map(&map));
        assert_eq!(can_move, vec![2, 12, 22, 4, 14, 24]);
        //4の動きテスト
        let can_move = get_can_move_panel_x(33, conv_map(&map));
        assert_eq!(can_move, vec![22, 32, 42, 24, 44]);

        let mut map = HashMap::new();
        map.insert(11, 5);
        map.insert(31, 6);
        map.insert(13, 7);
        map.insert(33, 8);

        //5の動きテスト
        let can_move = get_can_move_panel_x(11, conv_map(&map));
        assert_eq!(can_move, vec![0, 20, 2, 22]);
        //6の動きテスト
        let can_move = get_can_move_panel_x(31, conv_map(&map));
        assert_eq!(can_move, vec![20, 40, 32]);
        //7の動きテスト
        let can_move = get_can_move_panel_x(13, conv_map(&map));
        assert_eq!(can_move, vec![12, 14]);
        //8の動きテスト
        let can_move = get_can_move_panel_x(33, conv_map(&map));
        assert_eq!(can_move, vec![32]);

        let mut map = HashMap::new();
        map.insert(22, 1);
        map.insert(23, 8);

        //障害物のテスト
        let can_move = get_can_move_panel_x(23, conv_map(&map));
        assert_eq!(can_move, vec![]);
    }
}
