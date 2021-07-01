use discord_webhook::{DiscordWebHook, Embed};

use crate::{avatar::build_avatar_url, checks::results::check_result::{CheckResult, CheckResultStatus}, context::Context, ui::dialog::{display_error, display_reqwest_error}};

const COLOR_SUCCESS: u32 = 65280;
const COLOR_FAILURE: u32 = 16711680;
const COLOR_INFORMATION: u32 = 5814783;

pub fn fire_webhook(context: &mut Context, die_result: CheckResult) {
    let mut embed = Embed::default();
    embed.description = Some(die_result.message);
    embed.color = match die_result.status {
        CheckResultStatus::Success => Some(COLOR_SUCCESS),
        CheckResultStatus::Failure => Some(COLOR_FAILURE),
        CheckResultStatus::Information => Some(COLOR_INFORMATION),
        _ => None,
    };

    let mut webhook = DiscordWebHook::new_with_embed(context.config.webhook_url().as_str(), embed.clone());

    let avatar_url = build_avatar_url(context);
    if !avatar_url.is_empty() {        
        webhook.set_avatar_url(avatar_url.as_str());
    }    

    webhook.set_username(context.characters.active().name().as_str());
    let webhook_result = webhook.fire();
    
    
    if webhook_result.is_err() {
        dbg!(&webhook_result);
        display_reqwest_error("Discord Webhock Error", &webhook_result.err().unwrap());
        return;
    }

    let hook_result = webhook_result.unwrap();
    let status_code = hook_result.status().clone();
    if status_code != 204 {   
        dbg!(&embed);
        dbg!(&hook_result);     
        display_error("Discord Webhock Error", format!("Unbekannter fehler beim senden and Discord.\nDie Server Antwort lautet: {}", status_code).as_str());
    }
}


