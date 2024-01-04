use std::ops::Sub;

pub fn saturated_sub(op1: usize, op2: usize, min: usize) -> usize {
    let diff: isize = op1 as isize - op2 as isize;
    if diff < min as isize {
        return min;
    } else {
        return diff as usize;
    }
}

pub fn saturated_add(op1: usize, op2: usize, max: usize) -> usize {
    let sum: usize = op1 + op2;
    if sum >= max {
        return max - 1;
    } else {
        return sum as usize;
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tool {
    Pickaxe,
    Hoe,
    FishingRod,
    Bucket,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Item {
    Rock, 
    Seed,
}

impl Item {
    pub fn get_name(&self) -> String {
        match self {
            Item::Rock => "Rock".to_owned(),
            Item::Seed => "Seed".to_owned(),
        }
    }
}
    
