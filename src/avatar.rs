use crate::context::Context;


pub fn upload_avatar(context: &mut Context) {
    if context.config.is_avatar_uploader_url_set() {
        context.heroes.active_hero().upload_avatar(context.config.avatar_uploader_url());            
    }
}

pub fn build_avatar_url(context: &mut Context) -> String {
    if !context.config.use_avatar() {
        return String::default();
    }

    if context.config.is_avatar_static_url_set() {
        return context.config.avatar_static_url();
    }

    let mut avatar_url = context.config.avatar_base_url();
    if !avatar_url.ends_with("/") {
        avatar_url.push_str("/");
    }
    avatar_url.push_str(context.heroes.active_hero().get_avatar_file_name().as_str());       
    return avatar_url;
}
