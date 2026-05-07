pub mod world;

use std::{marker::PhantomData, ops::{Deref, DerefMut}, ptr};

use crate::world::World;

// PARAMETERS
pub struct Query<'w, const KEY: char> {
    value: &'w isize,
}
pub struct QueryMut<'w, const KEY: char> {
    value: &'w mut isize,
}

pub trait SystemParam {
    type Item<'w>;
    fn fetch<'w>(world: &'w mut World) -> Self::Item<'w>;
}

impl<const KEY: char> SystemParam for Query<'static, KEY> {
    type Item<'w> = Query<'w, KEY>;
    fn fetch<'w>(world: &'w mut World) -> Self::Item<'w> {
        Query {
            value: world.get_data(&KEY),
        }
    }
}
impl<const KEY: char> SystemParam for QueryMut<'static, KEY> {
    type Item<'w> = QueryMut<'w, KEY>;
    fn fetch<'w>(world: &'w mut World) -> Self::Item<'w> {
        QueryMut {
            value: world.get_data_mut(&KEY),
        }
    }
}

impl<'w, const KEY: char> Deref for Query<'w, KEY> {
    type Target = isize;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}
impl<'w, const KEY: char> Deref for QueryMut<'w, KEY> {
    type Target = isize;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}
impl<'w, const KEY: char> DerefMut for QueryMut<'w, KEY>  {
    fn deref_mut(&mut self) -> &mut Self::Target {
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

// 2 Param
impl<Func, P1: SystemParam + 'static, P2: SystemParam + 'static> SystemParamFunction<(P1, P2)>
    for Func
where
    Func: 'static,
    for<'w> &'w mut Func: FnMut(P1::Item<'w>, P2::Item<'w>),
{
    fn run(&mut self, world: &mut World) {
        fn run_function<'w, P1: SystemParam, P2: SystemParam>(
            mut f: impl FnMut(P1::Item<'w>, P2::Item<'w>),
            world: &'w mut World,
        ) {
            let p1 = unsafe { P1::fetch(&mut *ptr::from_mut(world)) };
            let p2 = unsafe { P2::fetch(&mut *ptr::from_mut(world)) };
            f(p1, p2);
        }
        run_function::<P1, P2>(self, world);
    }
}
