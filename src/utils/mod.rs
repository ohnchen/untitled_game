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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    Rock(i32), 
    Seed(i32),
}

 impl Item {
    pub fn get_name(&self) -> String {
        match self {
            Item::Rock(_) => "Rock".to_string(),
            Item::Seed(_) => "Seed".to_string(),
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            Item::Rock(x) => *x,
            Item::Seed(x) => *x,
        }
    }

    pub fn add(&mut self, num: i32) -> Self {
        match self {
           Item::Rock(ref x) => Item::Rock(x+num),
           Item::Seed(ref x) => Item::Seed(x+num),
        } 
    }

     pub fn is_more(&self, other: Item) -> bool {
         match (*self, other) {
             (Item::Rock(x), Item::Rock(y)) => {
                 if x >= y {
                    return true;
                 } else {
                    return false;
                }
             },
             (Item::Seed(x), Item::Seed(y)) => {
                if x >= y {
                    return true;
                } else {
                    return false;
                }
            },
             _ => false,
         }
     }
 }

impl Sub for Item {
    type Output = Item;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Item::Rock(x), Item::Rock(y)) => {
                if x > y {
                    Item::Rock(x-y)
                } else {
                    Item::Rock(y-x)
                }
            },
            (Item::Seed(x), Item::Seed(y)) => {
                if x > y {
                    Item::Seed(x-y)
                } else {
                    Item::Seed(y-x)
                }
            },
            _ => self,
        }
    }

}
