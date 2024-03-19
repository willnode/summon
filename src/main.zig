const std = @import("std");
const c = @cImport({
    @cInclude("ngx_config.h");
    @cInclude("ngx_core.h");
    @cInclude("ngx_http.h");
});

export fn ngx_http_summon_handler(r: *c.ngx_http_request_t) callconv(.C) c.ngx_int_t {
    // Example response
    const response = "Hello, World!";
    const response_len: u32 = 12;

    // Set the Content-Type header
    // _ = c.ngx_http_discard_request_body(r);
    // r.headers_out.content_type_len = c.ngx_string("text/plain").len;
    // r.headers_out.content_type = c.ngx_string("text/plain");
    // r.headers_out.status = c.NGX_HTTP_OK;
    // r.headers_out.content_length_n = response_len;

    // Send the headers of your response
    _ = c.ngx_http_send_header(r);

    // Send the body of your response
    var b: c.ngx_buf_t = undefined;
    b.start = @ptrCast(&response);
    b.pos = b.start;
    b.last = b.start + response_len;
    b.memory = 1; // This memory is not going to move, so it's okay to not mark it as volatile.
    b.last_buf = 1; // This is the last buffer in the buffer chain.

    var out: c.ngx_chain_t = undefined;
    out.buf = &b;
    out.next = null;

    return c.ngx_http_output_filter(r, &out);
}

export fn ngx_http_summon(cf: *c.ngx_conf_t, _: *c.ngx_command_t, _: *anyopaque) callconv(.C) c.ngx_int_t {
    // Cast the conf argument to the expected type if necessary.
    // Retrieve the first argument from the command.
    const args = cf.args;
    if (args.nelts >= 2) {
        const arg = @as(**c.char, @ptrCast(args.elts))[1];
        const ngx_log = cf.cycle.log;
        c.ngx_log_error(c.NGX_LOG_NOTICE, ngx_log, 0, "hello world: %s", arg);
    }
    // Cast the conf argument to the expected configuration structure and set the handler.
    const core_loc_conf = c.ngx_http_conf_get_module_loc_conf(cf, c.ngx_http_core_module);
    @as(*c.ngx_http_core_loc_conf_t, @ptrCast(core_loc_conf)).handler = ngx_http_summon_handler;

    return c.NGX_CONF_OK;
}

const ngx_http_summon_commands = [_]c.ngx_command_t{
    c.ngx_command_t{
        .name = c.ngx_string("summon_command"),
        .type = c.NGX_HTTP_LOC_CONF | c.NGX_CONF_TAKE1,
        .set = ngx_http_summon,
        .conf = 0,
        .offset = 0,
        .post = null,
    },
    c.ngx_command_t{
        .name = c.ngx_string("summon_user"),
        .type = c.NGX_HTTP_LOC_CONF | c.NGX_CONF_TAKE1,
        .set = ngx_http_summon,
        .conf = 0,
        .offset = 0,
        .post = null,
    },
    c.ngx_null_command,
};

const ngx_http_summon_module_ctx = c.ngx_http_module_t{
    .preconfiguration = null,
    .postconfiguration = null,
    .create_main_conf = null,
    .init_main_conf = null,
    .create_srv_conf = null,
    .merge_srv_conf = null,
    .create_loc_conf = null,
    .merge_loc_conf = null,
};

export const ngx_http_summon_module = c.ngx_module_t{
    .ctx_index = c.NGX_MODULE_V1,
    .index = c.NGX_MODULE_V1_PADDING,
    .name = &ngx_http_summon_module_ctx,
    .spare0 = 0,
    .spare1 = 0,
    .version = 1,
    .signature = c.NGX_MODULE_SIGNATURE,
    .ctx = &ngx_http_summon_module_ctx,
    .commands = &ngx_http_summon_commands,
    .type = c.NGX_HTTP_MODULE,
    .init_master = null,
    .init_module = null,
    .init_process = null,
    .init_thread = null,
    .exit_thread = null,
    .exit_process = null,
    .exit_master = null,
    .spare_hook0 = 0,
    .spare_hook1 = 0,
    .spare_hook2 = 0,
    .spare_hook3 = 0,
    .spare_hook4 = 0,
    .spare_hook5 = 0,
    .spare_hook6 = 0,
    .spare_hook7 = 0,
};

pub fn init() !void {

    // Additional setup or registration code can go here.
}

pub fn main() !void {
    try init();
    // Your main function or additional setup code.
}
