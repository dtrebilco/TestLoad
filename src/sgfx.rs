// DT_TODO: Temp
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use windows_sys::s;
use windows_sys::Win32::Foundation::{HINSTANCE, PROC};
use windows_sys::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};

use crate::enum_sequential;
use crate::EnumLoadError;

#[derive(Default, Clone, Copy)]
pub struct sg_buffer {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_image {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_shader {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_pipeline {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_pass {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_context {
    id: u32,
}

#[derive(Default, Clone, Copy)]
pub struct sg_color {
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

const SG_STRING_SIZE: u32 = 16;
const SG_SLOT_SHIFT: u32 = 16;
const SG_SLOT_MASK: u32 = (1 << SG_SLOT_SHIFT) - 1;
const SG_MAX_POOL_SIZE: u32 = 1 << SG_SLOT_SHIFT;
const SG_DEFAULT_BUFFER_POOL_SIZE: u32 = 128;
const SG_DEFAULT_IMAGE_POOL_SIZE: u32 = 128;
const SG_DEFAULT_SHADER_POOL_SIZE: u32 = 32;
const SG_DEFAULT_PIPELINE_POOL_SIZE: u32 = 64;
const SG_DEFAULT_PASS_POOL_SIZE: u32 = 16;
const SG_DEFAULT_CONTEXT_POOL_SIZE: u32 = 16;
const SG_DEFAULT_SAMPLER_CACHE_CAPACITY: u32 = 64;
const SG_DEFAULT_UB_SIZE: u32 = 4 * 1024 * 1024;
const SG_DEFAULT_STAGING_SIZE: u32 = 8 * 1024 * 1024;
const SG_DEFAULT_MAX_COMMIT_LISTENERS: u32 = 1024;

#[derive(Default)]
enum sg_backend {
    #[default]
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

#[derive(Default, Clone, Copy, PartialEq)]
enum sg_resource_state {
    #[default]
    INITIAL,
    ALLOC,
    VALID,
    FAILED,
    INVALID,
}

#[derive(Default, Clone, Copy)]
struct sg_slot_t {
    id: u32,
    ctx_id: u32,
    state: sg_resource_state,
}

#[derive(Default)]
struct sg_pool_t {
    size: u32,
    queue_top: u32,
    gen_ctrs: Vec<u32>,
    free_queue: Vec<u32>,
}

#[derive(Default)]
struct sg_pools_t {
    buffer_pool: sg_pool_t,
    image_pool: sg_pool_t,
    shader_pool: sg_pool_t,
    pipeline_pool: sg_pool_t,
    pass_pool: sg_pool_t,
    context_pool: sg_pool_t,

    buffers: Vec<sg_buffer_t>,
    images: Vec<sg_image_t>,
    shaders: Vec<sg_shader_t>,
    pipelines: Vec<sg_pipeline_t>,
    passes: Vec<sg_pass_t>,
    contexts: Vec<sg_context_t>,
}

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    pub enum sg_pixel_format {
        #[default]
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

#[derive(Default, Clone, Copy)]
struct sg_pixelformat_info {
    sample: bool, // pixel format can be sampled in shaders
    filter: bool, // pixel format can be sampled with filtering
    render: bool, // pixel format can be used as render target
    blend: bool,  // alpha-blending is supported
    msaa: bool,   // pixel format can be used as MSAA render target
    depth: bool,  // pixel format is a depth format
}

enum_sequential! {

    #[derive(Default, Clone, Copy)]
    pub enum sg_compare_func {
        #[default]
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
    #[derive(Default, Clone, Copy)]
    enum sg_stencil_op {
        #[default]
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
    #[derive(Default, Clone, Copy)]
    enum sg_blend_factor {
        #[default]
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
    #[derive(Default, Clone, Copy)]
    enum sg_blend_op {
        #[default]
        DEFAULT,    /* value 0 reserved for default-init */
        ADD,
        SUBTRACT,
        REVERSE_SUBTRACT,
    }
}
const SG_BLENDOP_NUM: u32 = sg_blend_op::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_buffer_type {
        #[default]
        DEFAULT,         /* value 0 reserved for default-init */
        VERTEXBUFFER,
        INDEXBUFFER,
    }
}
const SG_BUFFERTYPE_NUM: u32 = sg_buffer_type::len() as u32;

#[derive(Default, Clone, Copy)]
enum sg_color_mask {
    #[default]
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
    #[derive(Default, Clone, Copy)]
    enum sg_cull_mode {
        #[default]
        DEFAULT,   /* value 0 reserved for default-init */
        NONE,
        FRONT,
        BACK,
    }
}
const SG_CULLMODE_NUM: u32 = sg_cull_mode::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_face_winding {
        #[default]
        DEFAULT,    /* value 0 reserved for default-init */
        CCW,
        CW,
    }
}
const SG_FACEWINDING_NUM: u32 = sg_face_winding::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_usage {
        #[default]
        DEFAULT,      /* value 0 reserved for default-init */
        IMMUTABLE,
        DYNAMIC,
        STREAM,
    }
}
const SG_USAGE_NUM: u32 = sg_usage::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_image_type {
        #[default]
        IMAGE_DEFAULT,  /* value 0 reserved for default-init */
        IMAGE_2D,
        IMAGE_CUBE,
        IMAGE_3D,
        IMAGE_ARRAY,
    }
}
const SG_IMAGETYPE_NUM: u32 = sg_image_type::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_filter {
        #[default]
        DEFAULT, /* value 0 reserved for default-init */
        NEAREST,
        LINEAR,
        NEAREST_MIPMAP_NEAREST,
        NEAREST_MIPMAP_LINEAR,
        LINEAR_MIPMAP_NEAREST,
        LINEAR_MIPMAP_LINEAR,
    }
}
const SG_FILTER_NUM: u32 = sg_filter::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_wrap {
        #[default]
        DEFAULT,   /* value 0 reserved for default-init */
        REPEAT,
        CLAMP_TO_EDGE,
        CLAMP_TO_BORDER,
        MIRRORED_REPEAT,
    }
}
const SG_WRAP_NUM: u32 = sg_wrap::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_border_color {
        #[default]
        DEFAULT,    /* value 0 reserved for default-init */
        TRANSPARENT_BLACK,
        OPAQUE_BLACK,
        OPAQUE_WHITE,
    }
}
const SG_BORDERCOLOR_NUM: u32 = sg_border_color::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_uniform_type {
        #[default]
        INVALID,
        FLOAT,
        FLOAT2,
        FLOAT3,
        FLOAT4,
        INT,
        INT2,
        INT3,
        INT4,
        MAT4,
    }
}
const SG_UNIFORMTYPE_NUM: u32 = sg_uniform_type::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_vertex_step {
        #[default]
        DEFAULT,     /* value 0 reserved for default-init */
        PER_VERTEX,
        PER_INSTANCE,
    }
}
const SG_VERTEXSTEP_NUM: u32 = sg_vertex_step::len() as u32;

#[derive(Default, Clone, Copy)]
enum sg_sampler_type {
    #[default]
    DEFAULT, /* value 0 reserved for default-init */
    FLOAT,
    SINT,
    UINT,
}

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_primitive_type {
        #[default]
        DEFAULT,  /* value 0 reserved for default-init */
        POINTS,
        LINES,
        LINE_STRIP,
        TRIANGLES,
        TRIANGLE_STRIP,
    }
}
const SG_PRIMITIVETYPE_NUM: u32 = sg_primitive_type::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_vertex_format {
        #[default]
        INVALID,
        FLOAT,
        FLOAT2,
        FLOAT3,
        FLOAT4,
        BYTE4,
        BYTE4N,
        UBYTE4,
        UBYTE4N,
        SHORT2,
        SHORT2N,
        USHORT2N,
        SHORT4,
        SHORT4N,
        USHORT4N,
        UINT10_N2,
        HALF2,
        HALF4,
    }
}
const SG_VERTEXFORMAT_NUM: u32 = sg_vertex_format::len() as u32;

enum_sequential! {
    #[derive(Default, Clone, Copy)]
    enum sg_index_type {
        #[default]
        DEFAULT,   /* value 0 reserved for default-init */
        NONE,
        UINT16,
        UINT32,
    }
}
const SG_INDEXTYPE_NUM: u32 = sg_index_type::len() as u32;

#[derive(Default, Clone, Copy)]
struct sg_stencil_face_state {
    compare: sg_compare_func,
    fail_op: sg_stencil_op,
    depth_fail_op: sg_stencil_op,
    pass_op: sg_stencil_op,
}

#[derive(Default, Clone, Copy)]
struct sg_stencil_state {
    enabled: bool,
    front: sg_stencil_face_state,
    back: sg_stencil_face_state,
    read_mask: u8,
    write_mask: u8,
    ref_val: u8,
}

#[derive(Default, Clone, Copy)]
struct sg_depth_state {
    pixel_format: sg_pixel_format,
    compare: sg_compare_func,
    write_enabled: bool,
    bias: f32,
    bias_slope_scale: f32,
    bias_clamp: f32,
}

#[derive(Default, Clone, Copy)]
struct sg_blend_state {
    enabled: bool,
    src_factor_rgb: sg_blend_factor,
    dst_factor_rgb: sg_blend_factor,
    op_rgb: sg_blend_op,
    src_factor_alpha: sg_blend_factor,
    dst_factor_alpha: sg_blend_factor,
    op_alpha: sg_blend_op,
}

#[derive(Default, Clone, Copy)]
struct sg_color_state {
    pixel_format: sg_pixel_format,
    write_mask: sg_color_mask,
    blend: sg_blend_state,
}

#[derive(Default, Clone, Copy)]
struct sg_gl_attr_t {
    vb_index: i8, /* -1 if attr is not enabled */
    divisor: i8,  /* -1 if not initialized */
    stride: u8,
    size: u8,
    normalized: u8,
    offset: i32,
    type_arr: GLenum,
}

#[derive(Default)]
struct sg_gl_cache_attr_t {
    gl_attr: sg_gl_attr_t,
    gl_vbuf: u32,
}

#[derive(Default, Clone, Copy)]
struct sg_gl_texture_bind_slot {
    target: GLenum,
    texture: u32,
}

#[derive(Default, Clone, Copy)]
struct sg_pass_attachment_common_t {
    image_id: sg_image,
    mip_level: i32,
    slice: i32,
}

#[derive(Default, Clone, Copy)]
struct sg_buffer_common_t {
    size: i32,
    append_pos: i32,
    append_overflow: bool,
    update_frame_index: u32,
    append_frame_index: u32,
    num_slots: i32,
    active_slot: i32,
    type_val: sg_buffer_type,
    usage: sg_usage,
}

