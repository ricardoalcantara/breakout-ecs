#![allow(unused, dead_code)]
use breakout_ecs::world::World;

fn main() {
    let mut world = World::new();

    let a = world.spawn((123, true, "abc"));
    let b = world.spawn((42, false));

    for (id, (number)) in world.query::<(&i32,)>() {
        println!("entity: {id} - {number}")
    }

    // for (id, (number, flag)) in world.query::<(&i32, &bool)>() {
    //     println!("entity: {id} - {number} - {flag}")
    // }

    // for (id, (number, &flag)) in world.query_mut::<(&mut i32, &bool)>() {
    //     if flag {
    //         *number *= 2;
    //     }
    // }

    // assert_eq!(*world.get::<i32>(a).unwrap(), 246);
    // assert_eq!(*world.get::<i32>(b).unwrap(), 42);

    // let mut number = world.get_mut::<i32>(a).unwrap();
    // *number = 999;

    // assert_eq!(*world.get::<i32>(a).unwrap(), 999);
}
