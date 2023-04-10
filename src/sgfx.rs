use windows_sys::Win32::Foundation::HINSTANCE;

use crate::enum_sequential;
use crate::EnumLoadError;

struct sg_buffer {
    id: u32,
}
struct sg_image {
    id: u32,
}
struct sg_shader {
    id: u32,
}
struct sg_pipeline {
    id: u32,
}
struct sg_pass {
    id: u32,
}
struct sg_context {
    id: u32,
}

struct sg_color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

const SG_INVALID_SLOT_INDEX: u32 = 0;
const SG_INVALID_ID: u32 = 0;
const SG_NUM_SHADER_STAGES: u32 = 2;
const SG_NUM_INFLIGHT_FRAMES: u32 = 2;
const SG_MAX_COLOR_ATTACHMENTS: u32 = 4;
const SG_MAX_SHADERSTAGE_BUFFERS: u32 = 8;
const SG_MAX_SHADERSTAGE_IMAGES: u32 = 12;
const SG_MAX_SHADERSTAGE_UBS: u32 = 4;
const SG_MAX_UB_MEMBERS: u32 = 16;
const SG_MAX_VERTEX_ATTRIBUTES: u32 = 16; /* NOTE: actual max vertex attrs can be less on GLES2, see sg_limits! */
const SG_MAX_MIPMAPS: u32 = 16;
const SG_MAX_TEXTUREARRAY_LAYERS: u32 = 128;

enum sg_backend {
    GLCORE33,
    GLES2,
    GLES3,
    D3D11,
    METAL_IOS,
    METAL_MACOS,
    METAL_SIMULATOR,
    WGPU,
    DUMMY,
}

struct sg_pool_t {
    size: i32,
    queue_top: i32,
    //uint32_t* gen_ctrs;
    //int* free_queue;
}

struct sg_pools_t {
    buffer_pool: sg_pool_t,
    image_pool: sg_pool_t,
    shader_pool: sg_pool_t,
    pipeline_pool: sg_pool_t,
    pass_pool: sg_pool_t,
    context_pool: sg_pool_t,
    //_sg_buffer_t* buffers;
    //_sg_image_t* images;
    //_sg_shader_t* shaders;
    //_sg_pipeline_t* pipelines;
    //_sg_pass_t* passes;
    //_sg_context_t* contexts;
}

enum_sequential! {
    enum sg_pixel_format {
        DEFAULT,    /* value 0 reserved for default-init */
        NONE,

        R8,
        R8SN,
        R8UI,
        R8SI,

        R16,
        R16SN,
        R16UI,
        R16SI,
        R16F,
        RG8,
        RG8SN,
        RG8UI,
        RG8SI,

        R32UI,
        R32SI,
        R32F,
        RG16,
        RG16SN,
        RG16UI,
        RG16SI,
        RG16F,
        RGBA8,
        SRGB8A8,
        RGBA8SN,
        RGBA8UI,
        RGBA8SI,
        BGRA8,
        RGB10A2,
        RG11B10F,

        RG32UI,
        RG32SI,
        RG32F,
        RGBA16,
        RGBA16SN,
        RGBA16UI,
        RGBA16SI,
        RGBA16F,

        RGBA32UI,
        RGBA32SI,
        RGBA32F,

        DEPTH,
        DEPTH_STENCIL,

        BC1_RGBA,
        BC2_RGBA,
        BC3_RGBA,
        BC4_R,
        BC4_RSN,
        BC5_RG,
        BC5_RGSN,
        BC6H_RGBF,
        BC6H_RGBUF,
        BC7_RGBA,
        PVRTC_RGB_2BPP,
        PVRTC_RGB_4BPP,
        PVRTC_RGBA_2BPP,
        PVRTC_RGBA_4BPP,
        ETC2_RGB8,
        ETC2_RGB8A1,
        ETC2_RGBA8,
        ETC2_RG11,
        ETC2_RG11SN,

        RGB9E5,
    }
}
const SG_PIXELFORMAT_NUM: u32 = sg_pixel_format::len() as u32;