#[derive(Default, Clone, Copy)]
struct sg_image_common_t {
    upd_frame_index: u32,
    num_slots: i32,
    active_slot: i32,
    type_val: sg_image_type,
    render_target: bool,
    width: i32,
    height: i32,
    num_slices: i32,
    num_mipmaps: i32,
    usage: sg_usage,
    pixel_format: sg_pixel_format,
    sample_count: i32,
    min_filter: sg_filter,
    mag_filter: sg_filter,
    wrap_u: sg_wrap,
    wrap_v: sg_wrap,
    wrap_w: sg_wrap,
    border_color: sg_border_color,
    max_anisotropy: u32,
    min_lod: f32,
    max_lod: f32,
}

#[derive(Default, Clone, Copy)]
struct GLBuffer_Data {
    buf: [u32; SG_NUM_INFLIGHT_FRAMES as usize],
    ext_buffers: bool, /* if true, external buffers were injected with sg_buffer_desc.gl_buffers */
}

#[derive(Default, Clone, Copy)]
struct sg_gl_buffer_t {
    slot: sg_slot_t,
    cmn: sg_buffer_common_t,
    gl: GLBuffer_Data,
}

type sg_buffer_t = sg_gl_buffer_t;

#[derive(Default, Clone, Copy)]
struct GLImage_Data {
    target: GLenum,
    depth_render_buffer: u32,
    msaa_render_buffer: u32,
    tex: [u32; SG_NUM_INFLIGHT_FRAMES as usize],
    ext_textures: bool, /* if true, external textures were injected with sg_image_desc.gl_textures */
}

#[derive(Default, Clone, Copy)]
struct sg_gl_image_t {
    slot: sg_slot_t,
    cmn: sg_image_common_t,
    gl: GLImage_Data,
}
type sg_image_t = sg_gl_image_t;

#[derive(Default, Clone, Copy)]
struct sg_gl_uniform_t {
    gl_loc: i32,
    type_val: sg_uniform_type,
    count: u16,
    offset: u16,
}

