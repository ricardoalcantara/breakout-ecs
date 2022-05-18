#![allow(unused, dead_code)]
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use breakout_ecs::world::World;

fn main() {
    let mut world = World::new();

    let b = world.spawn((42, false));
    let a = world.spawn((123, true, "abc"));

    for (id, flag) in world.query::<(&str,)>() {
        println!("entity: {id} {}", flag.borrow())
    }

    for (id, (number, flag)) in world.query::<(i32, bool)>() {
        println!("entity: {id} - {} - {}", number.borrow(), flag.borrow())
    }

    // for (id, (number, &flag)) in world.query_mut::<(&mut i32, &bool)>() {
    //     if flag {
    //         *number *= 2;
    //     }
    // }

    // let number = world.borrow_component_vec::<i32>().unwrap();
    // let flag = world.borrow_component_vec::<bool>().unwrap();
    // let zip = number.iter().zip(flag.iter());
    // let iter =
    //     zip.filter_map(|(number, flag)| Some((number.as_ref()?.borrow(), flag.as_ref()?.borrow())));

    // for (number, flag) in iter {
    //     println!("entity: _ - {number} - {flag}")
    // }

    // assert_eq!(*world.get::<i32>(a).unwrap(), 246);
    // assert_eq!(*world.get::<i32>(b).unwrap(), 42);

    // let mut number = world.get_mut::<i32>(a).unwrap();
    // *number = 999;

    // assert_eq!(*world.get::<i32>(a).unwrap(), 999);
}
