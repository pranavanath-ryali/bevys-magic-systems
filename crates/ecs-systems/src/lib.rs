pub mod world;
//
// use std::{marker::PhantomData, ops::Deref};
//
// use crate::world::World;
//
// // PARAMETERS
// pub struct Query<'w> { value: &'w isize, }
// impl<'w> SystemParam<'w> for Query<'w> {
//     type Item = Query<'w>;
//     fn fetch(world: &'w mut World) -> Self::Item {
//         Query { value: world.get_data("A") }
//     }
// }
//
// impl<'w> Deref for Query<'w> {
//     type Target = isize;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// pub trait SystemParam<'w> {
//     type Item;
//     fn fetch(world: &'w mut World) -> Self::Item;
// }
//
// // ACTUAL SYSTEM
// pub trait System<'w>: 'static {
//     fn run(&mut self, world: &'w mut World);
// }
//
// pub struct FunctionSystem<'w, F, Marker: 'static>
// where
//     F: SystemParamFunction<'w, Marker>,
// {
//     func: F,
//     marker: PhantomData<Marker>,
// }
//
// impl<'w, F, Marker> FunctionSystem<'w, F, Marker>
// where
//     F: SystemParamFunction<'w, Marker>,
// {
//     pub fn new(function: F) -> Self {
//         Self {
//             func: function,
//             marker: PhantomData,
//         }
//     }
// }
//
// impl<'w, F, Marker> System<'w> for FunctionSystem<'w, F, Marker>
// where
//     F: SystemParamFunction<'w, Marker>,
// {
//     fn run(&mut self, world: &'w mut World) {
//         self.func.run(world);
//     }
// }
//
// pub trait SystemParamFunction<'w, Marker>: 'static {
//     fn run(&mut self, world: &'w mut World);
// }
//
// // NO Param
// impl<'w, Func> SystemParamFunction<'w, ()> for Func
// where
//     Func: 'static,
//     for<'a> &'a mut Func: FnMut(),
// {
//     fn run(&mut self, _: &'w mut World) {
//         fn run_function(mut f: impl FnMut()) {
//             f();
//         }
//
//         run_function(self);
//     }
// }
//
// // 1 Param
// impl<'w, Func, P1: SystemParam<'w>> SystemParamFunction<'w, (P1,)> for Func
// where
//     Func: 'static,
//     for<'a> &'a mut Func: FnMut(P1::Item),
// {
//     fn run(&mut self, world: &'w mut World) {
//         fn run_function<'w, P1: SystemParam<'w>>(mut f: impl FnMut(P1::Item), world: &'w mut World) {
//             f(P1::fetch(world));
//         }
//         run_function::<P1>(self, world);
//         // (self)(p1);
//     }
// }

use std::{marker::PhantomData, ops::Deref};

use crate::world::World;

// PARAMETERS
pub struct Query<'w, const KEY: char> {
    value: &'w isize,
}

pub trait SystemParam {
    type Item<'w>;
    fn fetch<'w>(world: &'w mut World) -> Self::Item<'w>;
}

impl<const KEY: char> SystemParam for Query<'static, KEY> {
    type Item<'w> = Query<'w, KEY>;
    fn fetch<'w>(world: &'w mut World) -> Self::Item<'w> {
        Query { value: world.get_data(&KEY) }
    }
}

impl<'w, const KEY: char> Deref for Query<'w, KEY> {
    type Target = isize;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

// ACTUAL SYSTEM
pub trait System: 'static {
    fn run(&mut self, world: &mut World);
}

pub struct FunctionSystem<F, Marker: 'static>
where
    F: SystemParamFunction<Marker>,
{
    func: F,
    marker: PhantomData<Marker>,
}

impl<F, Marker> FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    pub fn new(function: F) -> Self {
        Self {
            func: function,
            marker: PhantomData,
        }
    }
}

impl<F, Marker> System for FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    fn run(&mut self, world: &mut World) {
        self.func.run(world);
    }
}

pub trait SystemParamFunction<Marker>: 'static {
    fn run(&mut self, world: &mut World);
}

// NO Param
impl<Func> SystemParamFunction<()> for Func
where
    Func: 'static,
    for<'a> &'a mut Func: FnMut(),
{
    fn run(&mut self, _: &mut World) {
        fn run_function(mut f: impl FnMut()) {
            f();
        }
        run_function(self);
    }
}

// 1 Param
impl<Func, P1: SystemParam + 'static> SystemParamFunction<(P1,)> for Func
where
    Func: 'static,
    for<'w> &'w mut Func: FnMut(P1::Item<'w>),
{
    fn run(&mut self, world: &mut World) {
        fn run_function<'w, P1: SystemParam>(
            mut f: impl FnMut(P1::Item<'w>),
            world: &'w mut World,
        ) {
            let p1 = P1::fetch(world);
            f(p1);
        }
        run_function::<P1>(self, world);
    }
}