#[derive(Default, Clone, Copy)]
struct sg_gl_uniform_block_t {
    num_uniforms: i32,
    uniforms: [sg_gl_uniform_t; SG_MAX_UB_MEMBERS as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_gl_shader_image_t {
    gl_tex_slot: i32,
}

#[derive(Default, Clone, Copy)]
struct sg_str_t {
    buf: [u8; SG_STRING_SIZE as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_gl_shader_attr_t {
    name: sg_str_t,
}

#[derive(Default, Clone, Copy)]
struct sg_shader_uniform_block_t {
    size: usize,
}

#[derive(Default, Clone, Copy)]
struct sg_shader_image_t {
    image_type: sg_image_type,
    sampler_type: sg_sampler_type,
}

#[derive(Default, Clone, Copy)]
struct sg_shader_stage_t {
    num_uniform_blocks: i32,
    num_images: i32,
    uniform_blocks: [sg_shader_uniform_block_t; SG_MAX_SHADERSTAGE_UBS as usize],
    images: [sg_shader_image_t; SG_MAX_SHADERSTAGE_IMAGES as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_buffer_layout_desc {
    stride: i32,
    step_func: sg_vertex_step,
    step_rate: i32,
}

#[derive(Default, Clone, Copy)]
struct sg_vertex_attr_desc {
    buffer_index: i32,
    offset: i32,
    format: sg_vertex_format,
}

#[derive(Default, Clone, Copy)]
struct sg_layout_desc {
    buffers: [sg_buffer_layout_desc; SG_MAX_SHADERSTAGE_BUFFERS as usize],
    attrs: [sg_vertex_attr_desc; SG_MAX_VERTEX_ATTRIBUTES as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_pipeline_common_t {
    vertex_layout_valid: [bool; SG_MAX_SHADERSTAGE_BUFFERS as usize],
    use_instanced_draw: bool,
    shader_id: sg_shader,
    layout: sg_layout_desc,
    depth: sg_depth_state,
    stencil: sg_stencil_state,
    color_count: i32,
    colors: [sg_color_state; SG_MAX_COLOR_ATTACHMENTS as usize],
    primitive_type: sg_primitive_type,
    index_type: sg_index_type,
    cull_mode: sg_cull_mode,
    face_winding: sg_face_winding,
    sample_count: i32,
    blend_color: sg_color,
    alpha_to_coverage_enabled: bool,
}

#[derive(Default, Clone, Copy)]
struct sg_shader_common_t {
    stage: [sg_shader_stage_t; SG_NUM_SHADER_STAGES as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_gl_shader_stage_t {
    uniform_blocks: [sg_gl_uniform_block_t; SG_MAX_SHADERSTAGE_UBS as usize],
    images: [sg_gl_shader_image_t; SG_MAX_SHADERSTAGE_IMAGES as usize],
}

#[derive(Default, Clone, Copy)]
struct GLShader_Data {
    prog: u32,
    attrs: [sg_gl_shader_attr_t; SG_MAX_VERTEX_ATTRIBUTES as usize],
    stage: [sg_gl_shader_stage_t; SG_NUM_SHADER_STAGES as usize],
}

#[derive(Default, Clone, Copy)]
struct sg_gl_shader_t {
    slot: sg_slot_t,
    cmn: sg_shader_common_t,
    gl: GLShader_Data,
}
type sg_shader_t = sg_gl_shader_t;

#[derive(Default, Clone, Copy)]
struct GLPipeline_Data {
    attrs: [sg_gl_attr_t; SG_MAX_VERTEX_ATTRIBUTES as usize],
    depth: sg_depth_state,
    stencil: sg_stencil_state,
    primitive_type: sg_primitive_type,
    blend: sg_blend_state,
    color_write_mask: [sg_color_mask; SG_MAX_COLOR_ATTACHMENTS as usize],
    cull_mode: sg_cull_mode,
    face_winding: sg_face_winding,
    sample_count: i32,
    alpha_to_coverage_enabled: bool,
}

#[derive(Default, Clone, Copy)]
struct sg_gl_pipeline_t {
    slot: sg_slot_t,
    cmn: sg_pipeline_common_t,
    //_sg_shader_t* shader;
    gl: GLPipeline_Data,
}
type sg_pipeline_t = sg_gl_pipeline_t;

#[derive(Default, Clone, Copy)]
struct sg_gl_attachment_t {
    //_sg_image_t* image;
    gl_msaa_resolve_buffer: u32,
}

#[derive(Default, Clone, Copy)]
struct sg_pass_common_t {
    num_color_atts: i32,
    color_atts: [sg_pass_attachment_common_t; SG_MAX_COLOR_ATTACHMENTS as usize],
    ds_att: sg_pass_attachment_common_t,
}

#[derive(Default, Clone, Copy)]
struct GLPass_Data {
    fb: u32,
    color_atts: [sg_gl_attachment_t; SG_MAX_COLOR_ATTACHMENTS as usize],
    ds_att: sg_gl_attachment_t,
}

#[derive(Default, Clone, Copy)]
struct sg_gl_pass_t {
    slot: sg_slot_t,
    cmn: sg_pass_common_t,
    gl: GLPass_Data,
}
type sg_pass_t = sg_gl_pass_t;
type sg_pass_attachment_t = sg_pass_attachment_common_t;

#[derive(Default, Clone, Copy)]
struct sg_gl_context_t {
    slot: sg_slot_t,
    //#if !defined(SOKOL_GLES2)
    vao: GLuint,
    //#endif
    default_framebuffer: u32,
}
type sg_context_t = sg_gl_context_t;

const SG_GL_IMAGE_CACHE_SIZE: u32 = SG_MAX_SHADERSTAGE_IMAGES * SG_NUM_SHADER_STAGES;

#[derive(Default)]
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
    cur_primitive_type: GLenum,
    cur_index_type: GLenum,
    cur_active_texture: GLenum,
    //_sg_pipeline_t* cur_pipeline;
    cur_pipeline_id: sg_pipeline, // DT_TODO: Is this duplicated in the main state? sg_pipeline
}

#[derive(Default)]
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
    max_anisotropy: u32,

    opengl32_dll: HINSTANCE,
}

#[derive(Default)]
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

#[derive(Default)]
struct sg_limits {
    max_image_size_2d: u32,      // max width/height of SG_IMAGETYPE_2D images
    max_image_size_cube: u32,    // max width/height of SG_IMAGETYPE_CUBE images
    max_image_size_3d: u32,      // max width/height/depth of SG_IMAGETYPE_3D images
    max_image_size_array: u32,   // max width/height of SG_IMAGETYPE_ARRAY images
    max_image_array_layers: u32, // max number of layers in SG_IMAGETYPE_ARRAY images
    max_vertex_attrs: u32,       // <= SG_MAX_VERTEX_ATTRIBUTES or less (on some GLES2 impls)
    gl_max_vertex_uniform_vectors: u32, // <= GL_MAX_VERTEX_UNIFORM_VECTORS (only on GL backends)
    gl_max_combined_texture_image_units: u32, // <= GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS (only on GL backends)
}

#[derive(Clone, Copy)]
pub struct sg_gl_context_desc {
    pub force_gles2: bool,
}

#[derive(Clone, Copy)]
pub struct sg_context_desc {
    pub color_format: sg_pixel_format,
    pub depth_format: sg_pixel_format,
    pub sample_count: u32,
    pub gl: sg_gl_context_desc,
    //sg_metal_context_desc metal;
    //sg_d3d11_context_desc d3d11;
    //sg_wgpu_context_desc wgpu;
}

#[derive(Clone, Copy)]
pub struct sg_desc {
    pub buffer_pool_size: u32,
    pub image_pool_size: u32,
    pub shader_pool_size: u32,
    pub pipeline_pool_size: u32,
    pub pass_pool_size: u32,
    pub context_pool_size: u32,
    pub uniform_buffer_size: u32,
    pub staging_buffer_size: u32,
    pub sampler_cache_size: u32,
    pub max_commit_listeners: u32,
    pub disable_validation: bool, // disable validation layer even in debug mode, useful for tests
    //sg_allocator allocator;
    //sg_logger logger; // optional log function override
    pub context: sg_context_desc,
}

impl Default for sg_desc {
    fn default() -> Self {
        sg_desc {
            buffer_pool_size: SG_DEFAULT_BUFFER_POOL_SIZE,
            image_pool_size: SG_DEFAULT_IMAGE_POOL_SIZE,
            shader_pool_size: SG_DEFAULT_SHADER_POOL_SIZE,
            pipeline_pool_size: SG_DEFAULT_PIPELINE_POOL_SIZE,
            pass_pool_size: SG_DEFAULT_PASS_POOL_SIZE,
            context_pool_size: SG_DEFAULT_CONTEXT_POOL_SIZE,
            uniform_buffer_size: SG_DEFAULT_UB_SIZE,
            staging_buffer_size: SG_DEFAULT_STAGING_SIZE,
            sampler_cache_size: SG_DEFAULT_SAMPLER_CACHE_CAPACITY,
            max_commit_listeners: SG_DEFAULT_MAX_COMMIT_LISTENERS,
            disable_validation: false,
            context: sg_context_desc {
                color_format: sg_pixel_format::RGBA8, // DT_TODO: See sg_desc_defaults - different targets have different defaults
                depth_format: sg_pixel_format::DEPTH_STENCIL,
                sample_count: 1,
                gl: sg_gl_context_desc { force_gles2: false },
            },
        }
    }
}

pub struct sg_state_t {
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
impl Default for sg_state_t {
    fn default() -> Self {
        sg_state_t {
            valid: false,
            desc: sg_desc::default(),
            frame_index: 0,
            active_context: sg_context::default(),
            cur_pass: sg_pass::default(),
            cur_pipeline: sg_pipeline::default(),
            pass_valid: false,
            bindings_valid: false,
            next_draw_valid: false,
            pools: sg_pools_t::default(),
            backend: sg_backend::default(),
            features: sg_features::default(),
            limits: sg_limits::default(),
            formats: [sg_pixelformat_info::default(); SG_PIXELFORMAT_NUM as usize],
            gl: sg_gl_backend_t::default(),
        }
    }
}

fn sg_init_pool(pool: &mut sg_pool_t, num: u32) {
    debug_assert!(num >= 1);
    // slot 0 is reserved for the 'invalid id', so bump the pool size by 1
    pool.size = num + 1;
    pool.queue_top = 0;
    // generation counters indexable by pool slot index, slot 0 is reserved
    pool.gen_ctrs.resize(pool.size as usize, 0);

    // it's not a bug to only reserve 'num' here
    pool.free_queue.resize(num as usize, 0);

    // DT_TODO: Test this
    // never allocate the zero-th pool item since the invalid id is 0
    for i in (1..pool.size).rev() {
        pool.free_queue[pool.queue_top as usize] = i;
        pool.queue_top += 1;
    }
}

fn sg_setup_pools(p: &mut sg_pools_t, desc: &sg_desc) {
    // note: the pools here will have an additional item, since slot 0 is reserved
    debug_assert!((desc.buffer_pool_size > 0) && (desc.buffer_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.buffer_pool, desc.buffer_pool_size);
    p.buffers
        .resize(p.buffer_pool.size as usize, sg_buffer_t::default());

    debug_assert!((desc.image_pool_size > 0) && (desc.image_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.image_pool, desc.image_pool_size);
    p.images
        .resize(p.image_pool.size as usize, sg_image_t::default());

    debug_assert!((desc.shader_pool_size > 0) && (desc.shader_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.shader_pool, desc.shader_pool_size);
    p.shaders
        .resize(p.shader_pool.size as usize, sg_shader_t::default());

    debug_assert!((desc.pipeline_pool_size > 0) && (desc.pipeline_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.pipeline_pool, desc.pipeline_pool_size);
    p.pipelines
        .resize(p.pipeline_pool.size as usize, sg_pipeline_t::default());

    debug_assert!((desc.pass_pool_size > 0) && (desc.pass_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.pass_pool, desc.pass_pool_size);
    p.passes
        .resize(p.pass_pool.size as usize, sg_pass_t::default());

    debug_assert!((desc.context_pool_size > 0) && (desc.context_pool_size < SG_MAX_POOL_SIZE));
    sg_init_pool(&mut p.context_pool, desc.context_pool_size);
    p.contexts
        .resize(p.context_pool.size as usize, sg_context_t::default());
}

/* capability table pixel format helper functions */
fn sg_pixelformat_all(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.filter = true;
    pfi.blend = true;
    pfi.render = true;
    pfi.msaa = true;
}

fn sg_pixelformat_s(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
}

fn sg_pixelformat_sf(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.filter = true;
}

fn sg_pixelformat_sr(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.render = true;
}

fn sg_pixelformat_srmd(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.render = true;
    pfi.msaa = true;
    pfi.depth = true;
}

fn sg_pixelformat_srm(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.render = true;
    pfi.msaa = true;
}

fn sg_pixelformat_sfrm(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.filter = true;
    pfi.render = true;
    pfi.msaa = true;
}
fn sg_pixelformat_sbrm(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.blend = true;
    pfi.render = true;
    pfi.msaa = true;
}

fn sg_pixelformat_sbr(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.blend = true;
    pfi.render = true;
}

fn sg_pixelformat_sfbr(pfi: &mut sg_pixelformat_info) {
    pfi.sample = true;
    pfi.filter = true;
    pfi.blend = true;
    pfi.render = true;
}

const GL_INT_2_10_10_10_REV: u32 = 0x8D9F;
const GL_PROGRAM_POINT_SIZE: u32 = 0x8642;
const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
const GL_NUM_EXTENSIONS: u32 = 0x821D;
const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
const GL_VERTEX_SHADER: u32 = 0x8B31;
const GL_INCR: u32 = 0x1E02;
const GL_DYNAMIC_DRAW: u32 = 0x88E8;
const GL_STATIC_DRAW: u32 = 0x88E4;
const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
const GL_FUNC_SUBTRACT: u32 = 0x800A;
const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
const GL_CONSTANT_COLOR: u32 = 0x8001;
const GL_DECR_WRAP: u32 = 0x8508;
const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
const GL_SHORT: u32 = 0x1402;
const GL_DEPTH_TEST: u32 = 0x0B71;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
const GL_LINK_STATUS: u32 = 0x8B82;
const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
const GL_RGBA16F: u32 = 0x881A;
const GL_CONSTANT_ALPHA: u32 = 0x8003;
const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
const GL_TEXTURE0: u32 = 0x84C0;
const GL_TEXTURE_MIN_LOD: u32 = 0x813A;
const GL_CLAMP_TO_EDGE: u32 = 0x812F;
const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
const GL_TEXTURE_WRAP_R: u32 = 0x8072;
const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
const GL_STREAM_DRAW: u32 = 0x88E0;
const GL_ONE: u32 = 1;
const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
const GL_RGB10_A2: u32 = 0x8059;
const GL_RGBA8: u32 = 0x8058;
const GL_SRGB8_ALPHA8: u32 = 0x8C43;
const GL_RGBA4: u32 = 0x8056;
const GL_RGB8: u32 = 0x8051;
const GL_ARRAY_BUFFER: u32 = 0x8892;
const GL_STENCIL: u32 = 0x1802;
const GL_TEXTURE_2D: u32 = 0x0DE1;
const GL_DEPTH: u32 = 0x1801;
const GL_FRONT: u32 = 0x0404;
const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
const GL_REPEAT: u32 = 0x2901;
const GL_RGBA: u32 = 0x1908;
const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
const GL_DECR: u32 = 0x1E03;
const GL_FRAGMENT_SHADER: u32 = 0x8B30;
const GL_FLOAT: u32 = 0x1406;
const GL_TEXTURE_MAX_LOD: u32 = 0x813B;
const GL_DEPTH_COMPONENT: u32 = 0x1902;
const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
const GL_COLOR: u32 = 0x1800;
const GL_TEXTURE_2D_ARRAY: u32 = 0x8C1A;
const GL_TRIANGLES: u32 = 0x0004;
const GL_UNSIGNED_BYTE: u32 = 0x1401;
const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
const GL_NONE: u32 = 0;
const GL_SRC_COLOR: u32 = 0x0300;
const GL_BYTE: u32 = 0x1400;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
const GL_LINE_STRIP: u32 = 0x0003;
const GL_TEXTURE_3D: u32 = 0x806F;
const GL_CW: u32 = 0x0900;
const GL_LINEAR: u32 = 0x2601;
const GL_RENDERBUFFER: u32 = 0x8D41;
const GL_GEQUAL: u32 = 0x0206;
const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
const GL_RGBA32F: u32 = 0x8814;
const GL_BLEND: u32 = 0x0BE2;
const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
const GL_TEXTURE_WRAP_T: u32 = 0x2803;
const GL_TEXTURE_WRAP_S: u32 = 0x2802;
const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
const GL_EXTENSIONS: u32 = 0x1F03;
const GL_NO_ERROR: u32 = 0;
const GL_REPLACE: u32 = 0x1E01;
const GL_KEEP: u32 = 0x1E00;
const GL_CCW: u32 = 0x0901;
const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
const GL_RGB: u32 = 0x1907;
const GL_TRIANGLE_STRIP: u32 = 0x0005;
const GL_FALSE: u32 = 0;
const GL_ZERO: u32 = 0;
const GL_CULL_FACE: u32 = 0x0B44;
const GL_INVERT: u32 = 0x150A;
const GL_INT: u32 = 0x1404;
const GL_UNSIGNED_INT: u32 = 0x1405;
const GL_UNSIGNED_SHORT: u32 = 0x1403;
const GL_NEAREST: u32 = 0x2600;
const GL_SCISSOR_TEST: u32 = 0x0C11;
const GL_LEQUAL: u32 = 0x0203;
const GL_STENCIL_TEST: u32 = 0x0B90;
const GL_DITHER: u32 = 0x0BD0;
const GL_DEPTH_COMPONENT16: u32 = 0x81A5;
const GL_EQUAL: u32 = 0x0202;
const GL_FRAMEBUFFER: u32 = 0x8D40;
const GL_RGB5: u32 = 0x8050;
const GL_LINES: u32 = 0x0001;
const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
const GL_SRC_ALPHA: u32 = 0x0302;
const GL_INCR_WRAP: u32 = 0x8507;
const GL_LESS: u32 = 0x0201;
const GL_MULTISAMPLE: u32 = 0x809D;
const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
const GL_BACK: u32 = 0x0405;
const GL_ALWAYS: u32 = 0x0207;
const GL_FUNC_ADD: u32 = 0x8006;
const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
const GL_NOTEQUAL: u32 = 0x0205;
const GL_DST_COLOR: u32 = 0x0306;
const GL_COMPILE_STATUS: u32 = 0x8B81;
const GL_RED: u32 = 0x1903;
const GL_DST_ALPHA: u32 = 0x0304;
const GL_RGB5_A1: u32 = 0x8057;
const GL_GREATER: u32 = 0x0204;
const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
const GL_TRUE: u32 = 1;
const GL_NEVER: u32 = 0x0200;
const GL_POINTS: u32 = 0x0000;
const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
const GL_MIRRORED_REPEAT: u32 = 0x8370;
const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
const GL_R11F_G11F_B10F: u32 = 0x8C3A;
const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
const GL_RGB9_E5: u32 = 0x8C3D;
const GL_UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;
const GL_RGBA32UI: u32 = 0x8D70;
const GL_RGB32UI: u32 = 0x8D71;
const GL_RGBA16UI: u32 = 0x8D76;
const GL_RGB16UI: u32 = 0x8D77;
const GL_RGBA8UI: u32 = 0x8D7C;
const GL_RGB8UI: u32 = 0x8D7D;
const GL_RGBA32I: u32 = 0x8D82;
const GL_RGB32I: u32 = 0x8D83;
const GL_RGBA16I: u32 = 0x8D88;
const GL_RGB16I: u32 = 0x8D89;
const GL_RGBA8I: u32 = 0x8D8E;
const GL_RGB8I: u32 = 0x8D8F;
const GL_RED_INTEGER: u32 = 0x8D94;
const GL_RG: u32 = 0x8227;
const GL_RG_INTEGER: u32 = 0x8228;
const GL_R8: u32 = 0x8229;
const GL_R16: u32 = 0x822A;
const GL_RG8: u32 = 0x822B;
const GL_RG16: u32 = 0x822C;
const GL_R16F: u32 = 0x822D;
const GL_R32F: u32 = 0x822E;
const GL_RG16F: u32 = 0x822F;
const GL_RG32F: u32 = 0x8230;
const GL_R8I: u32 = 0x8231;
const GL_R8UI: u32 = 0x8232;
const GL_R16I: u32 = 0x8233;
const GL_R16UI: u32 = 0x8234;
const GL_R32I: u32 = 0x8235;
const GL_R32UI: u32 = 0x8236;
const GL_RG8I: u32 = 0x8237;
const GL_RG8UI: u32 = 0x8238;
const GL_RG16I: u32 = 0x8239;
const GL_RG16UI: u32 = 0x823A;
const GL_RG32I: u32 = 0x823B;
const GL_RG32UI: u32 = 0x823C;
const GL_RGBA_INTEGER: u32 = 0x8D99;
const GL_R8_SNORM: u32 = 0x8F94;
const GL_RG8_SNORM: u32 = 0x8F95;
const GL_RGB8_SNORM: u32 = 0x8F96;
const GL_RGBA8_SNORM: u32 = 0x8F97;
const GL_R16_SNORM: u32 = 0x8F98;
const GL_RG16_SNORM: u32 = 0x8F99;
const GL_RGB16_SNORM: u32 = 0x8F9A;
const GL_RGBA16_SNORM: u32 = 0x8F9B;
const GL_RGBA16: u32 = 0x805B;
const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
const GL_CLAMP_TO_BORDER: u32 = 0x812D;
const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
const GL_CURRENT_PROGRAM: u32 = 0x8B8D;
const GL_MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;
const GL_UNPACK_ALIGNMENT: u32 = 0x0CF5;
const GL_FRAMEBUFFER_SRGB: u32 = 0x8DB9;
const GL_UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
const GL_UNSIGNED_INT_24_8: u32 = 0x84FA;
const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;
const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 0x83F1;
const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 0x83F2;
const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 0x83F3;
const GL_COMPRESSED_RED_RGTC1: u32 = 0x8DBB;
const GL_COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8DBC;
const GL_COMPRESSED_RED_GREEN_RGTC2: u32 = 0x8DBD;
const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2: u32 = 0x8DBE;
const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB: u32 = 0x8E8C;
const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: u32 = 0x8E8D;
const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: u32 = 0x8E8E;
const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: u32 = 0x8E8F;
const GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: u32 = 0x8C01;
const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: u32 = 0x8C00;
const GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: u32 = 0x8C03;
const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 0x8C02;
const GL_COMPRESSED_RGB8_ETC2: u32 = 0x9274;
const GL_COMPRESSED_RGBA8_ETC2_EAC: u32 = 0x9278;
const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9276;
const GL_COMPRESSED_RG11_EAC: u32 = 0x9272;
const GL_COMPRESSED_SIGNED_RG11_EAC: u32 = 0x9273;
const GL_DEPTH24_STENCIL8: u32 = 0x88F0;
const GL_HALF_FLOAT: u32 = 0x140B;
const GL_DEPTH_STENCIL: u32 = 0x84F9;
const GL_LUMINANCE: u32 = 0x1909;

type GLbyte = i8;
type GLubyte = u8;
type GLboolean = u8;
type GLchar = u8;

type GLshort = i16;
type GLushort = u16;
type GLhalf = u16;

type GLint = i32;
type GLenum = u32;
type GLuint = u32;
type GLbitfield = u32;

type GLsizei = i32;
type GLintptr = isize;
type GLsizeiptr = usize;

type GLint64 = i64;
type GLuint64 = u64;

type GLclampf = f32;
type GLfloat = f32;

type GLclampd = f64;
type GLdouble = f64;

// helper function to lookup GL functions in GL DLL
type sg_wglGetProcAddressT = extern "system" fn(name: *const u8) -> PROC;

unsafe fn sg_gl_getprocaddr(
    sg: &sg_state_t,
    name: *const u8,
    wgl_getprocaddress: sg_wglGetProcAddressT,
) -> PROC {
    let mut proc_addr = wgl_getprocaddress(name);
    if proc_addr.is_none() {
        proc_addr = GetProcAddress(sg.gl.opengl32_dll, name);
    }
    proc_addr
}

macro_rules! generate_gl_types {
    ( $( $name:ident, $ret:ty, $retval:expr, ( $( $param:ident : $param_type:ty ),* ); )* ) => {
        mod GLDUMMY {
            use super::*;
            $(
                pub unsafe extern "system" fn $name( $( $param : $param_type ),* ) -> $ret { $retval }
            )*
        }

        $(
            static mut $name : unsafe extern "system" fn( $( $param : $param_type ),* ) -> $ret = GLDUMMY::$name;
        )*

        unsafe fn sg_gl_load_funcs(sg : &sg_state_t, wgl_getprocaddress : sg_wglGetProcAddressT){
            $(
                let loader = std::mem::transmute(sg_gl_getprocaddr(sg, concat!(stringify!($name), "\0").as_ptr(), wgl_getprocaddress));
                if let Some(val) = loader {
                    $name = val;
                }
            )*
        }

    };
}

generate_gl_types!(
    glBindVertexArray, (), (), (array: GLuint);
    glFramebufferTextureLayer, (), (), (target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
    glGenFramebuffers, (), (), (n : GLsizei, framebuffers: *const GLuint);
    glBindFramebuffer, (), (), (target : GLenum , framebuffer: GLuint );
    glBindRenderbuffer, (), (), (target : GLenum, renderbuffer : GLuint);
    glGetStringi, *const GLubyte, std::ptr::null(), (name : GLenum, index : GLuint);
    glClearBufferfi, (), (), (buffer : GLenum, drawbuffer: GLint, depth : GLfloat, stencil : GLint);
    glClearBufferfv, (), (), (buffer : GLenum, drawbuffer: GLint, value : *const GLfloat);
    glClearBufferuiv, (), (), (buffer : GLenum, drawbuffer: GLint, value : *const GLuint);
    glClearBufferiv,  (), (), (buffer : GLenum, drawbuffer: GLint, value : *const GLint);
    glDeleteRenderbuffers, (), (), (n : GLsizei, renderbuffers : *const GLuint);
    glUniform1fv, (), (), (location : GLint, count : GLsizei, value : *const GLfloat);
    glUniform2fv, (), (), (location : GLint, count : GLsizei, value : *const GLfloat);
    glUniform3fv, (), (), (location : GLint, count : GLsizei, value : *const GLfloat);
    glUniform4fv, (), (), (location : GLint, count : GLsizei, value : *const GLfloat);
    glUniform1iv, (), (), (location : GLint, count : GLsizei, value : *const GLint);
    glUniform2iv, (), (), (location : GLint, count : GLsizei, value : *const GLint);
    glUniform3iv, (), (), (location : GLint, count : GLsizei, value : *const GLint);
    glUniform4iv, (), (), (location : GLint, count : GLsizei, value : *const GLint);
    glUniformMatrix4fv,                (), (), (location : GLint, count : GLsizei, transpose : GLboolean, value : *const GLfloat);
    glUseProgram,                      (), (), (program : GLuint);
    glShaderSource,                    (), (), (shader : GLuint, count : GLsizei, string : *const *const GLchar, length : *const GLint);
    glLinkProgram,                     (), (), (program : GLuint);
    glGetUniformLocation,              GLint,0, (program : GLuint, name : *const GLchar);
    glGetShaderiv,                     (), (), (shader : GLuint, pname : GLenum, params : *mut GLint);
    glGetProgramInfoLog,               (), (), (program : GLuint, bufSize : GLsizei, length : *mut GLsizei, infoLog : *mut GLchar);
    glGetAttribLocation,               GLint,0, (program : GLuint, name : *const GLchar);
    glDisableVertexAttribArray,        (), (), (index : GLuint);
    glDeleteShader,                    (), (), (shader : GLuint);
    glDeleteProgram,                   (), (), (program : GLuint);
    glCompileShader,                   (), (), (shader : GLuint);
    glStencilFuncSeparate,             (), (), (face : GLenum, func : GLenum, refval : GLint, mask : GLuint);
    glStencilOpSeparate,               (), (), (face : GLenum, sfail : GLenum, dpfail : GLenum, dppass : GLenum);
    glRenderbufferStorageMultisample,  (), (), (target : GLenum, samples : GLsizei, internalformat : GLenum, width : GLsizei, height : GLsizei);
    glDrawBuffers,                     (), (), (n : GLsizei, bufs : *const GLenum);
    glVertexAttribDivisor,             (), (), (index : GLuint, divisor : GLuint);
    glBufferSubData,                   (), (), (target : GLenum, offset : GLintptr, size : GLsizeiptr, data : *const ::core::ffi::c_void);
    glGenBuffers,                      (), (), (n : GLsizei, buffers : *mut GLuint);
    glCheckFramebufferStatus,          GLenum,0, (target : GLenum);
    glFramebufferRenderbuffer,         (), (), (target : GLenum, attachment : GLenum, renderbuffertarget : GLenum, renderbuffer : GLuint);
    glCompressedTexImage2D,            (), (), (target : GLenum, level : GLint, internalformat : GLenum, width : GLsizei, height : GLsizei, border : GLint, imageSize : GLsizei, data : *const ::core::ffi::c_void);
    glCompressedTexImage3D,            (), (), (target : GLenum, level : GLint, internalformat : GLenum, width : GLsizei, height : GLsizei, depth : GLsizei, border : GLint, imageSize : GLsizei, data : *const ::core::ffi::c_void);
    glActiveTexture,                   (), (), (texture : GLenum);
    glTexSubImage3D,                   (), (), (target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, zoffset : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, format : GLenum, typeval : GLenum, pixels : *const ::core::ffi::c_void);
    glRenderbufferStorage,             (), (), (target : GLenum, internalformat : GLenum, width : GLsizei, height : GLsizei);
    glGenTextures,                     (), (), (n : GLsizei, textures : *mut GLuint);
    glPolygonOffset,                   (), (), (factor : GLfloat, units : GLfloat);
    glDrawElements,                    (), (), (mode : GLenum, count : GLsizei, typeval : GLenum, indices : *const ::core::ffi::c_void);
    glDeleteFramebuffers,              (), (), (n : GLsizei, framebuffers : *const GLuint);
    glBlendEquationSeparate,           (), (), (modeRGB : GLenum, modeAlpha : GLenum);
    glDeleteTextures,                  (), (), (n : GLsizei, textures : *const GLuint);
    glGetProgramiv,                    (), (), (program : GLuint, pname : GLenum, params : *mut GLint);
    glBindTexture,                     (), (), (target : GLenum, texture : GLuint);
    glTexImage3D,                      (), (), (target : GLenum, level : GLint, internalformat : GLint, width : GLsizei, height : GLsizei, depth : GLsizei, border : GLint, format : GLenum, typeval : GLenum, pixels : *const ::core::ffi::c_void);
    glCreateShader,                    GLuint,0, (typeval : GLenum);
    glTexSubImage2D,                   (), (), (target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, width : GLsizei, height : GLsizei, format : GLenum, typeval : GLenum, pixels : *const ::core::ffi::c_void);
    glClearDepth,                      (), (), (depth : GLdouble);
    glFramebufferTexture2D,            (), (), (target : GLenum, attachment : GLenum, textarget : GLenum, texture : GLuint, level : GLint);
    glCreateProgram,                   GLuint,0, ();
    glViewport,                        (), (), (x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    glDeleteBuffers,                   (), (), (n : GLsizei, buffers : *const GLuint);
    glDrawArrays,                      (), (), (mode : GLenum, first : GLint, count : GLsizei);
    glDrawElementsInstanced,           (), (), (mode : GLenum, count : GLsizei, typeval : GLenum, indices : *const ::core::ffi::c_void, instancecount : GLsizei);
    glVertexAttribPointer,             (), (), (index : GLuint, size : GLint, typeval : GLenum, normalized : GLboolean, stride : GLsizei, pointer : *const ::core::ffi::c_void);
    glUniform1i,                       (), (), (location : GLint, v0 : GLint);
    glDisable,                         (), (), (cap : GLenum);
    glColorMask,                       (), (), (red : GLboolean, green : GLboolean, blue : GLboolean, alpha : GLboolean);
    glColorMaski,                      (), (), (buf : GLuint, red : GLboolean, green : GLboolean, blue : GLboolean, alpha : GLboolean);
    glBindBuffer,                      (), (), (target : GLenum, buffer : GLuint);
    glDeleteVertexArrays,              (), (), (n : GLsizei, arrays : *const GLuint);
    glDepthMask,                       (), (), (flag : GLboolean);
    glDrawArraysInstanced,             (), (), (mode : GLenum, first : GLint, count : GLsizei, instancecount : GLsizei);
    glClearStencil,                    (), (), (s : GLint);
    glScissor,                         (), (), (x : GLint, y : GLint, width : GLsizei, height : GLsizei);
    glGenRenderbuffers,                (), (), (n : GLsizei, renderbuffers : *mut GLuint);
    glBufferData,                      (), (), (target : GLenum, size : GLsizeiptr, data : *const ::core::ffi::c_void, usage : GLenum);
    glBlendFuncSeparate,               (), (), (sfactorRGB : GLenum, dfactorRGB : GLenum, sfactorAlpha : GLenum, dfactorAlpha : GLenum);
    glTexParameteri,                   (), (), (target : GLenum, pname : GLenum, param : GLint);
    glGetIntegerv,                     (), (), (pname : GLenum, data : *mut GLint);
    glEnable,                          (), (), (cap : GLenum);
    glBlitFramebuffer,                 (), (), (srcX0 : GLint, srcY0 : GLint, srcX1 : GLint, srcY1 : GLint, dstX0 : GLint, dstY0 : GLint, dstX1 : GLint, dstY1 : GLint, mask : GLbitfield, filter : GLenum);
    glStencilMask,                     (), (), (mask : GLuint);
    glAttachShader,                    (), (), (program : GLuint, shader : GLuint);
    glGetError,                        GLenum,0, ();
    glClearColor,                      (), (), (red : GLfloat, green : GLfloat, blue : GLfloat, alpha : GLfloat);
    glBlendColor,                      (), (), (red : GLfloat, green : GLfloat, blue : GLfloat, alpha : GLfloat);
    glTexParameterf,                   (), (), (target : GLenum, pname : GLenum, param : GLfloat);
    glTexParameterfv,                  (), (), (target : GLenum, pname : GLenum, params : *mut GLfloat);
    glGetShaderInfoLog,                (), (), (shader : GLuint, bufSize : GLsizei, length : *mut GLsizei, infoLog : *mut GLchar);
    glDepthFunc,                       (), (), (func : GLenum);
    glStencilOp ,                      (), (), (fail : GLenum, zfail : GLenum, zpass : GLenum);
    glStencilFunc,                     (), (), (func : GLenum, refval : GLint, mask : GLuint);
    glEnableVertexAttribArray,         (), (), (index : GLuint);
    glBlendFunc,                       (), (), (sfactor : GLenum, dfactor : GLenum);
    glReadBuffer,                      (), (), (src : GLenum);
    glReadPixels,                      (), (), (x : GLint, y : GLint, width : GLsizei, height : GLsizei, format : GLenum, typeval : GLenum, data : *mut ::core::ffi::c_void);
    glClear,                           (), (), (mask : GLbitfield);
    glTexImage2D,                      (), (), (target : GLenum, level : GLint, internalformat : GLint, width : GLsizei, height : GLsizei, border : GLint, format : GLenum, typeval : GLenum, pixels : *const ::core::ffi::c_void);
    glGenVertexArrays,                 (), (), (n : GLsizei, arrays : *mut GLuint);
    glFrontFace,                       (), (), (mode : GLenum);
    glCullFace,                        (), (), (mode : GLenum);
    glPixelStorei,                     (), (), (pname : GLenum, param : GLint);

);

fn sg_gl_init_pixelformats(sg: &mut sg_state_t, has_bgra: bool) {
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::R8 as usize]);
    } else {
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R8 as usize]);
    }
    //#else
    //sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R8]);
    //#endif
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R8SN as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::R8UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::R8SI as usize]);
        //#if !defined(SOKOL_GLES3)
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::R16 as usize]);
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::R16SN as usize]);
        //#endif
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::R16UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::R16SI as usize]);
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RG8 as usize]);
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RG8SN as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG8UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG8SI as usize]);
        sg_pixelformat_sr(&mut sg.formats[sg_pixel_format::R32UI as usize]);
        sg_pixelformat_sr(&mut sg.formats[sg_pixel_format::R32SI as usize]);
        //#if !defined(SOKOL_GLES3)
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RG16 as usize]);
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RG16SN as usize]);
        //#endif
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG16UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG16SI as usize]);
    }
    //#endif
    sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA8 as usize]);
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::SRGB8A8 as usize]);
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGBA8SN as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA8UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA8SI as usize]);
    }
    //#endif
    if has_bgra {
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::BGRA8 as usize]);
    }
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGB10A2 as usize]);
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RG11B10F as usize]);
        sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGB9E5 as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG32UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RG32SI as usize]);
        //#if !defined(SOKOL_GLES3)
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA16 as usize]);
        sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA16SN as usize]);
        //#endif
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA16UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA16SI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA32UI as usize]);
        sg_pixelformat_srm(&mut sg.formats[sg_pixel_format::RGBA32SI as usize]);
    }
    //#endif
    // FIXME: WEBGL_depth_texture extension?
    sg_pixelformat_srmd(&mut sg.formats[sg_pixel_format::DEPTH as usize]);
    sg_pixelformat_srmd(&mut sg.formats[sg_pixel_format::DEPTH_STENCIL as usize]);
}

