use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use crate::bounded2d::{Array2DIndex, BoundedLinear2DArray};


#[derive(Debug, Clone, Eq, PartialEq)]
enum Drainage {
    Unknown,
    Ridge,
    DrainsTo(Array2DIndex),
    Pending,
}


fn ensure_drainage_initialized(target: &mut BoundedLinear2DArray<Drainage>, elevation: &BoundedLinear2DArray<i32>, start: Array2DIndex) {
    if target[start] == Drainage::Unknown {
        target[start] = Drainage::Pending;
        let mut drain_found = None;

        for idx in target.direct_neighbours_of(start) {
            if elevation[idx] <= elevation[start] {
                ensure_drainage_initialized(target, elevation, idx);

                if Drainage::Ridge == target[idx] {
                    target[start] = Drainage::Ridge;
                    return;
                } else if let Drainage::DrainsTo(sink) = target[idx] {
                    if let Some(d) = drain_found {
                        if d != sink {
                            target[start] = Drainage::Ridge;
                            return;
                        }
                    } else {
                        drain_found = Some(sink)
                    }
                }
            }
        }
        if let Some(drain) = drain_found {
            target[start] = Drainage::DrainsTo(drain)
        } else {
            panic!("No drainage identified for ({},{})", start.0, start.1)
        }
    }
}

pub fn solve(input: &str) -> String {
    let line_width = input.find('\n').unwrap();
    let mut nrs = Vec::with_capacity(input.len());
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            nrs.push(c as i32 - '0' as i32)
        }
    }

    let mut drainage = BoundedLinear2DArray::new(vec![Drainage::Unknown; nrs.len()], line_width, Drainage::Ridge);
    let mut total_risk = 0;
    let mut drains = Vec::new();
    let map = BoundedLinear2DArray::new(nrs, line_width, 10);

    for position in map.indices() {
        let value_here = map[position];

        if value_here == 9 {
            drainage[position] = Drainage::Ridge
        } else if map.direct_neighbours_of(position).all(|neighbour| map[neighbour] > value_here) {
            drains.push((position, 0));
            total_risk += 1 + value_here;
            drainage[position] = Drainage::DrainsTo(position)
        }
    }

    for position in map.indices() {
        ensure_drainage_initialized(&mut drainage, &map, position);
    }

    for next in drainage.iter() {
        if let Drainage::DrainsTo(pos) = next {
            for (drain_pos, count) in &mut drains {
                if pos == drain_pos {
                    *count += 1
                }
            }
        }
    }
    let mut drain_sizes = drains.into_iter().map(|(_,count )|count).collect::<Vec<_>>();
    drain_sizes.sort();
    let best_three = drain_sizes[drain_sizes.len() - 1] *
        drain_sizes[drain_sizes.len() - 2] *
        drain_sizes[drain_sizes.len() - 3];
    format!("Total risk level: {}, Three largest basins: {}", total_risk, best_three)
}