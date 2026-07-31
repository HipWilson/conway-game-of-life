pub fn block() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (1, 1)]
}

pub fn beehive() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]
}

pub fn loaf() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)]
}

pub fn boat() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]
}

pub fn tub() -> Vec<(i32, i32)> {
    vec![(1, 0), (0, 1), (2, 1), (1, 2)]
}

pub fn blinker() -> Vec<(i32, i32)> {
    vec![(0, 0), (4, 1), (2, 0)]
}

pub fn toad() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]
}

pub fn beacon() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 3), (2, 3), (3, 3)]
}

pub fn pulsar() -> Vec<(i32, i32)> {
    let rows_edge = [2, 3, 4, 8, 9, 10];
    let rows_side = [0, 5, 7, 12];
    let mut cells = Vec::new();

    for &x in &rows_edge {
        cells.push((x, 0));
        cells.push((x, 5));
        cells.push((x, 7));
        cells.push((x, 12));
    }
    for &y in &rows_side {
        cells.push((0, y));
        cells.push((5, y));
        cells.push((7, y));
        cells.push((12, y));
    }
    cells
}

pub fn pentadecathlon() -> Vec<(i32, i32)> {
    let mut cells = vec![(2, 0), (7, 0), (2, 2), (7, 2)];
    for x in [0, 1, 3, 4, 5, 6, 8, 9] {
        cells.push((x, 1));
    }
    cells
}

pub fn glider() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}

pub fn lwss() -> Vec<(i32, i32)> {
    vec![
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ]
}

pub fn mwss() -> Vec<(i32, i32)> {
    vec![
        (3, 0),
        (1, 1), (5, 1),
        (0, 2),
        (0, 3), (5, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
    ]
}

pub fn hwss() -> Vec<(i32, i32)> {
    vec![
        (3, 0), (4, 0),
        (1, 1), (5, 1),
        (0, 2),
        (0, 3), (6, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
    ]
}