/* FIXME: OES_half_float_blend */
fn sg_gl_init_pixelformats_half_float(
    sg: &mut sg_state_t,
    has_colorbuffer_half_float: bool,
    has_texture_half_float_linear: bool,
) {
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        if has_texture_half_float_linear {
            if has_colorbuffer_half_float {
                sg_pixelformat_all(&mut sg.formats[sg_pixel_format::R16F as usize]);
                sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RG16F as usize]);
                sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            } else {
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R16F as usize]);
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RG16F as usize]);
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            }
        } else {
            if has_colorbuffer_half_float {
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::R16F as usize]);
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RG16F as usize]);
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            } else {
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::R16F as usize]);
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RG16F as usize]);
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            }
        }
    } else {
        //#endif
        /* GLES2 can only render to RGBA, and there's no RG format */
        if has_texture_half_float_linear {
            if has_colorbuffer_half_float {
                sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            } else {
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            }
            sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R16F as usize]);
        } else {
            if has_colorbuffer_half_float {
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            } else {
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RGBA16F as usize]);
            }
            sg_pixelformat_s(&mut sg.formats[sg_pixel_format::R16F as usize]);
        }
        //#if !defined(SOKOL_GLES2)
    }
    //#endif
}

fn sg_gl_init_pixelformats_float(
    sg: &mut sg_state_t,
    has_colorbuffer_float: bool,
    has_texture_float_linear: bool,
    has_float_blend: bool,
) {
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        if has_texture_float_linear {
            if has_colorbuffer_float {
                if has_float_blend {
                    sg_pixelformat_all(&mut sg.formats[sg_pixel_format::R32F as usize]);
                    sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RG32F as usize]);
                    sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
                } else {
                    sg_pixelformat_sfrm(&mut sg.formats[sg_pixel_format::R32F as usize]);
                    sg_pixelformat_sfrm(&mut sg.formats[sg_pixel_format::RG32F as usize]);
                    sg_pixelformat_sfrm(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
                }
            } else {
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R32F as usize]);
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RG32F as usize]);
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            }
        } else {
            if has_colorbuffer_float {
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::R32F as usize]);
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RG32F as usize]);
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            } else {
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::R32F as usize]);
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RG32F as usize]);
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            }
        }
    } else {
        //#endif
        /* GLES2 can only render to RGBA, and there's no RG format */
        if has_texture_float_linear {
            if has_colorbuffer_float {
                if has_float_blend {
                    sg_pixelformat_all(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
                } else {
                    sg_pixelformat_sfrm(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
                }
            } else {
                sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            }
            sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::R32F as usize]);
        } else {
            if has_colorbuffer_float {
                sg_pixelformat_sbrm(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            } else {
                sg_pixelformat_s(&mut sg.formats[sg_pixel_format::RGBA32F as usize]);
            }
            sg_pixelformat_s(&mut sg.formats[sg_pixel_format::R32F as usize]);
        }
        //#if !defined(SOKOL_GLES2)
    }
    //#endif
}

