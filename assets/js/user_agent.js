
function get_navigator_prop (prop, init_val) {
    if (prop in navigator) {
        return eval("navigator." + prop);
    } else {
        return init_val;
    }
}

const v_user_agent = get_navigator_prop('userAgent', '');
return v_user_agent;
/* vim: set ts=4 sw=4 sts=0 expandtab: ### mode line for vim */
