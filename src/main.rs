use ecs_systems::{FunctionSystem, Query, QueryMut, System, SystemParamFunction, world::World};

fn main() {
    let mut world = World::default();
    world.add_data('A', 123);
    world.add_data('B', 187238);

    let mut system: Box<dyn System> = Box::new(FunctionSystem::new(test_system));
    system.run(&mut world);
    let mut system: Box<dyn System> = Box::new(FunctionSystem::<_, (QueryMut<_>,)>::new(
        test_another_system,
    ));
    system.run(&mut world);
    let mut system: Box<dyn System> = Box::new(FunctionSystem::<_, (Query<_>, Query<_>)>::new(
        test_another_fucking_system,
    ));
    system.run(&mut world);
}

fn test_system() {
    println!("CALLED NO PARAM SYSTEM")
}

fn test_another_system(mut value: QueryMut<'A'>) {
    println!("VALUE BEFORE: {}", *value);
    *value = 10;
    println!("VALUE AFTER: {}", *value);
}

fn test_another_fucking_system(u: Query<'B'>, v: Query<'A'>) {
    println!("CALLED A PARAM SYSTEM FROM ANOTHER FUNCTION SYSTEM: {}", *u);
    println!("CALLED A PARAM SYSTEM FROM ANOTHER FUNCTION SYSTEM: {}", *v);
}
