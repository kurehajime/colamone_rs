use std::collections::HashMap;

use colamone::rule::{get_can_move_panel_x, MapArray};

extern crate colamone;

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
