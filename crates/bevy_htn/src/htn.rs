use crate::prelude::*;
use std::collections::VecDeque;

pub trait Condition {
    fn is_valid(&self, ctx: &WorldContext) -> bool;
}

impl<F> Condition for F 
where F: Fn(&WorldContext) -> bool {
    fn is_valid(&self, ctx: &WorldContext) -> bool {
        self(ctx)
    }
}

pub trait Operator {
    fn update(&self, ctx: &mut WorldContext) -> TaskStatus;
    fn stop(&self, ctx: &mut WorldContext);
}

impl<F> Operator for F
where F: Fn(&mut WorldContext) -> TaskStatus 
{
    fn update(&self, ctx: &mut WorldContext) -> TaskStatus {
        self(ctx)
    }

    fn stop(&self, ctx: &mut WorldContext) {

    }
}

pub trait Effect {
    fn apply(&self, ctx: &mut WorldContext);
}

impl<F> Effect for F
where F: Fn(&mut WorldContext) -> ()
{
    fn apply(&self, ctx: &mut WorldContext) {
        self(ctx)
    }
}

pub type Plan = VecDeque<usize>;