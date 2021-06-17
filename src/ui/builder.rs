use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::{Align, BoxExt, ButtonExt, ComboBoxExt, ComboBoxTextExt, ContainerExt, EditableSignals, EntryExt, LabelExt, ListBoxExt, SpinButtonExt, WidgetExt, prelude::{ComboBoxExtManual, NotebookExtManual}};

use crate::{context::Context, optolith::{spell::Spell, weapon::OptolithWeapon}, ui::{actions::*, get_check_difficulty, settings::display_config}};

use super::dialog::abort_app_with_message;


pub fn build_skill_name_label(skill_name: &String) -> gtk::Label {
    let lbl_skill_name = gtk::Label::new(Some(skill_name.as_str()));    
    lbl_skill_name.set_halign(gtk::Align::Start);
    lbl_skill_name
}

pub fn build_parry_check_button(context: &Rc<RefCell<Context>>, weapon: &OptolithWeapon) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("parry_check_button#{}", weapon.id());
    let difficulty_widget_name = format!("parry_difficulty#{}", weapon.id());
    btn_die.set_widget_name(widget_name.as_str());
    let aweapon_tmp = weapon.clone();
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;    
        role_parry_check(&mut context.borrow_mut(), &aweapon_tmp, difficulty);
    }));
    return btn_die;
}

pub fn build_attack_check_button(context: &Rc<RefCell<Context>>, weapon: &OptolithWeapon) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("attack_check_button#{}", weapon.id());
    let difficulty_widget_name = format!("attack_difficulty#{}", weapon.id());
    btn_die.set_widget_name(widget_name.as_str());
    let weapon_tmp = weapon.clone();
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;
        role_attack_check(&mut context.borrow_mut(), &weapon_tmp, difficulty);
    }));
    return btn_die;
}

pub fn build_spell_check_button(context: &Rc<RefCell<Context>>, spell: &Spell) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("spell_button#{}", spell.id());
    let difficulty_widget_name = format!("spell_difficulty#{}", spell.id());
    btn_die.set_widget_name(widget_name.as_str());
    let spell_tmp = spell.clone();
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;
        role_spell_check(&mut context.borrow_mut(), &spell_tmp, difficulty);
    }));
    return btn_die;
}

pub fn build_dodge_check_button(context: &Rc<RefCell<Context>>, dodge_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("dodge_check_button#{}", dodge_id);
    let difficulty_widget_name = format!("dodge_difficulty#{}", dodge_id);
    btn_die.set_widget_name(widget_name.as_str());
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;
        role_dodge_check(&mut context.borrow_mut(), difficulty);
    }));
    return btn_die;
}

pub fn build_skill_check_button(context: &Rc<RefCell<Context>>, skill_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::with_label("🎲");
    let widget_name = format!("skill_check_button#{}", skill_id);
    let difficulty_widget_name = format!("skill_difficulty#{}", skill_id);
    btn_die.set_widget_name(widget_name.as_str());
    let skill_id_tmp = skill_id.to_string();    
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;
        role_skill_check(&mut context.borrow_mut(), &skill_id_tmp, difficulty);
    }));
    return btn_die;
}

pub fn build_attribute_check_button(context: &Rc<RefCell<Context>>, attribute_id: &str) -> gtk::Button {
    let btn_die = gtk::Button::new(); //::with_label("<span size='20'>big text</span>🎲");
    let btn_die_label: gtk::Label = gtk::Label::new(None);
    btn_die_label.set_use_markup(true);
    // TODO check die size
    btn_die_label.set_markup("<span size='14000'>🎲</span>");
    btn_die.add(&btn_die_label);    
    let widget_name = format!("attribute_check_button#{}", attribute_id);
    let difficulty_widget_name = format!("attribute_difficulty#{}", attribute_id);
    btn_die.set_widget_name(widget_name.as_str());
    let attribute_id_tmp = attribute_id.to_string();
    btn_die.connect_clicked(clone!(@weak context => move |but| {
        //let hero_id = get_hero_id(&but);
        //let attribute_id = get_skill_id(&but.clone().upcast::<gtk::Widget>());
        let condition_modification = condition_modification(&mut context.borrow_mut());
        let difficulty = get_check_difficulty(&but, &difficulty_widget_name) + condition_modification;
        role_attribute_check(&mut context.borrow_mut(), &attribute_id_tmp, difficulty);
    }));
    return btn_die;
}

