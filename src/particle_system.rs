use crate::game_rand::GameRand;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(PartialEq, Copy, Clone)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(PartialEq, Copy, Clone)]
pub struct vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,	
}

enum ColorScheme {
    Fire,
    Ice,
    Smoke,
    Rainbow
}

struct Particle {
    pos: vec3,
    size: f32,

    dir: vec3,
    life: f32,
    invInitialLife: f32,

    depth: f32, 

    angle: f32, 
    angleSpeed: f32, 
}

struct PointForce {
    pos: vec3,
    strength: f32,
    linearAttenuation: f32, 
    quadraticAttenuation: f32,
}

struct ParticleSystem {
    particles: Vec<Particle>, 
    pointForces: Vec<PointForce>, 
    directionalForce: vec3, 
    
    lastTime: f32, 
    particleCredit : f32,

    colors: [vec4; 12],
    pos: vec3,

    spawnRate: f32,
    speed: f32,
    speedSpread: f32,
    size: f32,
    sizeSpread: f32,
    life: f32,
    lifeSpread: f32,
    frictionFactor: f32,

    rotate : bool,

    //char *vertexArray;
    vertexArraySize : u32,

    //unsigned short *indexArray;
    indexArraySize : u32,

    rand: GameRand,
}

impl ParticleSystem {
    fn random(&mut self, mean: f32, diff: f32) -> f32 {
        let r: f32 = 2.0 * self.rand.next_random01() - 1.0;
        return mean + r * r.abs() * diff;
    }
}
/*
ParticleSystem();
virtual ~ParticleSystem();

const vec3 &getPosition() const { return pos; }
unsigned int getParticleCount() const { return (int)particles.size(); }
void setPosition(const vec3 &position){ pos = position; }
void setSpawnRate(const float spawnrate){ spawnRate = spawnrate; }

void setSpeed(const float meanSpeed, const float spread){
    speed = meanSpeed;
    speedSpread = spread;
}

void setLife(const float meanLife, const float spread){
    life = meanLife;
    lifeSpread = spread;
}

void setSize(const float meanSize, const float spread){
    size = meanSize;
    sizeSpread = spread;
}

void setDirectionalForce(const vec3 &df){ directionalForce = df; }
void addPointForce(const PointForce &pf){ pointForces.push_back(pf); }
void setPointForce(const int force, const PointForce &pf){ pointForces[force] = pf;	}
void setFrictionFactor(const float friction){ frictionFactor = friction; }

void setColor(const int color, const vec4 &col){ colors[color] = col; }
void setColorScheme(const COLOR_SCHEME colorScheme);

void setRotate(const bool rot){ rotate = rot; }

void update(const float timeStamp);
void updateTime(const float timeStamp);
void depthSort(const vec3 &pos, const vec3 &depthAxis);

char *getVertexArray(const vec3 &dx, const vec3 &dy, bool useColors = true, bool tex3d = false);
char *getPointSpriteArray(bool useColors = true);
unsigned short *getIndexArray();

void fillVertexArray(char *dest, const vec3 &dx, const vec3 &dy, bool useColors = true, bool tex3d = false);
void fillInstanceVertexArray(char *dest);
void fillInstanceVertexArrayRange(vec4 *posAndSize, vec4 *color, const unsigned int start, unsigned int count);
void fillIndexArray(unsigned short *dest);

protected:
virtual void initParticle(Particle &p);
virtual void updateParticle(Particle &p, const float time);
*/


