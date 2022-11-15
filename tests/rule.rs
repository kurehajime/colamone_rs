use std::collections::HashMap;

use colamone::{
    ai::think_ai,
    rule::{self, get_can_move_panel_x, is_draw, is_end_x, MapArray},
};
use serde_json::Value;

extern crate colamone;

fn conv_map(map: &HashMap<isize, isize>) -> MapArray {
    let mut rtn: MapArray = vec![0; 56];
    for (key, value) in map {
        rtn[*key as usize] = *value as isize;
    }
    return rtn;
}
fn json_to_map(json: &str) -> MapArray {
    let v: Value = serde_json::from_str(json).unwrap();
    let mut map: MapArray = vec![0; 56];
    for (key, value) in v.as_object().unwrap().into_iter() {
        map.insert(
            key.parse::<usize>().unwrap(),
            value.as_i64().unwrap() as isize,
        );
    }
    map
}

#[test]
fn test_get_can_move_panel_x() {
    let mut map = HashMap::new();
    map.insert(11, 1);
    map.insert(31, 2);
    map.insert(13, 3);
    map.insert(33, 4);

    //1の動きテスト
    let can_move = get_can_move_panel_x(11, &conv_map(&map));
    assert_eq!(can_move, vec![0, 10, 20, 1, 21, 2, 12, 22]);
    //2の動きテスト
    let can_move = get_can_move_panel_x(31, &conv_map(&map));
    assert_eq!(can_move, vec![20, 30, 40, 21, 41, 22, 42]);
    //3の動きテスト
    let can_move = get_can_move_panel_x(13, &conv_map(&map));
    assert_eq!(can_move, vec![2, 12, 22, 4, 14, 24]);
    //4の動きテスト
    let can_move = get_can_move_panel_x(33, &conv_map(&map));
    assert_eq!(can_move, vec![22, 32, 42, 24, 44]);

    let mut map = HashMap::new();
    map.insert(11, 5);
    map.insert(31, 6);
    map.insert(13, 7);
    map.insert(33, 8);

    //5の動きテスト
    let can_move = get_can_move_panel_x(11, &conv_map(&map));
    assert_eq!(can_move, vec![0, 20, 2, 22]);
    //6の動きテスト
    let can_move = get_can_move_panel_x(31, &conv_map(&map));
    assert_eq!(can_move, vec![20, 40, 32]);
    //7の動きテスト
    let can_move = get_can_move_panel_x(13, &conv_map(&map));
    assert_eq!(can_move, vec![12, 14]);
    //8の動きテスト
    let can_move = get_can_move_panel_x(33, &conv_map(&map));
    assert_eq!(can_move, vec![32]);

    let mut map = HashMap::new();
    map.insert(22, 1);
    map.insert(23, 8);

    //障害物のテスト
    let can_move = get_can_move_panel_x(23, &conv_map(&map));
    let expect: Vec<usize> = vec![];
    assert_eq!(can_move, expect);
}

#[test]
fn test_is_end_x() {
    // 勝敗はついてない
    let mut map = HashMap::new();
    map.insert(11, 6);
    map.insert(51, -2);
    assert_eq!(is_end_x(&conv_map(&map), false), 0);
    assert_eq!(is_draw(&conv_map(&map)), false);

    // 青が勝つ
    let mut map = HashMap::new();
    map.insert(40, 6);
    map.insert(50, 2);
    map.insert(10, -2);
    map.insert(15, 7);
    assert_eq!(is_end_x(&conv_map(&map), false), 1);
    assert_eq!(is_draw(&conv_map(&map)), false);

    // 青が判定勝ち
    let mut map = HashMap::new();
    map.insert(40, 3);
    map.insert(11, -2);
    map.insert(15, -1);
    assert_eq!(is_end_x(&conv_map(&map), false), 1);
    assert_eq!(is_draw(&conv_map(&map)), false);

    // 赤が勝つ
    let mut map = HashMap::new();
    map.insert(45, -6);
    map.insert(55, -2);
    map.insert(15, 2);
    map.insert(10, -7);
    assert_eq!(is_end_x(&conv_map(&map), false), -1);
    assert_eq!(is_draw(&conv_map(&map)), false);

    // 青が勝つ
    let mut map = HashMap::new();
    map.insert(40, 6);
    map.insert(11, -2);
    map.insert(15, -6);
    assert_eq!(is_end_x(&conv_map(&map), false), 0);
    assert_eq!(is_draw(&conv_map(&map)), true);
}

#[test]
fn test_think() {
    let mut map = json_to_map(
        "{
        0: -1, 10: -2, 20: -3, 30: -4, 40: -5, 50: -6,
        1: 0, 11: -8, 21: 0, 31: 0, 41: -7, 51: 0,
        2: 0, 12: 0, 22: 0, 32: 0, 42: 0, 52: 0,
        3: 0, 13: 0, 23: 0, 33: 0, 43: 0, 53: 0,
        4: 0, 14: 7, 24: 0, 34: 0, 44: 8, 54: 0,
        5: 6, 15: 5, 25: 4, 35: 3, 45: 2, 55: 1,
    }",
    );
    let mut turn_player = 1;
    let mut end;
    let mut count = 0;
    let result = json_to_map(
        "
        {
            0: 5, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0,
            10: 6, 11: 0, 12: 0, 13: 0, 14: 0, 15: 0,
            20: 0, 21: 0, 22: 0, 23: 0, 24: 0, 25: 0,
            30: 0, 31: 0, 32: 0, 33: 0, 34: 0, 35: 0,
            40: 0, 41: 0, 42: 0, 43: 0, 44: 0, 45: -2,
            50: -6, 51: 0, 52: 0, 53: 0, 54: 0, 55: 1
        }",
    );
    while count <= 255 {
        count += 1;
        if count > 255 {
            break;
        }
        let hand = think_ai(&map, turn_player, 3, None, None, None).0;
        if hand != None {
            map[hand.unwrap().1] = map[hand.unwrap().0];
            map[hand.unwrap().0] = 0;
        }
        if rule::is_draw(&map) == true {
            break;
        }
        end = rule::is_end_x(&map, false);
        if end == 1 || end == -1 {
            break;
        }
        turn_player = turn_player * -1
    }

    assert_eq!(map, result);
}