pub fn build_skill_checks_label(skill_id: &String, context: &mut Context) -> gtk::Label {
    let attribute_ids = context.skills.by_id(skill_id).get_check();
    let check_name_abbr = context.attributes.name_abbrs(attribute_ids);
    
    let lbl_skill_test = gtk::Label::new(Some(check_name_abbr.join(" / ").as_str()));
    lbl_skill_test.set_justify(gtk::Justification::Right);
    lbl_skill_test.set_property_width_request(100);
    return lbl_skill_test;
}

pub fn build_spell_checks_label(spell: &Spell, context: &mut Context) -> gtk::Label {
    let attribute_ids: Vec<String> = spell.check().to_owned();
    let check_name_abbr = context.attributes.name_abbrs(attribute_ids);
    
    let lbl_spell_test = gtk::Label::new(Some(check_name_abbr.join(" / ").as_str()));
    lbl_spell_test.set_justify(gtk::Justification::Right);
    lbl_spell_test.set_property_width_request(100);
    return lbl_spell_test;
}

pub fn build_skill_difficulty_entry(context: &Rc<RefCell<Context>>, skill_id: &str) -> gtk::Entry {
    let widget_name = format!("skill_difficulty#{}", skill_id);
    let en_skill_check_difculty = build_default_dificulty_entry_field(widget_name.as_str());    
    let skill_id_tmp = skill_id.to_string();
    en_skill_check_difculty.connect_activate(clone!(@weak context => move |entry| {
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_skill_check(&mut context.borrow_mut(), &skill_id_tmp, difficulty);
    }));
    en_skill_check_difculty
}

pub fn build_attribute_difficulty_entry(context: &Rc<RefCell<Context>>, attribute_id: &str) -> gtk::Entry {
    let widget_name = format!("attribute_difficulty#{}", attribute_id);
    let en_attribute_check_difculty = build_default_dificulty_entry_field(widget_name.as_str());
    let attribute_id_tmp = attribute_id.to_string();
    en_attribute_check_difculty.connect_activate(clone!(@weak context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_attribute_check(&mut context.borrow_mut(), &attribute_id_tmp, difficulty);
    }));
    en_attribute_check_difculty
}

pub fn build_attack_difficulty_entry(context: &Rc<RefCell<Context>>, weapon_id: &str) -> gtk::Entry {
    let widget_name = format!("attack_difficulty#{}", weapon_id);
    let en_attack_difculty = build_default_dificulty_entry_field(widget_name.as_str());
    let attribute_id_tmp = weapon_id.to_string();
    en_attack_difculty.connect_activate(clone!(@weak context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_attribute_check(&mut context.borrow_mut(), &attribute_id_tmp, difficulty);
    }));
    en_attack_difculty
}

pub fn build_spell_difficulty_entry(context: &Rc<RefCell<Context>>, spell: &Spell) -> gtk::Entry {
    let widget_name = format!("spell_difficulty#{}", spell.id());
    let en_spell_difculty = build_default_dificulty_entry_field(widget_name.as_str());
    let clone_spell = spell.clone();
    en_spell_difculty.connect_activate(clone!(@weak context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_spell_check(&mut context.borrow_mut(), &clone_spell, difficulty);
    }));
    en_spell_difculty
}

pub fn build_dodge_difficulty_entry(context: &Rc<RefCell<Context>>, dodge_id: &str) -> gtk::Entry {
    let widget_name = format!("dodge_difficulty#{}", dodge_id);
    let en_dodge_difculty = build_default_dificulty_entry_field(widget_name.as_str());
    en_dodge_difculty.connect_activate(clone!(@weak context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_dodge_check(&mut context.borrow_mut(), difficulty);
    }));
    en_dodge_difculty
}

pub fn build_parry_difficulty_entry(context: &Rc<RefCell<Context>>, weapon: &OptolithWeapon) -> gtk::Entry {
    let widget_name = format!("parry_difficulty#{}", weapon.id());
    let en_parry_difculty = build_default_dificulty_entry_field(widget_name.as_str());
    let weapon_tmp = weapon.clone();
    en_parry_difculty.connect_activate(clone!(@weak context => move |entry| {        
        let difficulty = entry.get_text().to_string().parse::<i32>().or::<i32>(Ok(0)).unwrap();
        role_parry_check(&mut context.borrow_mut(), &weapon_tmp, difficulty);
    }));
    en_parry_difculty
}

