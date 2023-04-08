

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

pub fn sg_shutdown(){

}

pub fn sg_commit(){
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