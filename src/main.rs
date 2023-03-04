mod game_rand;
mod model;

use model::*;
use game_rand::GameRand;


fn main() {

    if let Ok(model) = model::Model::new("data/room0.hmdl") {
        for batch in model.batches {
            println!("Vertices {} Indices {}", batch.num_vertices, batch.num_indices);
        }
    }
    else {
        println!("Failure to read file!");
    }


    for p in PrimitiveType::iter() {
        println!("{:?}", p);
    }

    let mut rand = GameRand::new(12345);

    for _ in 0..1_000_000 {

        println!("{}", rand.next_random());
        println!("{}", rand.rand_range(5, 1067));
        break;
    }

    //let val = rand.rand_range(&(0u32..=3));

    //println!("MyEnum: {:?} {test3}", test2);
}