fn sg_gl_init_pixelformats_s3tc(sg: &mut sg_state_t) {
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC1_RGBA as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC2_RGBA as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC3_RGBA as usize]);
}

fn sg_gl_init_pixelformats_rgtc(sg: &mut sg_state_t) {
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC4_R as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC4_RSN as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC5_RG as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC5_RGSN as usize]);
}

fn sg_gl_init_pixelformats_bptc(sg: &mut sg_state_t) {
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC6H_RGBF as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC6H_RGBUF as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::BC7_RGBA as usize]);
}

fn sg_gl_init_pixelformats_pvrtc(sg: &mut sg_state_t) {
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::PVRTC_RGB_2BPP as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::PVRTC_RGB_4BPP as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::PVRTC_RGBA_2BPP as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::PVRTC_RGBA_4BPP as usize]);
}

fn sg_gl_init_pixelformats_etc2(sg: &mut sg_state_t) {
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::ETC2_RGB8 as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::ETC2_RGB8A1 as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::ETC2_RGBA8 as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::ETC2_RG11 as usize]);
    sg_pixelformat_sf(&mut sg.formats[sg_pixel_format::ETC2_RG11SN as usize]);
}

unsafe fn sg_gl_init_limits(sg: &mut sg_state_t) {
    //_SG_GL_CHECK_ERROR(); // DT_TODO:
    let mut gl_int: GLint = 0;
    glGetIntegerv(GL_MAX_TEXTURE_SIZE, &mut gl_int);
    //_SG_GL_CHECK_ERROR();
    sg.limits.max_image_size_2d = gl_int as u32;
    sg.limits.max_image_size_array = gl_int as u32;
    glGetIntegerv(GL_MAX_CUBE_MAP_TEXTURE_SIZE, &mut gl_int);
    //_SG_GL_CHECK_ERROR();
    sg.limits.max_image_size_cube = gl_int as u32;
    glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut gl_int);
    //_SG_GL_CHECK_ERROR();
    if gl_int > SG_MAX_VERTEX_ATTRIBUTES as i32 {
        gl_int = SG_MAX_VERTEX_ATTRIBUTES as i32;
    }
    sg.limits.max_vertex_attrs = gl_int as u32;
    glGetIntegerv(GL_MAX_VERTEX_UNIFORM_VECTORS, &mut gl_int);
    //_SG_GL_CHECK_ERROR();
    sg.limits.gl_max_vertex_uniform_vectors = gl_int as u32;
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        glGetIntegerv(GL_MAX_3D_TEXTURE_SIZE, &mut gl_int);
        //_SG_GL_CHECK_ERROR();
        sg.limits.max_image_size_3d = gl_int as u32;
        glGetIntegerv(GL_MAX_ARRAY_TEXTURE_LAYERS, &mut gl_int);
        //_SG_GL_CHECK_ERROR();
        sg.limits.max_image_array_layers = gl_int as u32;
    }
    //#endif
    if sg.gl.ext_anisotropic {
        glGetIntegerv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut gl_int);
        //_SG_GL_CHECK_ERROR();
        sg.gl.max_anisotropy = gl_int as u32;
    } else {
        sg.gl.max_anisotropy = 1;
    }
    glGetIntegerv(GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut gl_int);
    //_SG_GL_CHECK_ERROR();
    sg.limits.gl_max_combined_texture_image_units = gl_int as u32;
}