/*



ParticleSystem::ParticleSystem(){
    pos = vec3(0, 0, 0);
    directionalForce = vec3(0, 0, 0);
    
    spawnRate = 10;
    speed = 100;
    speedSpread = 25;
    
    size = 100;
    sizeSpread = 10;
    
    life = 2.5f;
    lifeSpread = 0.5f;
    
    frictionFactor = 0.7f;
    
    setColorScheme(COLOR_SCHEME_FIRE);

    lastTime = 0;
    particleCredit = 0;

    rotate = false;

    vertexArray = NULL;
    vertexArraySize = 0;
    indexArray = NULL;
    indexArraySize = 0;
}

ParticleSystem::~ParticleSystem(){
    delete vertexArray;
    delete indexArray;
}

void ParticleSystem::setColorScheme(const COLOR_SCHEME colorScheme){
    float f;
    int i;
    
    switch(colorScheme){
    case COLOR_SCHEME_FIRE:
        for (i = 0; i < 4; i++){
            colors[i    ] = vec4(i / 4.0f, 0, 0, 0);
            colors[i + 4] = vec4(1, i / 4.0f, 0, 0);
            colors[i + 8] = vec4((3 - i) / 3.0f, (3 - i) / 3.0f, 1, 0);
        }
        break;
    case COLOR_SCHEME_ICE:
        for (i = 0; i < 6; i++){
            colors[i    ] = vec4(0, 0, i / 6.0f, 0);
            colors[i + 6] = vec4(i / 5.0f, 1, 1, 0);
        }
        break;
    case COLOR_SCHEME_SMOKE:
        for (i = 0; i < 12; i++){
            f = i / 44.0f;
            colors[i] = vec4(f, f, f, f);
        }
        break;
    case COLOR_SCHEME_RAINBOW:
        colors[0]  = vec4(0,   0,   0,     0);
        colors[1]  = vec4(0,   0,   0.25f, 0);
        colors[2]  = vec4(0,   0,   0.5f,  0);
        colors[3]  = vec4(0,   0,   1,     0);
        colors[4]  = vec4(0,   0.5f,1,     0);
        colors[5]  = vec4(0,   1,   1,     0);
        colors[6]  = vec4(0,   1,   0.5f,  0);
        colors[7]  = vec4(0,   1,   0,     0);
        colors[8]  = vec4(0.5f,1,   0,     0);
        colors[9]  = vec4(1,   1,   0,     0);
        colors[10] = vec4(1,   0.5f,0,     0);
        colors[11] = vec4(1,   0,   0,     0);
        break;
    }
}

void ParticleSystem::initParticle(Particle &p){
    p.pos = pos;
    p.dir = normalize(vec3(random(0, 0.3f), 1, random(0, 0.3f)));
    p.dir *= random(speed, speedSpread);
    p.size = random(size, sizeSpread);
    /*p.initialLife = */p.life = random(life, lifeSpread);
    p.invInitialLife = 1.0f / p.life;
}

void ParticleSystem::updateParticle(Particle &p, const float time){
    p.pos += p.dir * time;
}

void ParticleSystem::update(const float timeStamp){
    Particle p;
    float time, dist, friction;
    unsigned int i, j, len;

    time = timeStamp - lastTime;
    lastTime = timeStamp;

    particleCredit += time * spawnRate;
    len = (int) particleCredit;
    particleCredit -= len;

    for (i = 0; i < len; i++){
        initParticle(p);
        particles.push_back(p);
    }

    friction = powf(frictionFactor, time);

    i = 0;
    while (i < particles.size()){
        if ((particles[i].life -= time) < 0){
            particles.erase(particles.begin() + i);
            continue;
        }

        vec3 v(0, 0, 0);
        for (j = 0; j < pointForces.size(); j++){
            vec3 dir = pointForces[j].pos - particles[i].pos;
            dist = dot(dir, dir);
            v += dir * (pointForces[j].strength / (1.0f + sqrtf(dist) * pointForces[j].linearAttenuation + dist * pointForces[j].quadraticAttenuation));
        }

        particles[i].dir += (directionalForce + v) * time;
        particles[i].dir *= friction;
        if (rotate) particles[i].angle += particles[i].angleSpeed * time;

        //particles[i].pos += particles[i].dir * time;
        updateParticle(particles[i], time);

        i++;
    }
}

void ParticleSystem::updateTime(const float timeStamp){
    lastTime = timeStamp;
}

int depthComp(const Particle &elem0, const Particle &elem1){
    return (elem0.depth < elem1.depth);
}

void ParticleSystem::depthSort(const vec3 &pos, const vec3 &depthAxis){
    for (unsigned int i = 0; i < particles.size(); i++){
        particles[i].depth = fabsf(dot(particles[i].pos - pos, depthAxis));
    }
    
    std::sort(particles.begin(), particles.end(), depthComp);
    //particles.sort(depthComp);
}

char *ParticleSystem::getVertexArray(const vec3 &dx, const vec3 &dy, bool useColors, bool tex3d){
    unsigned int vertexSize = sizeof(vec3) + sizeof(vec2);
    if (useColors) vertexSize += sizeof(vec4);
    if (tex3d) vertexSize += sizeof(float);
    unsigned int size = (unsigned int)particles.size() * vertexSize * 4;

    if (size > vertexArraySize){
        delete vertexArray;
        vertexArray = new char[size];
        vertexArraySize = size;
    }

    fillVertexArray(vertexArray, dx, dy, useColors, tex3d);

    return vertexArray;
}

