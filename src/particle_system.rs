use crate::game_rand::GameRand;
use crate::vector::*;

use std::mem::size_of;

pub enum ColorScheme {
    Fire,
    Ice,
    Smoke,
    Rainbow,
}

struct Particle {
    pos: vec3,
    size: f32,

    dir: vec3,
    life: f32,
    inv_initial_life: f32,
}

pub struct PointForce {
    pos: vec3,
    strength: f32,
    linear_attenuation: f32,
    quadratic_attenuation: f32,
}

pub struct ParticleSystem {

    pub pos: vec3,
    pub spawn_rate: f32,
    pub speed: f32,
    pub speed_spread: f32,
    pub size: f32,
    pub size_spread: f32,
    pub life: f32,
    pub life_spread: f32,
    pub friction_factor: f32,

    pub colors: [vec4; 12],
    pub point_forces: Vec<PointForce>,
    pub directional_force: vec3,

    last_time: f32, 
    particle_credit: f32,

    particles: Vec<Particle>,
    vertex_array: Vec<u8>,
    index_array: Vec<u16>,
}

impl ParticleSystem {

    pub fn new() -> ParticleSystem {
        ParticleSystem {
            pos : vec3(0.0, 0.0, 0.0),
            spawn_rate : 10.0,
            speed : 100.0,
            speed_spread : 25.0,
            size : 100.0,
            size_spread : 10.0,
            life : 2.5,
            life_spread : 0.5,
            friction_factor : 0.7,

            colors : [vec4(0.0,0.0,0.0,0.0); 12], 
            point_forces : Vec::with_capacity(2),
            directional_force : vec3(0.0,0.0, 0.0),

            last_time : 0.0,
            particle_credit : 0.0,

            particles : Vec::with_capacity(20),
            vertex_array : Vec::new(),
            index_array : Vec::new(),
        }
    }

    pub fn get_particle_count(&self) -> usize {
        self.particles.len()
    }

    pub fn set_color_scheme(&mut self, color_scheme: ColorScheme) {
        match color_scheme {
            ColorScheme::Fire => {
                for i in 0..4 {
                    let f = i as f32;
                    self.colors[i] = vec4(f / 4.0, 0.0, 0.0, 0.0);
                    self.colors[i + 4] = vec4(1.0, f / 4.0, 0.0, 0.0);
                    self.colors[i + 8] = vec4((3.0 - f) / 3.0, (3.0 - f) / 3.0, 1.0, 0.0);
                }
            }
            ColorScheme::Ice => {
                for i in 0..6 {
                    let f = i as f32;
                    self.colors[i] = vec4(0.0, 0.0, f / 6.0, 0.0);
                    self.colors[i + 6] = vec4(f / 5.0, 1.0, 1.0, 0.0);
                }
            }
            ColorScheme::Smoke => {
                for i in 0..12 {
                    let f: f32 = i as f32 / 44.0;
                    self.colors[i] = vec4(f, f, f, f);
                }
            }
            ColorScheme::Rainbow => {
                self.colors[0] = vec4(0.0, 0.0, 0.0, 0.0);
                self.colors[1] = vec4(0.0, 0.0, 0.25, 0.0);
                self.colors[2] = vec4(0.0, 0.0, 0.5, 0.0);
                self.colors[3] = vec4(0.0, 0.0, 1.0, 0.0);
                self.colors[4] = vec4(0.0, 0.5, 1.0, 0.0);
                self.colors[5] = vec4(0.0, 1.0, 1.0, 0.0);
                self.colors[6] = vec4(0.0, 1.0, 0.5, 0.0);
                self.colors[7] = vec4(0.0, 1.0, 0.0, 0.0);
                self.colors[8] = vec4(0.5, 1.0, 0.0, 0.0);
                self.colors[9] = vec4(1.0, 1.0, 0.0, 0.0);
                self.colors[10] = vec4(1.0, 0.5, 0.0, 0.0);
                self.colors[11] = vec4(1.0, 0.0, 0.0, 0.0);
            }
        }
    }