pub fn build_default_dificulty_entry_field(widget_name: &str) -> gtk::Entry {
    let entry = gtk::Entry::new();
    entry.set_widget_name(widget_name);
    entry.set_alignment(0.5);
    entry.set_placeholder_text(Some("+/-"));
    entry.set_width_chars(4);
    entry.set_max_length(4);
    return entry
}

pub fn build_hero_select(context: &mut Context) -> gtk::ComboBoxText {
    let hero_list = context.heroes.simple_hero_list();
    if hero_list.len() == 0 {
        abort_app_with_message("We need more heroes!", "No heroes found in heroes.json");
    }
    let hero_select = gtk::ComboBoxText::new();
    for hero in hero_list {
        hero_select.append(Some(hero.id.as_str()), hero.name.as_str());
    }
    
    hero_select.set_widget_name("hero_select");
    if context.heroes.active_hero_id().is_empty() {
        hero_select.set_active(Some(0));
        let active_hero = hero_select.get_active_id().unwrap().to_string();
        context.heroes.set_active_hero(active_hero);
    } else {
        hero_select.set_active_id(Some(context.heroes.active_hero_id().as_str()));
    }
    
    return hero_select;
}

pub fn ui_add_dodge_to_tab(context: &Rc<RefCell<Context>>, tab: &gtk::ListBox) {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let weapon_name = gtk::Label::new(Some("Ausweichen"));
    weapon_name.set_halign(Align::Start);
    row.add(&weapon_name);
    row.set_child_packing(&weapon_name, true, true, 0, gtk::PackType::Start);
    
    let dodge_value = context.borrow_mut().heroes.active_hero().dodge_value();

    let at_value =  gtk::Label::new(Some(dodge_value.to_string().as_str()));  
    row.add(&at_value);

    let en_attack_test_difculty = build_dodge_difficulty_entry(&context, "dodge");
    row.add(&en_attack_test_difculty);
    let btn_die = build_dodge_check_button(&context, "dodge");
    row.add(&btn_die);
    
    tab.add(&row);
}

