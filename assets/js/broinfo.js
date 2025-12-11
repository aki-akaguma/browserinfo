
function get_navigator_prop (prop, init_val) {
    if (prop in navigator) {
        return eval("navigator." + prop);
    } else {
        return init_val;
    }
}

function get_window_prop (prop, init_val) {
    if (prop in window) {
        return eval("window." + prop);
    } else {
        return init_val;
    }
}

function get_screen_prop (prop, init_val) {
    if (prop in window.screen) {
        return eval("window.screen." + prop);
    } else {
        return init_val;
    }
}

function get_document_prop (prop, init_val) {
    if (prop in document) {
        return eval("document." + prop);
    } else {
        return init_val;
    }
}

function is_dark_mode() {
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
}

function get_timezone() {
    // Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (Intl) {
        if (Intl.DateTimeFormat) {
            var v = Intl.DateTimeFormat();
            if (v) {
                if (v.resolvedOptions) {
                    var vv = v.resolvedOptions();
                    if (vv) {
                        if (vv.timeZone) {
                            return vv.timeZone;
                        }
                    }
                }
            }
        }
    }
    return '';
}

const v_oscpu = get_navigator_prop('oscpu', '');
const v_platform = get_navigator_prop('platform', '');
const v_cpu_cores = get_navigator_prop('hardwareConcurrency', null);
const v_user_agent = get_navigator_prop('userAgent', '');
const v_cookie_enabled = get_navigator_prop('cookieEnabled', false);
const v_user_language = get_navigator_prop('language', '');
const v_device_memory = get_navigator_prop('deviceMemory', null);

const v_referrer = get_document_prop('referrer', '');

const v_screen_width = get_screen_prop('width', null);
const v_screen_height = get_screen_prop('height', null);
const v_screen_color_depth = get_screen_prop('colorDepth', null);
const v_device_pixcel_ratio = get_window_prop('devicePixelRatio', null);

const v_has_local_storage = (typeof localStorage != 'undefined');
const v_has_session_storage = (typeof sessionStorage != 'undefined');
const v_is_dark_mode = is_dark_mode();
const v_timezone = get_timezone();

return {
    basic: {
        referrer: v_referrer,
        user_agent: v_user_agent,
    },
    jsinfo: {
        oscpu: v_oscpu,
        platform: v_platform,
        cpu_cores: v_cpu_cores,
        cookie_enabled: v_cookie_enabled,
        user_language: v_user_language,
        device_memory: v_device_memory,

        screen_width: v_screen_width,
        screen_height: v_screen_height,
        screen_color_depth: v_screen_color_depth,
        device_pixcel_ratio: v_device_pixcel_ratio,

        has_local_storage: v_has_local_storage,
        has_session_storage: v_has_session_storage,
        is_dark_mode: v_is_dark_mode,
        timezone: v_timezone,
    },
};
/* vim: set ts=4 sw=4 sts=0 expandtab: ### mode line for vim */
