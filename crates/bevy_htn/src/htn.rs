use crate::prelude::*;
use std::collections::VecDeque;

pub trait Condition<C>: Sync + Send
where C: Context
{
    fn is_valid(&self, ctx: &C) -> bool;
}

impl<C, F> Condition<C> for F 
where
    C: Context,
    F: Fn(&C) -> bool + Sync + Send,
{
    fn is_valid(&self, ctx: &C) -> bool {
        self(ctx)
    }
}

pub trait Operator<C>: Sync + Send
where
    C: Context
{
    fn update(&self, ctx: &mut C) -> TaskStatus;
    fn stop(&self, ctx: &mut C);
}

impl<C, F> Operator<C> for F 
where
    C: Context,
    F: Fn(&mut C) -> TaskStatus + Sync + Send
{
    fn update(&self, ctx: &mut C) -> TaskStatus {
        self(ctx)
    }

    fn stop(&self, ctx: &mut C) {

    }
}

pub trait Effect<C>: Sync + Send
where
    C: Context
{
    fn apply(&self, ctx: &mut C);
}

impl<C, F> Effect<C> for F 
where
    C: Context,
    F: Fn(&mut C) -> () + Sync + Send
{
    fn apply(&self, ctx: &mut C) {
        self(ctx)
    }
}

pub type Plan = VecDeque<usize>;