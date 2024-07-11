use super::*;

#[test]
fn test_discover_stations() {
    let lines: Vec<&str> = vec!["[start]"];
    let stations = discover_stations(&lines).ok().unwrap();
    let station = &stations[0];
    assert_eq!(
        station.loc,
        SourceLocation {
            line: 0,
            col: 0,
            len: 7
        }
    );
    assert!(matches!(station.t, StationType::START));
}

#[test]
fn test_discover_stations_assign() {
    let lines: Vec<&str> = vec!["[start] {test}"];
    let stations = discover_stations(&lines).ok().unwrap();
    assert_eq!(stations.len(), 2);
    let station = &stations[1];
    #[allow(unused_variables)]
    {
        let s = String::from("test");
        assert!(matches!(&station.t, StationType::ASSIGN(s)));
    }
}

#[test]
fn test_discover_stations_four() {
    let lines: Vec<&str> = vec!["[start][exit]", "[exit][exit]"];
    let stations = discover_stations(&lines);
    assert_eq!(stations.ok().unwrap().len(), 4);
}

#[test]
fn test_discover_stations_five() {
    let lines: Vec<&str> = vec![
        "[start]                                               [exit]",
        "                         [exit]                             ",
        "[exit]                                                [exit]",
    ];
    let stations = discover_stations(&lines);
    assert_eq!(stations.ok().unwrap().len(), 5);
}

#[test]
fn test_discover_stations_none() {
    let lines: Vec<&str> = vec![""];
    let stations = discover_stations(&lines);
    assert!(stations.is_err());
}
#[test]
fn test_discover_stations_two_starts() {
    let lines: Vec<&str> = vec!["[start] [start]"];
    let stations = discover_stations(&lines);
    assert!(stations.is_err());
}

#[test]
fn test_get_char_index_from_byte_offset() {
    let s = "😼abcd";
    assert_eq!(get_char_index_from_byte_offset(4, s), 1);
    assert_eq!(get_char_index_from_byte_offset(6, s), 3);
}

#[test]
fn test_get_char_index_from_byte_offset_no_unicode() {
    let s = "abcd";
    assert_eq!(get_char_index_from_byte_offset(2, s), 2);
}

#[test]
fn test_get_char_index_from_byte_offset_multiple_unicode() {
    let s = "😻a😼b😾c";
    assert_eq!(get_char_index_from_byte_offset(14, s), 5);
}

#[test]
fn test_get_neighbors() {
    let map = vec![
        vec![' ', ' ', ' ', ' '],
        vec![' ', '[', ']', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let station = Station {
        t: StationType::START,
        loc: SourceLocation {
            line: 1,
            col: 1,
            len: 2,
        },
        modifiers: StationModifiers::default(),
        in_bays: Vec::new(),
        out_bays: Vec::new(),
    };
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (0, 1, Direction::NORTH),
            (0, 2, Direction::NORTH),
            (1, 3, Direction::EAST),
            (2, 2, Direction::SOUTH),
            (2, 1, Direction::SOUTH),
            (1, 0, Direction::WEST),
        ]
    )
}

#[test]
fn test_get_neighbors_on_border() {
    let map = vec![vec![' ', '[', ']', ' ']];
    let mut station = Station {
        t: StationType::START,
        loc: SourceLocation {
            line: 0,
            col: 1,
            len: 2,
        },
        modifiers: StationModifiers::default(),
        in_bays: Vec::new(),
        out_bays: Vec::new(),
    };
    assert_eq!(
        get_neighbors(&map, &station),
        vec![(0, 3, Direction::EAST), (0, 0, Direction::WEST),]
    );
    let map = vec![vec![' ', ' '], vec!['[', ']'], vec![' ', ' ']];
    station.loc = SourceLocation {
        line: 1,
        col: 0,
        len: 2,
    };
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (0, 0, Direction::NORTH),
            (0, 1, Direction::NORTH),
            (2, 1, Direction::SOUTH),
            (2, 0, Direction::SOUTH),
        ]
    );
}

#[test]
fn test_get_neighbors_none() {
    let map = vec![vec!['[', ']']];
    let station = Station {
        t: StationType::START,
        loc: SourceLocation {
            line: 0,
            col: 0,
            len: 2,
        },
        modifiers: StationModifiers::default(),
        in_bays: Vec::new(),
        out_bays: Vec::new(),
    };
    assert_eq!(get_neighbors(&map, &station), vec![])
}

#[test]
fn test_get_neighbors_reversed() {
    let map = vec![
        vec![' ', ' ', ' ', ' '],
        vec![' ', '[', ']', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let mut station = Station {
        t: StationType::START,
        loc: SourceLocation {
            line: 1,
            col: 1,
            len: 2,
        },
        modifiers: StationModifiers::default().reverse(),
        in_bays: Vec::new(),
        out_bays: Vec::new(),
    };
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (0, 2, Direction::NORTH),
            (0, 1, Direction::NORTH),
            (1, 0, Direction::WEST),
            (2, 1, Direction::SOUTH),
            (2, 2, Direction::SOUTH),
            (1, 3, Direction::EAST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::EAST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (1, 3, Direction::EAST),
            (0, 2, Direction::NORTH),
            (0, 1, Direction::NORTH),
            (1, 0, Direction::WEST),
            (2, 1, Direction::SOUTH),
            (2, 2, Direction::SOUTH),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::SOUTH);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (2, 1, Direction::SOUTH),
            (2, 2, Direction::SOUTH),
            (1, 3, Direction::EAST),
            (0, 2, Direction::NORTH),
            (0, 1, Direction::NORTH),
            (1, 0, Direction::WEST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::WEST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (1, 0, Direction::WEST),
            (2, 1, Direction::SOUTH),
            (2, 2, Direction::SOUTH),
            (1, 3, Direction::EAST),
            (0, 2, Direction::NORTH),
            (0, 1, Direction::NORTH),
        ]
    );
}

#[test]
fn test_get_neighbors_with_direction() {
    let map = vec![
        vec![' ', ' ', ' ', ' '],
        vec![' ', '[', ']', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let mut station = Station {
        t: StationType::START,
        loc: SourceLocation {
            line: 1,
            col: 1,
            len: 2,
        },
        modifiers: StationModifiers::default().with_priority(Direction::EAST),
        in_bays: Vec::new(),
        out_bays: Vec::new(),
    };
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (1, 3, Direction::EAST),
            (2, 2, Direction::SOUTH),
            (2, 1, Direction::SOUTH),
            (1, 0, Direction::WEST),
            (0, 1, Direction::NORTH),
            (0, 2, Direction::NORTH),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::SOUTH);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (2, 2, Direction::SOUTH),
            (2, 1, Direction::SOUTH),
            (1, 0, Direction::WEST),
            (0, 1, Direction::NORTH),
            (0, 2, Direction::NORTH),
            (1, 3, Direction::EAST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::WEST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (1, 0, Direction::WEST),
            (0, 1, Direction::NORTH),
            (0, 2, Direction::NORTH),
            (1, 3, Direction::EAST),
            (2, 2, Direction::SOUTH),
            (2, 1, Direction::SOUTH),
        ]
    );
}
