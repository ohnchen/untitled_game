pub fn saturated_sub(op1: u16, op2: u16, min: u16) -> u16 {
    let diff: i32 = op1 as i32 - op2 as i32;
    if diff < min.into() {
        return min;
    } else {
        return diff as u16;
    }
}

pub fn saturated_add(op1: u16, op2: u16, max: u16) -> u16 {
    let sum: u16 = op1 + op2;
    if sum >= max {
        return max - 1;
    } else {
        return sum as u16;
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tools {
    Pickaxe,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Items {
    Rock(u32), 
    Seed(u32),
}