char *ParticleSystem::getPointSpriteArray(bool useColors){
    unsigned int vertexSize = sizeof(vec3) + sizeof(float);
    if (useColors) vertexSize += sizeof(vec4);
    unsigned int size = vertexSize * (unsigned int)particles.size();

    if (size > vertexArraySize){
        delete vertexArray;
        vertexArray = new char[size];
        vertexArraySize = size;
    }

    char *dest = vertexArray;
    for (unsigned int i = 0; i < particles.size(); i++){
        *(vec3 *) dest = particles[i].pos;
        dest += sizeof(vec3);
        *(float *) dest = particles[i].size;
        dest += sizeof(float);

        if (useColors){
            //float colFrac = (11.0f * particles[i].life) / particles[i].initialLife;
            float colFrac = 11.0f * particles[i].life * particles[i].invInitialLife;

            int colInt = (int) colFrac;
            colFrac -= colInt;

            *(vec4 *) dest = lerp(colors[colInt], colors[colInt + 1], colFrac);
            dest += sizeof(vec4);
        }
    }

    return vertexArray;
}

unsigned short *ParticleSystem::getIndexArray(){
    unsigned int size = (unsigned int)particles.size() * 6;

    if (size > indexArraySize){
        delete indexArray;
        indexArray = new unsigned short[size];
        indexArraySize = size;

        fillIndexArray(indexArray);
    }

    return indexArray;
}

void ParticleSystem::fillVertexArray(char *dest, const vec3 &dx, const vec3 &dy, bool useColors, bool tex3d){
    static vec2 coords[4] = { vec2(0, 0), vec2(1, 0), vec2(1, 1), vec2(0, 1) };
    vec3 vect[4] = { -dx + dy, dx + dy, dx - dy, -dx - dy };

    float frac = 0;
    vec4 color;
    for (unsigned int i = 0; i < particles.size(); i++){
        if (useColors || tex3d)
            frac = particles[i].life * particles[i].invInitialLife;
//			frac = particles[i].life / particles[i].initialLife;

        if (useColors){
            float colFrac = 11.0f * frac;
            int colInt = (int) colFrac;
            colFrac -= colInt;

            color = lerp(colors[colInt], colors[colInt + 1], colFrac);
        }

        if (rotate){
            float fx = 1.4142136f * cosf(particles[i].angle);
            float fy = 1.4142136f * sinf(particles[i].angle);
        
            for (unsigned int k = 0; k < 4; k++){
                vect[k] = fx * dx + fy * dy;

                float t = fy;
                fy = -fx;
                fx = t;
            }
        }

        for (unsigned int j = 0; j < 4; j++){
            *(vec3 *) dest = particles[i].pos + particles[i].size * vect[j];
            dest += sizeof(vec3);
            *(vec2 *) dest = coords[j];
            dest += sizeof(vec2);

            if (tex3d){
                *(float *) dest = 1.0f - frac;
                dest += sizeof(float);
            }

            if (useColors){
                *(vec4 *) dest = color;
                dest += sizeof(vec4);
            }
        }
    }
}

void ParticleSystem::fillInstanceVertexArray(char *dest){
    Particle *part = particles.data();
    for (unsigned int i = 0; i < particles.size(); i++){
        *(vec3 *) dest = part->pos;
        dest += sizeof(vec3);
        *(float *) dest = part->size;
        dest += sizeof(float);

        float colFrac = 11.0f * part->life * part->invInitialLife;
        int colInt = (int) colFrac;
        colFrac -= colInt;

        *(vec4 *) dest = lerp(colors[colInt], colors[colInt + 1], colFrac);
        dest += sizeof(vec4);

        part++;
    }
}

void ParticleSystem::fillInstanceVertexArrayRange(vec4 *posAndSize, vec4 *color, const unsigned int start, unsigned int count){
    Particle *part = particles.data() + start;

    for (unsigned int i = 0; i < count; i++){
        *(vec3 *) posAndSize = part->pos;
        posAndSize->w = part->size;
        posAndSize++;

        float colFrac = 11.0f * part->life * part->invInitialLife;
        int colInt = (int) colFrac;
        colFrac -= colInt;

        *color++ = lerp(colors[colInt], colors[colInt + 1], colFrac);

        part++;
    }
}

void ParticleSystem::fillIndexArray(unsigned short *dest){
    for (unsigned int i = 0; i < particles.size(); i++){
        *dest++ = 4 * i;
        *dest++ = 4 * i + 1;
        *dest++ = 4 * i + 3;
        *dest++ = 4 * i + 3;
        *dest++ = 4 * i + 1;
        *dest++ = 4 * i + 2;
    }
}


 */