unsafe fn sg_gl_init_caps_glcore33(sg: &mut sg_state_t) {
    sg.backend = sg_backend::GLCORE33;

    sg.features.origin_top_left = false;
    sg.features.instancing = true;
    sg.features.multiple_render_targets = true;
    sg.features.msaa_render_targets = true;
    sg.features.imagetype_3d = true;
    sg.features.imagetype_array = true;
    sg.features.image_clamp_to_border = true;
    sg.features.mrt_independent_blend_state = false;
    sg.features.mrt_independent_write_mask = true;

    /* scan extensions */
    let mut has_s3tc = false; /* BC1..BC3 */
    let mut has_rgtc = false; /* BC4 and BC5 */
    let mut has_bptc = false; /* BC6H and BC7 */
    let mut has_pvrtc = false;
    let mut has_etc2 = false;
    let mut num_ext = 0;

    glGetIntegerv(GL_NUM_EXTENSIONS, &mut num_ext);
    for i in 0..num_ext {
        let ext = glGetStringi(GL_EXTENSIONS, i as GLuint);

        if ext != std::ptr::null() {
            let ext = std::ffi::CStr::from_ptr(ext as *const i8).to_bytes();

            // Test that the extension ends with the passed value
            fn test_extension(ext: &[u8], cmp: &[u8]) -> bool {
                if ext.len() < cmp.len() {
                    return false;
                }
                ext[(ext.len() - cmp.len())..] == *cmp
            }

            if !has_s3tc && test_extension(ext, "_texture_compression_s3tc".as_bytes()) {
                has_s3tc = true;
            } else if !has_rgtc && test_extension(ext, "_texture_compression_rgtc".as_bytes()) {
                has_rgtc = true;
            } else if !has_bptc && test_extension(ext, "_texture_compression_bptc".as_bytes()) {
                has_bptc = true;
            } else if !has_pvrtc && test_extension(ext, "_texture_compression_pvrtc".as_bytes()) {
                has_pvrtc = true;
            } else if !has_etc2 && test_extension(ext, "_ES3_compatibility".as_bytes()) {
                has_etc2 = true;
            } else if !sg.gl.ext_anisotropic
                && test_extension(ext, "_texture_filter_anisotropic".as_bytes())
            {
                sg.gl.ext_anisotropic = true;
            }
        }
    }

    /* limits */
    sg_gl_init_limits(sg);

    /* pixel formats */
    let has_bgra = false; /* not a bug */
    let has_colorbuffer_float = true;
    let has_colorbuffer_half_float = true;
    let has_texture_float_linear = true; /* FIXME??? */
    let has_texture_half_float_linear = true;
    let has_float_blend = true;
    sg_gl_init_pixelformats(sg, has_bgra);
    sg_gl_init_pixelformats_float(
        sg,
        has_colorbuffer_float,
        has_texture_float_linear,
        has_float_blend,
    );
    sg_gl_init_pixelformats_half_float(
        sg,
        has_colorbuffer_half_float,
        has_texture_half_float_linear,
    );
    if has_s3tc {
        sg_gl_init_pixelformats_s3tc(sg);
    }
    if has_rgtc {
        sg_gl_init_pixelformats_rgtc(sg);
    }
    if has_bptc {
        sg_gl_init_pixelformats_bptc(sg);
    }
    if has_pvrtc {
        sg_gl_init_pixelformats_pvrtc(sg);
    }
    if has_etc2 {
        sg_gl_init_pixelformats_etc2(sg);
    }
}

fn sg_gl_load_opengl(sg: &mut sg_state_t) {
    debug_assert!(0 == sg.gl.opengl32_dll);
    sg.gl.opengl32_dll = unsafe { LoadLibraryA(s!("opengl32.dll")) };
    debug_assert!(sg.gl.opengl32_dll != 0);

    unsafe {
        let wgl_getprocaddress_fn: Option<sg_wglGetProcAddressT> =
            std::mem::transmute(GetProcAddress(sg.gl.opengl32_dll, s!("wglGetProcAddress")));
        if let Some(wgl_getprocaddress) = wgl_getprocaddress_fn {
            sg_gl_load_funcs(sg, wgl_getprocaddress);
        }
    }
}

fn sg_gl_unload_opengl(sg: &mut sg_state_t) {
    debug_assert!(sg.gl.opengl32_dll != 0);
    unsafe {
        FreeLibrary(sg.gl.opengl32_dll);
    }
    sg.gl.opengl32_dll = 0;
}

fn sg_gl_setup_backend(sg: &mut sg_state_t) {
    /* assumes that _sg.gl is already zero-initialized */
    sg.gl.valid = true;
    //#if defined(SOKOL_GLES2) || defined(SOKOL_GLES3)
    //sg.gl.gles2 = sg.desc.context.gl.force_gles2;
    //#else
    //_SOKOL_UNUSED(desc);
    sg.gl.gles2 = false;
    //#endif

    //#if defined(_SOKOL_USE_WIN32_GL_LOADER)
    sg_gl_load_opengl(sg);
    //#endif

    /* clear initial GL error state */
 // DT_TODO:
 //#if defined(SOKOL_DEBUG)
 //    while (glGetError() != GL_NO_ERROR);
 //#endif
 //#if defined(SOKOL_GLCORE33)
    unsafe {
        sg_gl_init_caps_glcore33(sg);
    }
    //#elif defined(SOKOL_GLES3)
    //    if (_sg.gl.gles2) {
    //        _sg_gl_init_caps_gles2();
    //    }
    //    else {
    //        _sg_gl_init_caps_gles3();
    //    }
    //#else
    //    _sg_gl_init_caps_gles2();
    //#endif
}

fn sg_gl_discard_backend(sg: &mut sg_state_t) {
    debug_assert!(sg.gl.valid);
    sg.gl.valid = false;
    //#if defined(_SOKOL_USE_WIN32_GL_LOADER)
    sg_gl_unload_opengl(sg);
    //#endif
}

fn sg_setup_backend(sg: &mut sg_state_t) {
    sg_gl_setup_backend(sg);
}

fn sg_discard_backend(sg: &mut sg_state_t) {
    sg_gl_discard_backend(sg);
}

fn sg_pool_alloc_index(pool: &mut sg_pool_t) -> u32 {
    if pool.queue_top > 0 {
        pool.queue_top -= 1;
        let slot_index = pool.free_queue[pool.queue_top as usize];
        debug_assert!((slot_index > 0) && (slot_index < pool.size));
        return slot_index;
    } else {
        /* pool exhausted */
        return SG_INVALID_SLOT_INDEX;
    }
}

