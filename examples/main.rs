#![allow(unused, dead_code)]
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    time::Instant,
};

fn main() {
    let start = Instant::now();
    breakout_ecs();
    let duration = start.elapsed();
    println!("Time elapsed in breakout_ecs() is: {:?}", duration);

    let start = Instant::now();
    hecs_ecs();
    let duration = start.elapsed();
    println!("Time elapsed in hecs_ecs() is: {:?}", duration);
}

fn breakout_ecs() {
    let mut world = breakout_ecs::world::World::new();

    for i in 0..100_000 {
        world.spawn((42 + i, false));
        world.spawn((123 + i, true, "abc"));
    }

    for _ in 0..10 {
        let mut i = 0;
        for (id, flag) in world.query::<(&str,)>() {
            i += 1;
            // println!("entity: {id} {}", flag.borrow())
        }

        for (id, (number, flag)) in world.query::<(i32, bool)>() {
            i += *number.borrow();
            // println!("entity: {id} - {} - {}", number.borrow(), flag.borrow())
        }
    }
}

fn hecs_ecs() {
    let mut world = hecs::World::new();

    for i in 0..100_000 {
        world.spawn((42 + i, false));
        world.spawn((123 + i, true, "abc"));
    }

    for _ in 0..10 {
        let mut i = 0;
        for (id, (flag)) in world.query::<(&&str)>().iter() {
            // println!("entity: {id:?} {}", flag)
            i += 1;
        }

        for (id, (number, flag)) in world.query::<(&i32, &bool)>().iter() {
            // println!("entity: {id:?} - {} - {}", number, flag)
            i += number;
        }
    }
}
