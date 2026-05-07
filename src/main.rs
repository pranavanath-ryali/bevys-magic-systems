use ecs_systems::{FunctionSystem, Query, System, SystemParamFunction, world::World};

fn main() {
    let mut world = World::default();
    world.add_data('A', 123);
    world.add_data('B', 187238);

    let mut function_system = FunctionSystem::new(test_system); 
    function_system.run(&mut world);

    let mut function_system: FunctionSystem<_, (Query<_>,)> = FunctionSystem::new(test_another_system);
    function_system.run(&mut world);

    let mut function_system: FunctionSystem<_, (Query<_>,)> = FunctionSystem::new(test_another_fucking_system);
    function_system.run(&mut world);
}

fn test_system() {
    println!("CALLED NO PARAM SYSTEM")
}

fn test_another_system(value: Query<'A'>) {
    println!("CALLED A PARAM SYSTEM: {}", *value);
}

fn test_another_fucking_system(value: Query<'B'>) {
    println!("CALLED A PARAM SYSTEM: {}", *value);
}