/* allocate the slot at slot_index:
    - bump the slot's generation counter
    - create a resource id from the generation counter and slot index
    - set the slot's id to this id
    - set the slot's state to ALLOC
    - return the resource id
*/
fn sg_slot_alloc(pool: &mut sg_pool_t, slot: &mut sg_slot_t, slot_index: u32) -> u32 {
    /* FIXME: add handling for an overflowing generation counter,
       for now, just overflow (another option is to disable
       the slot)
    */
    debug_assert!((slot_index > SG_INVALID_SLOT_INDEX) && (slot_index < pool.size));
    debug_assert!((slot.state == sg_resource_state::INITIAL) && (slot.id == SG_INVALID_ID));

    let ctr = &mut pool.gen_ctrs[slot_index as usize];
    *ctr += 1;
    slot.id = (*ctr << SG_SLOT_SHIFT) | (slot_index & SG_SLOT_MASK);
    slot.state = sg_resource_state::ALLOC;
    slot.id
}

/* extract slot index from id */
fn sg_slot_index(id: u32) -> u32 {
    let slot_index = id & SG_SLOT_MASK;
    debug_assert!(SG_INVALID_SLOT_INDEX != slot_index);
    slot_index
}

fn sg_context_at(p: &mut sg_pools_t, context_id: u32) -> &mut sg_context_t {
    debug_assert!(SG_INVALID_ID != context_id);
    let slot_index = sg_slot_index(context_id);
    debug_assert!((slot_index > SG_INVALID_SLOT_INDEX) && (slot_index < p.context_pool.size));
    return &mut p.contexts[slot_index as usize];
}

fn sg_lookup_context(p: &mut sg_pools_t, ctx_id: u32) -> Option<&mut sg_context_t> {
    if SG_INVALID_ID != ctx_id {
        let ctx = sg_context_at(p, ctx_id);
        if ctx.slot.id == ctx_id {
            return Some(ctx);
        }
    }
    None
}

/*-- GL backend resource creation and destruction ----------------------------*/
fn sg_gl_create_context(sg: &mut sg_state_t, ctx_id: sg_context) {
    let ctx: &mut sg_gl_context_t = sg_context_at(&mut sg.pools, ctx_id.id);
    unsafe {
        debug_assert!(0 == ctx.default_framebuffer);
        //_SG_GL_CHECK_ERROR();
        let mut get_int = 0;
        glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut get_int);
        ctx.default_framebuffer = get_int as u32;
        //_SG_GL_CHECK_ERROR();
        //#if !defined(SOKOL_GLES2)
        if !sg.gl.gles2 {
            debug_assert!(0 == ctx.vao);
            glGenVertexArrays(1, &mut ctx.vao);
            glBindVertexArray(ctx.vao);
            //_SG_GL_CHECK_ERROR();
        }
        //#endif
        // incoming texture data is generally expected to be packed tightly
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    }
    ctx.slot.state = sg_resource_state::VALID;
}

fn sg_gl_discard_context(sg: &mut sg_state_t, ctx_id: sg_context) {
    let ctx: &mut sg_gl_context_t = sg_context_at(&mut sg.pools, ctx_id.id);
    //#if !defined(SOKOL_GLES2)
    if !sg.gl.gles2 {
        if ctx.vao != 0 {
            unsafe {
                glDeleteVertexArrays(1, &ctx.vao);
            }
        }
        //_SG_GL_CHECK_ERROR();
    }
    //#else
    //_SOKOL_UNUSED(ctx);
    //#endif
}

fn sg_create_context(sg: &mut sg_state_t, ctx_id: sg_context) {
    return sg_gl_create_context(sg, ctx_id);
}

unsafe fn sg_gl_cache_clear_buffer_bindings(sg: &mut sg_state_t, force: bool) {
    if force || (sg.gl.cache.vertex_buffer != 0) {
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        sg.gl.cache.vertex_buffer = 0;
    }
    if force || (sg.gl.cache.index_buffer != 0) {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        sg.gl.cache.index_buffer = 0;
    }
}

unsafe fn sg_gl_cache_clear_texture_bindings(sg: &mut sg_state_t, force: bool) {
    let mut max_images = sg.limits.gl_max_combined_texture_image_units;
    if max_images > SG_GL_IMAGE_CACHE_SIZE {
        max_images = SG_GL_IMAGE_CACHE_SIZE;
    }
    for i in 0..max_images {
        if force || (sg.gl.cache.textures[i as usize].texture != 0) {
            let gl_texture_slot = (GL_TEXTURE0 + i) as GLenum;
            glActiveTexture(gl_texture_slot);
            glBindTexture(GL_TEXTURE_2D, 0);
            glBindTexture(GL_TEXTURE_CUBE_MAP, 0);
            //#if !defined(SOKOL_GLES2)
            if !sg.gl.gles2 {
                glBindTexture(GL_TEXTURE_3D, 0);
                glBindTexture(GL_TEXTURE_2D_ARRAY, 0);
            }
            //#endif
            sg.gl.cache.textures[i as usize].target = 0;
            sg.gl.cache.textures[i as usize].texture = 0;
            sg.gl.cache.cur_active_texture = gl_texture_slot;
        }
    }
}

unsafe fn sg_gl_reset_state_cache(sg: &mut sg_state_t, ctx_id: sg_context) {
    if ctx_id.id != SG_INVALID_ID {
        let ctx = sg_context_at(&mut sg.pools, ctx_id.id);
        //_SG_GL_CHECK_ERROR();
        //#if !defined(SOKOL_GLES2)
        if !sg.gl.gles2 {
            glBindVertexArray(ctx.vao);
            //_SG_GL_CHECK_ERROR();
        }
        //#endif
        sg.gl.cache = sg_gl_state_cache_t::default();
        sg_gl_cache_clear_buffer_bindings(sg, true);
        //_SG_GL_CHECK_ERROR();
        sg_gl_cache_clear_texture_bindings(sg, true);
        //_SG_GL_CHECK_ERROR();
        for i in 0..sg.limits.max_vertex_attrs {
            let attr = &mut sg.gl.cache.attrs[i as usize].gl_attr;
            attr.vb_index = -1;
            attr.divisor = -1;
            glDisableVertexAttribArray(i as GLuint);
            //_SG_GL_CHECK_ERROR();
        }
        sg.gl.cache.cur_primitive_type = GL_TRIANGLES;

        /* shader program */
        let mut get_int = 0;
        glGetIntegerv(GL_CURRENT_PROGRAM, &mut get_int);
        sg.gl.cache.prog = get_int as u32;
        //_SG_GL_CHECK_ERROR();

        /* depth and stencil state */
        sg.gl.cache.depth.compare = sg_compare_func::ALWAYS;
        sg.gl.cache.stencil.front.compare = sg_compare_func::ALWAYS;
        sg.gl.cache.stencil.front.fail_op = sg_stencil_op::KEEP;
        sg.gl.cache.stencil.front.depth_fail_op = sg_stencil_op::KEEP;
        sg.gl.cache.stencil.front.pass_op = sg_stencil_op::KEEP;
        sg.gl.cache.stencil.back.compare = sg_compare_func::ALWAYS;
        sg.gl.cache.stencil.back.fail_op = sg_stencil_op::KEEP;
        sg.gl.cache.stencil.back.depth_fail_op = sg_stencil_op::KEEP;
        sg.gl.cache.stencil.back.pass_op = sg_stencil_op::KEEP;
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_ALWAYS);
        glDepthMask(GL_FALSE as u8);
        glDisable(GL_STENCIL_TEST);
        glStencilFunc(GL_ALWAYS, 0, 0);
        glStencilOp(GL_KEEP, GL_KEEP, GL_KEEP);
        glStencilMask(0);

        /* blend state */
        sg.gl.cache.blend.src_factor_rgb = sg_blend_factor::ONE;
        sg.gl.cache.blend.dst_factor_rgb = sg_blend_factor::ZERO;
        sg.gl.cache.blend.op_rgb = sg_blend_op::ADD;
        sg.gl.cache.blend.src_factor_alpha = sg_blend_factor::ONE;
        sg.gl.cache.blend.dst_factor_alpha = sg_blend_factor::ZERO;
        sg.gl.cache.blend.op_alpha = sg_blend_op::ADD;
        glDisable(GL_BLEND);
        glBlendFuncSeparate(GL_ONE, GL_ZERO, GL_ONE, GL_ZERO);
        glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
        glBlendColor(0.0, 0.0, 0.0, 0.0);

        /* standalone state */
        for mask in &mut sg.gl.cache.color_write_mask {
            *mask = sg_color_mask::RGBA;
        }
        sg.gl.cache.cull_mode = sg_cull_mode::NONE;
        sg.gl.cache.face_winding = sg_face_winding::CW;
        sg.gl.cache.sample_count = 1;
        glColorMask(GL_TRUE as u8, GL_TRUE as u8, GL_TRUE as u8, GL_TRUE as u8);
        glPolygonOffset(0.0, 0.0);
        glDisable(GL_POLYGON_OFFSET_FILL);
        glDisable(GL_CULL_FACE);
        glFrontFace(GL_CW);
        glCullFace(GL_BACK);
        glEnable(GL_SCISSOR_TEST);
        glDisable(GL_SAMPLE_ALPHA_TO_COVERAGE);
        glEnable(GL_DITHER);
        glDisable(GL_POLYGON_OFFSET_FILL);
        //#if defined(SOKOL_GLCORE33)
        glEnable(GL_MULTISAMPLE);
        glEnable(GL_PROGRAM_POINT_SIZE);
        //#endif
    }
}

fn sg_gl_activate_context(sg: &mut sg_state_t, ctx_id: sg_context) {
    debug_assert!(sg.gl.valid);
    /* NOTE: ctx can be 0 to unset the current context */
    //sg.gl.cur_context = ctx;
    unsafe {
        sg_gl_reset_state_cache(sg, ctx_id);
    }
}

fn sg_activate_context_internal(sg: &mut sg_state_t, ctx_id: sg_context) {
    sg_gl_activate_context(sg, ctx_id);
}

fn sg_setup_context(sg: &mut sg_state_t) -> sg_context {
    //SOKOL_ASSERT(_sg.valid);
    let slot_index = sg_pool_alloc_index(&mut sg.pools.context_pool);

    let ctx_id = if SG_INVALID_SLOT_INDEX != slot_index {
        let res_id = sg_context {
            id: sg_slot_alloc(
                &mut sg.pools.context_pool,
                &mut sg.pools.contexts[slot_index as usize].slot,
                slot_index,
            ),
        };

        sg_create_context(sg, res_id);
        debug_assert!(
            sg_context_at(&mut sg.pools, res_id.id).slot.state == sg_resource_state::VALID
        );
        sg_activate_context_internal(sg, res_id);
        res_id
    } else {
        /* pool is exhausted */
        sg_context { id: SG_INVALID_ID }
    };

    sg.active_context = ctx_id;
    return sg.active_context;
}