struct sg_pixelformat_info {
    sample: bool, // pixel format can be sampled in shaders
    filter: bool, // pixel format can be sampled with filtering
    render: bool, // pixel format can be used as render target
    blend: bool,  // alpha-blending is supported
    msaa: bool,   // pixel format can be used as MSAA render target
    depth: bool,  // pixel format is a depth format
}

enum_sequential! {
    pub enum sg_compare_func {
        DEFAULT,    /* value 0 reserved for default-init */
        NEVER,
        LESS,
        EQUAL,
        LESS_EQUAL,
        GREATER,
        NOT_EQUAL,
        GREATER_EQUAL,
        ALWAYS,
    }
}
const SG_COMPAREFUNC_NUM: u32 = sg_compare_func::len() as u32;

enum_sequential! {
    enum sg_stencil_op {
        DEFAULT,      /* value 0 reserved for default-init */
        KEEP,
        ZERO,
        REPLACE,
        INCR_CLAMP,
        DECR_CLAMP,
        INVERT,
        INCR_WRAP,
        DECR_WRAP,
    }
}
const SG_STENCILOP_NUM: u32 = sg_stencil_op::len() as u32;

enum_sequential! {
    enum sg_blend_factor {
        DEFAULT,    /* value 0 reserved for default-init */
        ZERO,
        ONE,
        SRC_COLOR,
        ONE_MINUS_SRC_COLOR,
        SRC_ALPHA,
        ONE_MINUS_SRC_ALPHA,
        DST_COLOR,
        ONE_MINUS_DST_COLOR,
        DST_ALPHA,
        ONE_MINUS_DST_ALPHA,
        SRC_ALPHA_SATURATED,
        BLEND_COLOR,
        ONE_MINUS_BLEND_COLOR,
        BLEND_ALPHA,
        ONE_MINUS_BLEND_ALPHA,
    }
}
const SG_BLENDFACTOR_NUM: u32 = sg_blend_factor::len() as u32;

enum_sequential! {
    enum sg_blend_op {
        SG_BLENDOP_DEFAULT,    /* value 0 reserved for default-init */
        SG_BLENDOP_ADD,
        SG_BLENDOP_SUBTRACT,
        SG_BLENDOP_REVERSE_SUBTRACT,
    }
}
const SG_BLENDOP_NUM: u32 = sg_blend_op::len() as u32;

enum sg_color_mask {
    DEFAULT = 0, /* value 0 reserved for default-init */
    NONE = 0x10, /* special value for 'all channels disabled */
    R = 0x1,
    G = 0x2,
    RG = 0x3,
    B = 0x4,
    RB = 0x5,
    GB = 0x6,
    RGB = 0x7,
    A = 0x8,
    RA = 0x9,
    GA = 0xA,
    RGA = 0xB,
    BA = 0xC,
    RBA = 0xD,
    GBA = 0xE,
    RGBA = 0xF,
}

enum_sequential! {
    enum sg_cull_mode {
        DEFAULT,   /* value 0 reserved for default-init */
        NONE,
        FRONT,
        BACK,
    }
}
const SG_CULLMODE_NUM: u32 = sg_cull_mode::len() as u32;

enum_sequential! {
    enum sg_face_winding {
        DEFAULT,    /* value 0 reserved for default-init */
        CCW,
        CW,
    }
}
const SG_FACEWINDING_NUM: u32 = sg_face_winding::len() as u32;

struct sg_stencil_face_state {
    compare: sg_compare_func,
    fail_op: sg_stencil_op,
    depth_fail_op: sg_stencil_op,
    pass_op: sg_stencil_op,
}

struct sg_stencil_state {
    enabled: bool,
    front: sg_stencil_face_state,
    back: sg_stencil_face_state,
    read_mask: u8,
    write_mask: u8,
    ref_val: u8,
}

struct sg_depth_state {
    pixel_format: sg_pixel_format,
    compare: sg_compare_func,
    write_enabled: bool,
    bias: f32,
    bias_slope_scale: f32,
    bias_clamp: f32,
}

struct sg_blend_state {
    enabled: bool,
    src_factor_rgb: sg_blend_factor,
    dst_factor_rgb: sg_blend_factor,
    op_rgb: sg_blend_op,
    src_factor_alpha: sg_blend_factor,
    dst_factor_alpha: sg_blend_factor,
    op_alpha: sg_blend_op,
}

