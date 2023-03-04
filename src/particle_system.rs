
enum ColorScheme {
	Fire,
	Ice,
	Smoke,
	Rainbow
}

struct Particle {
	//vec3 pos;
	size: f32,

	//vec3 dir;
	life: f32,
    invInitialLife: f32,

	depth: f32, 

	angle: f32, 
	angleSpeed: f32, 
}

struct PointForce {
	//vec3 pos;
	strength: f32,
	linearAttenuation: f32, 
	quadraticAttenuation: f32,
}

struct ParticleSystem {
	particles: std::Vec<Particle>, 
	pointForces: std::Vec<PointForce>, 
	//vec3 directionalForce;
	
	lastTime: f32, 
    particleCredit : f32,

	//vec4 colors[12];
	//vec3 pos;

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