    pub fn update(&mut self, time_stamp: f32, rand : &mut GameRand) {
        let time = time_stamp - self.last_time;
        self.last_time = time_stamp;

        self.particle_credit += time * self.spawn_rate;
        let len = self.particle_credit as u32;
        self.particle_credit -= len as f32;

        let mut random = |mean: f32, diff: f32| {
            let r: f32 = 2.0 * rand.next_random01() - 1.0;
            mean + r * r.abs() * diff
        };

        for _ in 0..len {
            let life = random(self.life, self.life_spread);
            let p = Particle {
                pos: self.pos,
                dir: normalize(&vec3(random(0.0, 0.3), 1.0, random(0.0, 0.3)))
                    * random(self.speed, self.speed_spread),

                size: random(self.size, self.size_spread),
                life,
                inv_initial_life: 1.0 / life,
            };
            self.particles.push(p);
        }

        let friction = self.friction_factor.powf(time);

        self.particles.retain_mut(|p| {
            p.life -= time;
            if p.life < 0.0 {
                return false;
            }

            let mut v = vec3(0.0, 0.0, 0.0);
            for f in &self.point_forces {
                let dir = f.pos - p.pos;
                let dist = dot(&dir, &dir);
                v += dir
                    * (f.strength
                        / (1.0
                            + dist.sqrt() * f.linear_attenuation
                            + dist * f.quadratic_attenuation));
            }

            p.dir += (self.directional_force + v) * time;
            p.dir *= friction;

            p.pos += p.dir * time;
            true
        });
    }

    pub fn update_time(&mut self, time_stamp: f32) {
        self.last_time = time_stamp;
    }

    pub fn get_index_array(&mut self) -> &[u16] {
        let old_size = self.index_array.len();
        let new_size = self.particles.len() * 6;

        if new_size > old_size {
            self.index_array.reserve(new_size);

            let start_index = (old_size / 6) as u16;
            let end_index = self.particles.len() as u16;
            for i in start_index..end_index {
                self.index_array.push(4 * i);
                self.index_array.push(4 * i + 1);
                self.index_array.push(4 * i + 3);
                self.index_array.push(4 * i + 3);
                self.index_array.push(4 * i + 1);
                self.index_array.push(4 * i + 2);
            }
        }

        &self.index_array
    }

    unsafe fn copy_to_buffer<T>(buffer: *mut u8, data: T) -> *mut u8 {
        let data = &data as *const T as * const u8;

        buffer.copy_from_nonoverlapping(data, size_of::<T>());
        buffer.add(size_of::<T>())
    }

    pub fn get_vertex_array(&mut self, dx: vec3, dy: vec3, use_colors: bool, tex3d: bool) -> &[u8] {
        let mut vertex_size = size_of::<vec3>() + size_of::<vec2>();
        if use_colors {
            vertex_size += size_of::<vec4>();
        }
        if tex3d {
            vertex_size += size_of::<f32>();
        }

        let size = self.particles.len() * vertex_size * 4;
        if size > self.vertex_array.len() {
            self.vertex_array.resize(size, 0);
        }

        const COORDS : [vec2; 4] = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
        let vect  = [-dx + dy, dx + dy, dx - dy, -dx - dy];

        let mut frac : f32 = 0.0;
        let mut color : vec4 = vec4(0.0,0.0,0.0,0.0);
        let dest_range = self.vertex_array.as_mut_ptr_range();
        let mut dest_ptr = dest_range.start;
        for p in &self.particles {

            if use_colors || tex3d {
                frac = p.life * p.inv_initial_life;
            }

            if use_colors {
                let mut col_frac = 11.0 * frac;
                let col_int = col_frac as i32;
                col_frac -= col_int as f32;
    
                color = lerp(&self.colors[col_int as usize], &self.colors[col_int as usize + 1], col_frac);
            }

            for j in 0..4 {

                let pos = p.pos + vect[j] * p.size;
                unsafe {
                    dest_ptr = Self::copy_to_buffer(dest_ptr, pos);
                    dest_ptr = Self::copy_to_buffer(dest_ptr, COORDS[j]);

                    if tex3d {
                        dest_ptr = Self::copy_to_buffer(dest_ptr, 1.0 - frac);
                    }
        
                    if use_colors {
                        dest_ptr = Self::copy_to_buffer(dest_ptr, color);
                    }
                }
            }
        }
        debug_assert!(dest_ptr <= dest_range.end);

        &self.vertex_array
    }
}