pub fn ui_add_tab_dice(context: &Rc<RefCell<Context>>) {
    let lbo_dice = gtk::ListBox::new();
    lbo_dice.set_selection_mode(gtk::SelectionMode::None);
    let nb_tab_name = gtk::Label::new(Some("Würfel"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_dice, Some(&nb_tab_name));

    let dice_list: Vec<(&str, &str)> = vec![
        ("6", "./assets/icons/d6.png"),    
        ("20", "./assets/icons/d20.png"),
        ("4", "./assets/icons/d4.png"),
        ("8", "./assets/icons/d8.png"),
        ("10", "./assets/icons/d10.png"),
        ("12", "./assets/icons/d12.png"),
        ("2", "./assets/icons/d2.png"),
    ];

    let mut dice_row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    
    let mut box_child_count = 0;
    for (dice_name, icon_path) in dice_list {
        let img_buf: gdk_pixbuf::Pixbuf = gdk_pixbuf::Pixbuf::from_file(icon_path).unwrap().scale_simple(80,80, gdk_pixbuf::InterpType::Bilinear).unwrap();
        let img = gtk::Image::from_pixbuf(Some(&img_buf));
        let dice_button = gtk::Button::new();
        dice_button.set_image(Some(&img));
        //dice_button.set_label(dice_name);
        dice_button.set_always_show_image(true);
        let tmp_dice = dice_name.clone();
        dice_button.connect_clicked(move |_| {
            dbg!(&tmp_dice);
        });

        dice_row.add(&dice_button);
        dice_row.set_child_packing(&dice_button, true, true, 0, gtk::PackType::Start);
        box_child_count += 1;

        if box_child_count == 2 {
            box_child_count = 0;
            lbo_dice.add(&dice_row.clone());
            dice_row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        }
    }

    if box_child_count == 1 {        
        lbo_dice.add(&dice_row.clone());    
    }
}

pub fn ui_add_tab_magic(context: &Rc<RefCell<Context>>) {
    let lbo_spells = gtk::ListBox::new();
    lbo_spells.set_selection_mode(gtk::SelectionMode::None);
    let nb_tab_name = gtk::Label::new(Some("Magie"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_spells, Some(&nb_tab_name));

    let spells = context.borrow_mut().heroes.active_hero().spells();
    for spell in spells {
        let box_spell = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let lbl_spell_name = gtk::Label::new(Some(spell.name()));    
        lbl_spell_name.set_halign(gtk::Align::Start);
        box_spell.add(&lbl_spell_name);
        box_spell.set_child_packing(&lbl_spell_name, true, true, 0, gtk::PackType::Start);        

        let lbl_checks = build_spell_checks_label(&spell, &mut context.borrow_mut());
        box_spell.add(&lbl_checks);

        let lbl_spell_points = gtk::Label::new(Some(&spell.points().to_string()));
        lbl_spell_points.set_halign(gtk::Align::End);
        lbl_spell_points.set_justify(gtk::Justification::Right);
        lbl_spell_points.set_property_width_request(30);
        lbl_spell_points.set_widget_name(&format!("spell_id#{}",&spell.id()));
        box_spell.add(&lbl_spell_points);            

        let en_spell_check_dificulty = build_spell_difficulty_entry(&context, &spell);
        box_spell.add(&en_spell_check_dificulty);

        let btn_die = build_spell_check_button(&context, &spell);
        box_spell.add(&btn_die);

        lbo_spells.add(&box_spell);
    }
}

pub fn build_hero_status_box(context: &Rc<RefCell<Context>>) -> gtk::Box{
    let hero_status_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
    hero_status_box.set_margin_start(15);
    hero_status_box.set_margin_end(15);

    let health = gtk::SpinButton::with_range(0.0, 1000.0, 1.0);
    health.set_alignment(0.5);
    health.set_value(0.0);
    health.set_widget_name("health_points");
    health.connect_changed(clone!(@weak context => move |health| {
        context.borrow_mut().heroes.active_hero().set_health(health.get_value_as_int());
    }));
    let health_label = gtk::Label::new(Some("LE"));
    hero_status_box.add(&health_label);
    hero_status_box.add(&health);

    if context.borrow_mut().heroes.active_hero().is_mage() {
        let asp = gtk::SpinButton::with_range(0.0, 1000.0, 1.0);
        asp.set_alignment(0.5);
        asp.set_value(0.0);
        asp.set_widget_name("astral_points");
        asp.connect_changed(clone!(@weak context => move |asp| {
            context.borrow_mut().heroes.active_hero().set_astral_points(asp.get_value_as_int());
        }));
        let asp_label = gtk::Label::new(Some("AsP"));
        hero_status_box.add(&asp_label);
        hero_status_box.add(&asp);
    }

    let pain = gtk::SpinButton::with_range(0.0, 4.0, 1.0);
    pain.set_alignment(0.5);
    pain.set_widget_name("pain_level");
    pain.connect_changed(clone!(@weak context => move |pain| {
        context.borrow_mut().difficulty.pain_level = pain.get_value_as_int();
        context.borrow_mut().heroes.active_hero().set_pain_level(pain.get_value_as_int());
    }));
    let pain_label = gtk::Label::new(Some("Schmerz"));
    hero_status_box.add(&pain_label);
    hero_status_box.add(&pain);

    let ini_button_lable = format!("Ini. ({}) 🎲", context.borrow_mut().heroes.active_hero().ini());
    let ini_button = gtk::Button::with_label(&ini_button_lable);
    ini_button.connect_clicked(clone!(@weak context => move |_| {
        role_ini(&mut context.borrow_mut());
    }));
    hero_status_box.add(&ini_button);

    let config_button_label = String::from("⚙️");
    let config_button = gtk::Button::with_label(&config_button_label);
    config_button.connect_clicked(clone!(@weak context => move |_| {
        display_config(&context);
    }));
    hero_status_box.add(&config_button);

    return hero_status_box;
}


pub fn ui_add_tabs_skills(context: &Rc<RefCell<Context>>) {
    let skill_groups_order = context.borrow().skills.group_order();
    for skill_group in skill_groups_order {     
        let skills = &context.borrow().skills.by_group.get(&skill_group).unwrap().clone();
        let lbo_skills = gtk::ListBox::new();        
        lbo_skills.set_selection_mode(gtk::SelectionMode::None);

        let nb_tab_name = gtk::Label::new(Some(&skill_group));
        context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_skills, Some(&nb_tab_name));

        for skill in skills {
            let box_skill = gtk::Box::new(gtk::Orientation::Horizontal, 0);

            let lbl_skill_name = build_skill_name_label(&skill.name);
            box_skill.add(&lbl_skill_name);
            box_skill.set_child_packing(&lbl_skill_name, true, true, 0, gtk::PackType::Start);

            let lbl_checks = build_skill_checks_label(&skill.id, &mut context.borrow_mut());
            box_skill.add(&lbl_checks);
            
            let lbl_skill_points = gtk::Label::new(Some(context.borrow_mut().heroes.active_hero().skill_points(&skill.id).to_string().as_str()));
            lbl_skill_points.set_halign(gtk::Align::End);
            lbl_skill_points.set_justify(gtk::Justification::Right);
            lbl_skill_points.set_property_width_request(30);
            lbl_skill_points.set_widget_name(&format!("skill_id#{}",&skill.id));
            box_skill.add(&lbl_skill_points);            

            let en_skill_check_difculty = build_skill_difficulty_entry(&context, &skill.id);
            box_skill.add(&en_skill_check_difculty);

            let btn_die = build_skill_check_button(&context, &skill.id);
            box_skill.add(&btn_die);

            lbo_skills.add(&box_skill);
        }
    }
}

pub fn ui_add_tab_attributes(context: &Rc<RefCell<Context>>) {
    let lbo_attributes = gtk::ListBox::new();
    lbo_attributes.set_selection_mode(gtk::SelectionMode::None);

    let nb_tab_name = gtk::Label::new(Some("Attribute"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_attributes, Some(&nb_tab_name));

    let attributes = context.borrow().attributes.clone().all().to_owned();
    for attribute in attributes {
        let box_attribute = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let lbl_attribute_name = build_skill_name_label(&attribute.name);
        box_attribute.add(&lbl_attribute_name);
        box_attribute.set_child_packing(&lbl_attribute_name, true, true, 0, gtk::PackType::Start);
        
        let lbl_attribute_value = gtk::Label::new(Some(context.borrow_mut().heroes.active_hero().attribute_value(&attribute.id.to_string()).to_string().as_str()));
        lbl_attribute_value.set_halign(gtk::Align::End);
        lbl_attribute_value.set_justify(gtk::Justification::Right);
        lbl_attribute_value.set_property_width_request(30);
        lbl_attribute_value.set_widget_name(&format!("attribute_id#{}",&attribute.id));
        box_attribute.add(&lbl_attribute_value);
        
        let en_atribute_test_difculty = build_attribute_difficulty_entry(&context, &attribute.id);
        box_attribute.add(&en_atribute_test_difculty);

        let btn_die = build_attribute_check_button(&context, &attribute.id);
        box_attribute.add(&btn_die);

        lbo_attributes.add(&box_attribute);
    }
}

pub fn ui_add_tab_battle(context: &Rc<RefCell<Context>>) {
    let lbo_weapons = gtk::ListBox::new();
    
    lbo_weapons.set_selection_mode(gtk::SelectionMode::None);

    let nb_tab_name = gtk::Label::new(Some("Kampf"));
    context.borrow_mut().gtk_notebook.as_ref().unwrap().append_page(&lbo_weapons, Some(&nb_tab_name));

    ui_add_dodge_to_tab(context, &lbo_weapons);

    let weapons = context.borrow_mut().heroes.active_hero().weapons();
    for weapon in weapons {
        let weapon_name = gtk::Label::new(Some(weapon.name()));
        weapon_name.set_halign(Align::Start);
        lbo_weapons.add(&weapon_name);

        let row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        row.set_halign(Align::End);
        
        let attack_value = context.borrow_mut().heroes.active_hero().attack_value(&weapon);

        let at_label =  gtk::Label::new(Some("AT"));  
        row.add(&at_label);
        let at_value =  gtk::Label::new(Some(format!("{:>2}", attack_value).as_str()));  
        row.add(&at_value);

        let en_attack_test_difculty = build_attack_difficulty_entry(&context, &weapon.id());
        row.add(&en_attack_test_difculty);
        let btn_die = build_attack_check_button(&context, &weapon);
        row.add(&btn_die);

        if !weapon.is_range_weapon() {
            let slash =  gtk::Label::new(Some(" / ")); 
            row.add(&slash);

            let ct_primary_attributes = context.borrow().combat_techniques.primary_attributes(weapon.combat_technique());
            let parry_value = context.borrow_mut().heroes.active_hero().parry_value(&weapon, ct_primary_attributes);
            let pa_label =  gtk::Label::new(Some("PA")); 
            row.add(&pa_label);
            let pa_value =  gtk::Label::new(Some(format!("{:>2}", parry_value).as_str())); 
            row.add(&pa_value);
            let en_parry_test_difculty = build_parry_difficulty_entry(&context, &weapon);
            row.add(&en_parry_test_difculty);
            let btn_die = build_parry_check_button(&context, &weapon);
            row.add(&btn_die);            
        }
        
        lbo_weapons.add(&row);
    }
}