struct sg_color_state {
    pixel_format: sg_pixel_format,
    write_mask: sg_color_mask,
    blend: sg_blend_state,
}

struct sg_gl_attr_t {
    vb_index: i8, /* -1 if attr is not enabled */
    divisor: i8,  /* -1 if not initialized */
    stride: u8,
    size: u8,
    normalized: u8,
    offset: i32,
    //type_arr : GLenum,
}

struct sg_gl_cache_attr_t {
    gl_attr: sg_gl_attr_t,
    gl_vbuf: u32,
}

struct sg_gl_texture_bind_slot {
    //target : GLenum,
    texture: u32,
}

const SG_GL_IMAGE_CACHE_SIZE: u32 = SG_MAX_SHADERSTAGE_IMAGES * SG_NUM_SHADER_STAGES;
struct sg_gl_state_cache_t {
    depth: sg_depth_state,
    stencil: sg_stencil_state,
    blend: sg_blend_state,
    color_write_mask: [sg_color_mask; SG_MAX_COLOR_ATTACHMENTS as usize],
    cull_mode: sg_cull_mode,
    face_winding: sg_face_winding,
    polygon_offset_enabled: bool,
    sample_count: i32,
    blend_color: sg_color,
    alpha_to_coverage_enabled: bool,
    attrs: [sg_gl_cache_attr_t; SG_MAX_VERTEX_ATTRIBUTES as usize],
    vertex_buffer: u32,
    index_buffer: u32,
    stored_vertex_buffer: u32,
    stored_index_buffer: u32,
    prog: u32,
    textures: [sg_gl_texture_bind_slot; SG_GL_IMAGE_CACHE_SIZE as usize],
    stored_texture: sg_gl_texture_bind_slot,
    cur_ib_offset: i32,
    //GLenum cur_primitive_type;
    //GLenum cur_index_type;
    //GLenum cur_active_texture;
    //_sg_pipeline_t* cur_pipeline;
    cur_pipeline_id: sg_pipeline,
}

struct sg_gl_backend_t {
    valid: bool,
    gles2: bool,
    in_pass: bool,
    cur_pass_width: i32,
    cur_pass_height: i32,
    //_sg_context_t* cur_context;
    //_sg_pass_t* cur_pass;
    cur_pass_id: sg_pass,
    cache: sg_gl_state_cache_t,
    ext_anisotropic: bool,
    max_anisotropy: i32,

    opengl32_dll: HINSTANCE,
}

struct sg_features {
    instancing: bool,                  // hardware instancing supported
    origin_top_left: bool,             // framebuffer and texture origin is in top left corner
    multiple_render_targets: bool, // offscreen render passes can have multiple render targets attached
    msaa_render_targets: bool,     // offscreen render passes support MSAA antialiasing
    imagetype_3d: bool,            // creation of SG_IMAGETYPE_3D images is supported
    imagetype_array: bool,         // creation of SG_IMAGETYPE_ARRAY images is supported
    image_clamp_to_border: bool,   // border color and clamp-to-border UV-wrap mode is supported
    mrt_independent_blend_state: bool, // multiple-render-target rendering can use per-render-target blend state
    mrt_independent_write_mask: bool, // multiple-render-target rendering can use per-render-target color write masks
}

struct sg_limits {
    max_image_size_2d: i32,      // max width/height of SG_IMAGETYPE_2D images
    max_image_size_cube: i32,    // max width/height of SG_IMAGETYPE_CUBE images
    max_image_size_3d: i32,      // max width/height/depth of SG_IMAGETYPE_3D images
    max_image_size_array: i32,   // max width/height of SG_IMAGETYPE_ARRAY images
    max_image_array_layers: i32, // max number of layers in SG_IMAGETYPE_ARRAY images
    max_vertex_attrs: i32,       // <= SG_MAX_VERTEX_ATTRIBUTES or less (on some GLES2 impls)
    gl_max_vertex_uniform_vectors: i32, // <= GL_MAX_VERTEX_UNIFORM_VECTORS (only on GL backends)
    gl_max_combined_texture_image_units: i32, // <= GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS (only on GL backends)
}