pub fn sg_setup(sg: &mut sg_state_t, desc: &sg_desc) {
    sg.desc = *desc;
    sg_setup_pools(&mut sg.pools, &sg.desc);
    //_sg_setup_commit_listeners(&_sg.desc);
    sg.frame_index = 1;
    sg_setup_backend(sg);
    sg.valid = true;
    sg_setup_context(sg);
}

/* called when _sg_gl_deinit_buffer() */
fn sg_gl_cache_invalidate_buffer(gl: &mut sg_gl_backend_t, buf: GLuint) {
    if buf == gl.cache.vertex_buffer {
        gl.cache.vertex_buffer = 0;
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, 0);
        }
    }
    if buf == gl.cache.index_buffer {
        gl.cache.index_buffer = 0;
        unsafe {
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    if buf == gl.cache.stored_vertex_buffer {
        gl.cache.stored_vertex_buffer = 0;
    }
    if buf == gl.cache.stored_index_buffer {
        gl.cache.stored_index_buffer = 0;
    }
    for attr in &mut gl.cache.attrs {
        if buf == attr.gl_vbuf {
            attr.gl_vbuf = 0;
        }
    }
}

fn sg_gl_discard_buffer(gl: &mut sg_gl_backend_t, buf: &mut sg_buffer_t) {
    //_SG_GL_CHECK_ERROR();
    for slot in 0..buf.cmn.num_slots as usize {
        if buf.gl.buf[slot] != 0 {
            sg_gl_cache_invalidate_buffer(gl, buf.gl.buf[slot]);
            if !buf.gl.ext_buffers {
                unsafe {
                    glDeleteBuffers(1, &buf.gl.buf[slot]);
                }
            }
        }
    }
    //_SG_GL_CHECK_ERROR();
}

fn sg_discard_buffer(gl: &mut sg_gl_backend_t, buf: &mut sg_buffer_t) {
    sg_gl_discard_buffer(gl, buf);
}

fn sg_gl_cache_active_texture(gl: &mut sg_gl_backend_t, texture: GLenum) {
    if gl.cache.cur_active_texture != texture {
        gl.cache.cur_active_texture = texture;
        unsafe {
            glActiveTexture(texture);
        }
    }
}

/* called from _sg_gl_destroy_texture() */
fn sg_gl_cache_invalidate_texture(gl: &mut sg_gl_backend_t, tex: GLuint) {
    for i in 0..SG_GL_IMAGE_CACHE_SIZE {
        let slot = gl.cache.textures[i as usize];
        if tex == slot.texture {
            sg_gl_cache_active_texture(gl, (GL_TEXTURE0 + i) as GLenum);
            unsafe {
                glBindTexture(slot.target, 0);
            }

            // Above copies the value to satisfy the borrow - set values here
            let slot = &mut gl.cache.textures[i as usize];
            slot.target = 0;
            slot.texture = 0;
        }
    }
    if tex == gl.cache.stored_texture.texture {
        gl.cache.stored_texture.target = 0;
        gl.cache.stored_texture.texture = 0;
    }
}

fn sg_gl_discard_image(gl: &mut sg_gl_backend_t, img: &mut sg_image_t) {
    //_SG_GL_CHECK_ERROR();
    for slot in 0..img.cmn.num_slots {
        let gl_tex = img.gl.tex[slot as usize];
        if gl_tex != 0 {
            sg_gl_cache_invalidate_texture(gl, gl_tex);
            if !img.gl.ext_textures {
                unsafe {
                    glDeleteTextures(1, &gl_tex);
                }
            }
        }
    }
    if img.gl.depth_render_buffer != 0 {
        unsafe {
            glDeleteRenderbuffers(1, &img.gl.depth_render_buffer);
        }
    }
    if img.gl.msaa_render_buffer != 0 {
        unsafe {
            glDeleteRenderbuffers(1, &img.gl.msaa_render_buffer);
        }
    }
    //_SG_GL_CHECK_ERROR();
}

fn sg_discard_image(gl: &mut sg_gl_backend_t, img: &mut sg_image_t) {
    sg_gl_discard_image(gl, img);
}

/* called from _sg_gl_discard_shader() */
fn sg_gl_cache_invalidate_program(gl: &mut sg_gl_backend_t, prog: GLuint) {
    if prog == gl.cache.prog {
        gl.cache.prog = 0;
        unsafe {
            glUseProgram(0);
        }
    }
}

fn sg_gl_discard_shader(gl: &mut sg_gl_backend_t, shd: &mut sg_shader_t) {
    //_SG_GL_CHECK_ERROR();
    if shd.gl.prog != 0 {
        sg_gl_cache_invalidate_program(gl, shd.gl.prog);
        unsafe {
            glDeleteProgram(shd.gl.prog);
        }
    }
    //_SG_GL_CHECK_ERROR();
}

fn sg_discard_shader(gl: &mut sg_gl_backend_t, shd: &mut sg_shader_t) {
    sg_gl_discard_shader(gl, shd);
}

/* called from _sg_gl_discard_pipeline() */
fn sg_gl_cache_invalidate_pipeline(gl: &mut sg_gl_backend_t, pip: &mut sg_pipeline_t) {
    if pip.slot.id == gl.cache.cur_pipeline_id.id {
        // DT_TODO: Check code change here comparing ids
        //gl.cache.cur_pipeline = 0;
        gl.cache.cur_pipeline_id.id = SG_INVALID_ID;
    }
}

fn sg_gl_discard_pipeline(gl: &mut sg_gl_backend_t, pip: &mut sg_pipeline_t) {
    sg_gl_cache_invalidate_pipeline(gl, pip);
}

fn sg_discard_pipeline(gl: &mut sg_gl_backend_t, pip: &mut sg_pipeline_t) {
    sg_gl_discard_pipeline(gl, pip);
}

fn sg_gl_discard_pass(gl: &mut sg_gl_backend_t, pass: &mut sg_pass_t) {
    debug_assert!(pass.slot.id != gl.cur_pass_id.id);
    //_SG_GL_CHECK_ERROR();
    if 0 != pass.gl.fb {
        unsafe {
            glDeleteFramebuffers(1, &pass.gl.fb);
        }
    }
    for att in &pass.gl.color_atts {
        if att.gl_msaa_resolve_buffer != 0 {
            unsafe {
                glDeleteFramebuffers(1, &att.gl_msaa_resolve_buffer);
            }
        }
    }
    if pass.gl.ds_att.gl_msaa_resolve_buffer != 0 {
        unsafe {
            glDeleteFramebuffers(1, &pass.gl.ds_att.gl_msaa_resolve_buffer);
        }
    }
    //_SG_GL_CHECK_ERROR();
}

fn sg_discard_pass(gl: &mut sg_gl_backend_t, pass: &mut sg_pass_t) {
    sg_gl_discard_pass(gl, pass);
}

fn sg_discard_all_resources(sg: &mut sg_state_t, ctx_id: u32) {
    let p = &mut sg.pools;
    /*  this is a bit dumb since it loops over all pool slots to
        find the occupied slots, on the other hand it is only ever
        executed at shutdown
        NOTE: ONLY EXECUTE THIS AT SHUTDOWN
              ...because the free queues will not be reset
              and the resource slots not be cleared!
    */
    for i in 1..p.buffer_pool.size as usize {
        if p.buffers[i].slot.ctx_id == ctx_id {
            let state = p.buffers[i].slot.state;
            if (state == sg_resource_state::VALID) || (state == sg_resource_state::FAILED) {
                sg_discard_buffer(&mut sg.gl, &mut p.buffers[i]);
            }
        }
    }
    for i in 1..p.image_pool.size as usize {
        if p.images[i].slot.ctx_id == ctx_id {
            let state = p.images[i].slot.state;
            if (state == sg_resource_state::VALID) || (state == sg_resource_state::FAILED) {
                sg_discard_image(&mut sg.gl, &mut p.images[i]);
            }
        }
    }
    for i in 1..p.shader_pool.size as usize {
        if p.shaders[i].slot.ctx_id == ctx_id {
            let state = p.shaders[i].slot.state;
            if (state == sg_resource_state::VALID) || (state == sg_resource_state::FAILED) {
                sg_discard_shader(&mut sg.gl, &mut p.shaders[i]);
            }
        }
    }
    for i in 1..p.pipeline_pool.size as usize {
        if p.pipelines[i].slot.ctx_id == ctx_id {
            let state = p.pipelines[i].slot.state;
            if (state == sg_resource_state::VALID) || (state == sg_resource_state::FAILED) {
                sg_discard_pipeline(&mut sg.gl, &mut p.pipelines[i]);
            }
        }
    }
    for i in 1..p.pass_pool.size as usize {
        if p.passes[i].slot.ctx_id == ctx_id {
            let state = p.passes[i].slot.state;
            if (state == sg_resource_state::VALID) || (state == sg_resource_state::FAILED) {
                sg_discard_pass(&mut sg.gl, &mut p.passes[i]);
            }
        }
    }
}

fn sg_discard_context_internal(sg: &mut sg_state_t, ctx_id: sg_context) {
    sg_gl_discard_context(sg, ctx_id);
}

pub fn sg_shutdown(sg: &mut sg_state_t) {
    /* can only delete resources for the currently set context here, if multiple
    contexts are used, the app code must take care of properly releasing them
    (since only the app code can switch between 3D-API contexts)
    */
    if sg.active_context.id != SG_INVALID_ID {
        let ctx = sg_lookup_context(&mut sg.pools, sg.active_context.id);
        if let Some(ctx) = ctx {
            sg_discard_all_resources(sg, sg.active_context.id);
            sg_discard_context_internal(sg, sg.active_context);
        }
    }
    sg_discard_backend(sg);
    //sg_discard_commit_listeners();
    //sg_discard_pools(&mut sg.pools);
    *sg = sg_state_t::default();
}

fn sg_gl_commit(sg: &mut sg_state_t) {
    debug_assert!(!sg.gl.in_pass);
    /* "soft" clear bindings (only those that are actually bound) */
    unsafe {
        sg_gl_cache_clear_buffer_bindings(sg, false);
        sg_gl_cache_clear_texture_bindings(sg, false);
    }
}

pub fn sg_commit(sg: &mut sg_state_t) {
    sg_gl_commit(sg)
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
