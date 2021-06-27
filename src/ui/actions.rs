use std::{cell::RefCell, rc::Rc};

use gdk_pixbuf::Colorspace;
use gtk::prelude::{ComboBoxExt, ImageExt, WidgetExt};
use image::GenericImageView;
use rand::Rng;

use crate::{avatar::upload_avatar, checks::{attribute_check::AttributeCheck, battle_check::BattleCheck, results::check_result::{CheckResult, CheckResultStatus}, skill_check_factory::SkillCheckFactory, spell_check::SpellCheck}, context::Context, optolith::{spell::Spell, weapon::OptolithWeapon}, ui::builder::*, webhook::fire_webhook};

use super::{clear_notebook, find_child_by_name};


pub fn role_parry_check(context: &mut Context, weapon: &OptolithWeapon, difficulty: i32) {
    let check_result = BattleCheck::parry(context, weapon, difficulty);
    fire_webhook(context, check_result.to_check_result());
}

pub fn role_attack_check(context: &mut Context, weapon: &OptolithWeapon, difficulty: i32) {
    let check_result = BattleCheck::attack(context, weapon, difficulty);
    fire_webhook(context, check_result.to_check_result());
}

pub fn role_skill_check(context: &mut Context, skill_id: &String, difficulty: i32) {
    let mut factory = SkillCheckFactory::new(context);
    let mut skill_check = factory.get_skill_check(skill_id.to_owned());
    let check_result = skill_check.check_skill(&difficulty);
   
    fire_webhook(context, check_result.to_check_result());
}

pub fn role_attribute_check(context: &mut Context, attribute_id: &String, difficulty: i32) {
    let mut skill_check = AttributeCheck::new(context, attribute_id.to_owned());
    let check_result = skill_check.check(&difficulty);

    fire_webhook(context, check_result.to_check_result());
}

pub fn role_spell_check(context: &mut Context, spell: &Spell, difficulty: i32) {
    let check_result = SpellCheck::check(context, spell, &difficulty);
    fire_webhook(context, check_result.to_check_result());
}

pub fn role_dodge_check(context: &mut Context, difficulty: i32) {
    let check_result = BattleCheck::dodge(context, difficulty);
    fire_webhook(context, check_result.to_check_result());
}

pub fn send_hero_status(context: &mut Context) {
    let mut msg = String::new();
    msg.push_str("**Zustand**\n");
    let health = context.heroes.active_hero().health();
    msg.push_str(format!("Lebensenergie: {:>2}\n", health).as_str());

    if context.heroes.active_hero().is_mage() {
        let asp = context.heroes.active_hero().astral_points();
        msg.push_str(format!("Astralpunkte: {:>2}", asp).as_str());    
    }

    let pain_level = context.heroes.active_hero().pain_level();
    msg.push_str(format!("Schmerz: {:>2}\n", pain_level).as_str());
    
    let discord_msg = CheckResult {
        message: msg,
        critical: false,
        status: CheckResultStatus::Information,

    };
    fire_webhook(context, discord_msg);
}

pub fn role_ini(context: &mut Context) 
{   
    let ini = context.heroes.active_hero().ini();
    let modification = condition_modification(context);
    let mut rng = rand::thread_rng();
    let dice_value = rng.gen_range(1..7);
    
    let ini_result = ini + dice_value + modification;

    let mut modification_str :String = String::default();
    if modification > 0 {
        modification_str.push_str("+");
        modification_str.push_str(modification.to_string().as_str());
    } else if modification < 0 {
        modification_str = modification.to_string();
    }

    let check = format!("{} {:>2} {:>2} +[{:>2}] = ", "INI", ini, modification_str, dice_value);    

    let webhook_msg = format!("**{skill_name}** {difficulty}\n \
                                `{check}` **{ini_result}**",
                                skill_name="Initative", 
                                difficulty=modification_str,                                 
                                check=check,
                                ini_result=ini_result);

    let ini_as_check_result = CheckResult {
        message: webhook_msg,
        critical: false,
        status: CheckResultStatus::Information,
    };                           
    fire_webhook(context, ini_as_check_result);
}

pub fn role_dice(dice_type: i32, context: &mut Context) 
{   
    let mut rng = rand::thread_rng();
    let dice_max = dice_type + 1;
    let dice_value = rng.gen_range(1..dice_max);
    
    let webhook_msg = format!("**Würfelwurf**\n \
                                `{check} = ` [**{dice_value:>2}**]",
                                check=format!("d{}", dice_type),
                                dice_value=dice_value);

    let dice_role_result = CheckResult {
        message: webhook_msg,
        critical: false,
        status: CheckResultStatus::Information,
    };                           
    fire_webhook(context, dice_role_result);
}

pub fn change_avatar(context: &mut Context, hero_select: &gtk::ComboBoxText) {
    let avatar_raw = base64::decode(&context.heroes.active_hero().avatar().split(',').collect::<Vec<&str>>()[1]);
    let mut avatar_buffer = image::load_from_memory(&avatar_raw.unwrap()).unwrap();
    avatar_buffer = avatar_buffer.resize(100, 100, image::imageops::FilterType::Lanczos3);
    let avatar_color_channels = 4; //(RGBA)
    let pixels_row_stride = (avatar_buffer.width() * avatar_color_channels + 3) & !3;
    let pixels = avatar_buffer.clone().into_rgba8().as_raw().to_owned();
    let avatar_pixbuf: gdk_pixbuf::Pixbuf = gdk_pixbuf::Pixbuf::from_mut_slice(pixels, Colorspace::Rgb, true, 8, avatar_buffer.width() as i32, avatar_buffer.height() as i32, pixels_row_stride as i32);
    let gtk_avatar = context.gtk_avatar.as_ref().unwrap();
    gtk_avatar.set_from_pixbuf(Some(&avatar_pixbuf));
}

pub fn change_hero(context: &Rc<RefCell<Context>>, hero_select: &gtk::ComboBoxText) {
    let hero_id = hero_select.active_id().expect("Unknown hero selected, this should not happen.");            
    context.borrow_mut().config.set_last_used_hero_id(hero_id.to_string());
    context.borrow_mut().heroes.set_active_hero(hero_id.to_string());
    
    upload_avatar(&mut context.borrow_mut());
    change_avatar(&mut context.borrow_mut(), &hero_select);
    reload_hero_stats(context);
}


pub fn reload_hero_stats(context: &Rc<RefCell<Context>>) {
    clear_notebook(&mut context.borrow_mut());
    //TODO reload hero status row (health, astral points, ...)
    // for this to work, the hero status has to be wraped into another box and this has to be referenced in context struct like the notebook.
    // maybee split this new box into two, so the config button will not be redrawn on character change
    ui_add_tab_attributes(&context);
    ui_add_tabs_skills(&context);
    ui_add_tab_battle(&context);
    ui_add_tab_magic(&context);
    ui_add_tab_dice(&context);

    context.borrow_mut().gtk_main_box.as_ref().unwrap().show_all();
}

pub fn condition_modification(context: &mut Context) -> i32 {
    let mut condition_mod = 0;
    condition_mod += context.heroes.active_hero().pain_level();

    return condition_mod * -1;
}