struct sg_gl_context_desc {
    force_gles2: bool,
}

struct sg_context_desc {
    color_format: sg_pixel_format,
    depth_format: sg_pixel_format,
    sample_count: i32,
    gl: sg_gl_context_desc,
    //sg_metal_context_desc metal;
    //sg_d3d11_context_desc d3d11;
    //sg_wgpu_context_desc wgpu;
}

struct sg_desc {
    buffer_pool_size: i32,
    image_pool_size: i32,
    shader_pool_size: i32,
    pipeline_pool_size: i32,
    pass_pool_size: i32,
    context_pool_size: i32,
    uniform_buffer_size: i32,
    staging_buffer_size: i32,
    sampler_cache_size: i32,
    max_commit_listeners: i32,
    disable_validation: bool, // disable validation layer even in debug mode, useful for tests
    //sg_allocator allocator;
    //sg_logger logger; // optional log function override
    context: sg_context_desc,
}

struct sg_state_t {
    valid: bool,
    desc: sg_desc, // original desc with default values patched in
    frame_index: u32,
    active_context: sg_context,
    cur_pass: sg_pass,
    cur_pipeline: sg_pipeline,
    pass_valid: bool,
    bindings_valid: bool,
    next_draw_valid: bool,
    //sg_log_item validate_error;
    pools: sg_pools_t,
    backend: sg_backend,
    features: sg_features,
    limits: sg_limits,
    formats: [sg_pixelformat_info; SG_PIXELFORMAT_NUM as usize],
    gl: sg_gl_backend_t,
    //commit_listeners : sg_commit_listeners_t,
}

pub fn sg_setup(/*const sg_desc* desc*/) {
    /*
        SOKOL_ASSERT(desc);
        SOKOL_ASSERT((desc->_start_canary == 0) && (desc->_end_canary == 0));
        SOKOL_ASSERT((desc->allocator.alloc && desc->allocator.free) || (!desc->allocator.alloc && !desc->allocator.free));
        _SG_CLEAR_ARC_STRUCT(_sg_state_t, _sg);
        _sg.desc = _sg_desc_defaults(desc);
        _sg_setup_pools(&_sg.pools, &_sg.desc);
        _sg_setup_commit_listeners(&_sg.desc);
        _sg.frame_index = 1;
        _sg_setup_backend(&_sg.desc);
        _sg.valid = true;
        sg_setup_context();
    */
}

pub fn sg_shutdown() {}

pub fn sg_commit() {
    //todo!()
}

