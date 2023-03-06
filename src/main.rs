mod game_rand;
mod model;
mod particle_system;
mod vector;

use game_rand::GameRand;
use model::*;
use vector::*;
use particle_system::*;

fn main() {
    let mut test = vec3(0.0, 1.0, 0.0);
    let test2 = test * 3.0;
    test.x = 7.0;

    drop(test);
    test.y = 8.0;

    if let Ok(model) = model::Model::new("data/room0.hmdl") {
        for batch in model.batches {
            println!(
                "Vertices {} Indices {}",
                batch.num_vertices, batch.num_indices
            );
        }
    } else {
        println!("Failure to read file!");
    }

    for p in PrimitiveType::iter() {
        println!("{:?}", p);
    }

    let mut rand = GameRand::new(12345);

    for _ in 0..1_000_000 {
        println!("{}", rand.next_random());
        println!("{}", rand.next_random01());
        println!("{}", rand.rand_range(5, 1067));
        break;
    }

    //let val = rand.rand_range(&(0u32..=3));

    //println!("MyEnum: {:?} {test3}", test2);
}
