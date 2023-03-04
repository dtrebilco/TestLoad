
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
