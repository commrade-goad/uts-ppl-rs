use std::ops::{Add, Div, Mul, Sub};

pub trait Command<T> {
    fn execute(lhs: T, rhs: T) -> T;
}

pub struct AddCommand;
pub struct MinCommand;
pub struct MulCommand;
pub struct DivCommand;

impl<T> Command<T> for AddCommand
where
    T: Add<Output = T>,
{
    fn execute(lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

impl<T> Command<T> for MinCommand
where
    T: Sub<Output = T>,
{
    fn execute(lhs: T, rhs: T) -> T {
        lhs - rhs
    }
}

impl<T> Command<T> for MulCommand
where
    T: Mul<Output = T>,
{
    fn execute(lhs: T, rhs: T) -> T {
        lhs * rhs
    }
}

impl<T> Command<T> for DivCommand
where
    T: Div<Output = T>,
{
    fn execute(lhs: T, rhs: T) -> T {
        lhs / rhs
    }
}

pub struct Calculator<T> {
    pub lhs: T,
    pub rhs: T,
    pub res: T,
}

impl<T> Calculator<T>
where
    T: Copy,
{
    pub fn new(lhs: T, rhs: T) -> Self {
        Self {
            lhs,
            rhs,
            res: lhs,
        }
    }

    pub fn apply<C>(&mut self)
    where
        C: Command<T>,
    {
        self.res = C::execute(self.lhs, self.rhs);
    }
}