/*
bool sg_isvalid();
void sg_reset_state_cache();
sg_trace_hooks sg_install_trace_hooks(const sg_trace_hooks* trace_hooks);
void sg_push_debug_group(const char* name);
void sg_pop_debug_group();
bool sg_add_commit_listener(sg_commit_listener listener);
bool sg_remove_commit_listener(sg_commit_listener listener);

// resource creation, destruction and updating
sg_buffer sg_make_buffer(const sg_buffer_desc* desc);
sg_image sg_make_image(const sg_image_desc* desc);
sg_shader sg_make_shader(const sg_shader_desc* desc);
sg_pipeline sg_make_pipeline(const sg_pipeline_desc* desc);
sg_pass sg_make_pass(const sg_pass_desc* desc);
void sg_destroy_buffer(sg_buffer buf);
void sg_destroy_image(sg_image img);
void sg_destroy_shader(sg_shader shd);
void sg_destroy_pipeline(sg_pipeline pip);
void sg_destroy_pass(sg_pass pass);
void sg_update_buffer(sg_buffer buf, const sg_range* data);
void sg_update_image(sg_image img, const sg_image_data* data);
int sg_append_buffer(sg_buffer buf, const sg_range* data);
bool sg_query_buffer_overflow(sg_buffer buf);
bool sg_query_buffer_will_overflow(sg_buffer buf, size_t size);

// rendering functions
void sg_begin_default_pass(const sg_pass_action* pass_action, int width, int height);
void sg_begin_default_passf(const sg_pass_action* pass_action, float width, float height);
void sg_begin_pass(sg_pass pass, const sg_pass_action* pass_action);
void sg_apply_viewport(int x, int y, int width, int height, bool origin_top_left);
void sg_apply_viewportf(float x, float y, float width, float height, bool origin_top_left);
void sg_apply_scissor_rect(int x, int y, int width, int height, bool origin_top_left);
void sg_apply_scissor_rectf(float x, float y, float width, float height, bool origin_top_left);
void sg_apply_pipeline(sg_pipeline pip);
void sg_apply_bindings(const sg_bindings* bindings);
void sg_apply_uniforms(sg_shader_stage stage, int ub_index, const sg_range* data);
void sg_draw(int base_element, int num_elements, int num_instances);
void sg_end_pass();
void sg_commit();

// getting information
sg_desc sg_query_desc();
sg_backend sg_query_backend();
sg_features sg_query_features();
sg_limits sg_query_limits();
sg_pixelformat_info sg_query_pixelformat(sg_pixel_format fmt);
// get current state of a resource (INITIAL, ALLOC, VALID, FAILED, INVALID)
sg_resource_state sg_query_buffer_state(sg_buffer buf);
sg_resource_state sg_query_image_state(sg_image img);
sg_resource_state sg_query_shader_state(sg_shader shd);
sg_resource_state sg_query_pipeline_state(sg_pipeline pip);
sg_resource_state sg_query_pass_state(sg_pass pass);
// get runtime information about a resource
sg_buffer_info sg_query_buffer_info(sg_buffer buf);
sg_image_info sg_query_image_info(sg_image img);
sg_shader_info sg_query_shader_info(sg_shader shd);
sg_pipeline_info sg_query_pipeline_info(sg_pipeline pip);
sg_pass_info sg_query_pass_info(sg_pass pass);
// get desc structs matching a specific resource (NOTE that not all creation attributes may be provided)
sg_buffer_desc sg_query_buffer_desc(sg_buffer buf);
sg_image_desc sg_query_image_desc(sg_image img);
sg_shader_desc sg_query_shader_desc(sg_shader shd);
sg_pipeline_desc sg_query_pipeline_desc(sg_pipeline pip);
sg_pass_desc sg_query_pass_desc(sg_pass pass);
// get resource creation desc struct with their default values replaced
sg_buffer_desc sg_query_buffer_defaults(const sg_buffer_desc* desc);
sg_image_desc sg_query_image_defaults(const sg_image_desc* desc);
sg_shader_desc sg_query_shader_defaults(const sg_shader_desc* desc);
sg_pipeline_desc sg_query_pipeline_defaults(const sg_pipeline_desc* desc);
sg_pass_desc sg_query_pass_defaults(const sg_pass_desc* desc);

// separate resource allocation and initialization (for async setup)
sg_buffer sg_alloc_buffer();
sg_image sg_alloc_image();
sg_shader sg_alloc_shader();
sg_pipeline sg_alloc_pipeline();
sg_pass sg_alloc_pass();
void sg_dealloc_buffer(sg_buffer buf);
void sg_dealloc_image(sg_image img);
void sg_dealloc_shader(sg_shader shd);
void sg_dealloc_pipeline(sg_pipeline pip);
void sg_dealloc_pass(sg_pass pass);
void sg_init_buffer(sg_buffer buf, const sg_buffer_desc* desc);
void sg_init_image(sg_image img, const sg_image_desc* desc);
void sg_init_shader(sg_shader shd, const sg_shader_desc* desc);
void sg_init_pipeline(sg_pipeline pip, const sg_pipeline_desc* desc);
void sg_init_pass(sg_pass pass, const sg_pass_desc* desc);
void sg_uninit_buffer(sg_buffer buf);
void sg_uninit_image(sg_image img);
void sg_uninit_shader(sg_shader shd);
void sg_uninit_pipeline(sg_pipeline pip);
void sg_uninit_pass(sg_pass pass);
void sg_fail_buffer(sg_buffer buf);
void sg_fail_image(sg_image img);
void sg_fail_shader(sg_shader shd);
void sg_fail_pipeline(sg_pipeline pip);
void sg_fail_pass(sg_pass pass);

// rendering contexts (optional)
sg_context sg_setup_context();
void sg_activate_context(sg_context ctx_id);
void sg_discard_context(sg_context ctx_id);
*/
