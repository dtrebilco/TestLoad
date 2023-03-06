mod game_rand;
mod model;
mod particle_system;
mod vector;

use game_rand::GameRand;
use model::*;
use vector::*;
use particle_system::*;

fn main() {
    let mut p = ParticleSystem::new();
    p.set_color_scheme(ColorScheme::Rainbow);

    let mut rand = GameRand::new(1235);

    p.update(0.1, &mut rand);
    p.update(0.2, &mut rand);
    p.update(0.5, &mut rand);    

    println!("Particle count {}", p.get_particle_count());
    let ia = p.get_index_array();
    println!("Index array size {}", ia.len());
    let va = p.get_vertex_array(vec3(0.0,1.0,2.0), vec3(0.0,1.0,2.0), true, false);
    println!("Vertex array size {}", va.len());

    p.spawn_rate = 0.0;
    p.update(50.0, &mut rand);
    println!("Particle count {}", p.get_particle_count());
    let ia = p.get_index_array();
    println!("Index array size {}", ia.len());
    let va = p.get_vertex_array(vec3(0.0,1.0,2.0), vec3(0.0,1.0,2.0), true, false);
    println!("Vertex array size {}", va.len());


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
/*/
    let mut rand = GameRand::new(12345);

    for _ in 0..1_000_000 {
        println!("{}", rand.next_random());
        println!("{}", rand.next_random01());
        println!("{}", rand.rand_range(5, 1067));
        break;
    }
*/
    //let val = rand.rand_range(&(0u32..=3));

    //println!("MyEnum: {:?} {test3}", test2);
}
