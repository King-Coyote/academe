use crate::prelude::*;
use std::collections::VecDeque;

pub trait Condition {
    fn is_valid(&self, ctx: &BeingContext) -> bool;
}

impl<F> Condition for F 
where F: Fn(&BeingContext) -> bool {
    fn is_valid(&self, ctx: &BeingContext) -> bool {
        self(ctx)
    }
}

pub trait Operator {
    fn update(&self, ctx: &mut BeingContext) -> TaskStatus;
    fn stop(&self, ctx: &mut BeingContext);
}

impl<F> Operator for F
where F: Fn(&mut BeingContext) -> TaskStatus 
{
    fn update(&self, ctx: &mut BeingContext) -> TaskStatus {
        self(ctx)
    }

    fn stop(&self, ctx: &mut BeingContext) {

    }
}

pub trait Effect {
    fn apply(&self, ctx: &mut BeingContext);
}

impl<F> Effect for F
where F: Fn(&mut BeingContext) -> ()
{
    fn apply(&self, ctx: &mut BeingContext) {
        self(ctx)
    }
}

pub type Plan = VecDeque<usize>;