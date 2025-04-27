use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::rng;
use rand::prelude::IndexedRandom;

static PROC_NAMES_MAP: Lazy<Mutex<HashMap<&str, bool>>> = Lazy::new(|| {
    let proc_names_map = Mutex::new(HashMap::new());
    proc_names_map.lock().unwrap().insert("eisenhower", false);
    proc_names_map.lock().unwrap().insert("patton", false);
    proc_names_map.lock().unwrap().insert("macarthur", false);
    proc_names_map.lock().unwrap().insert("bradley", false);
    proc_names_map.lock().unwrap().insert("clark", false);
    proc_names_map.lock().unwrap().insert("stilwell", false);
    proc_names_map.lock().unwrap().insert("hodges", false);
    proc_names_map.lock().unwrap().insert("krueger", false);
    proc_names_map.lock().unwrap().insert("truscott", false);
    proc_names_map.lock().unwrap().insert("patch", false);
    proc_names_map.lock().unwrap().insert("buckner", false);
    proc_names_map.lock().unwrap().insert("smith", false);
    proc_names_map.lock().unwrap().insert("wedemeyer", false);
    proc_names_map.lock().unwrap().insert("mcnair", false);
    proc_names_map.lock().unwrap().insert("spaatz", false);
    proc_names_map.lock().unwrap().insert("montgomery", false);
    proc_names_map.lock().unwrap().insert("brooke", false);
    proc_names_map.lock().unwrap().insert("alexander", false);
    proc_names_map.lock().unwrap().insert("wavell", false);
    proc_names_map.lock().unwrap().insert("auchinleck", false);
    proc_names_map.lock().unwrap().insert("freyberg", false);
    proc_names_map.lock().unwrap().insert("dempsey", false);
    proc_names_map.lock().unwrap().insert("slim", false);
    proc_names_map.lock().unwrap().insert("scobie", false);
    proc_names_map.lock().unwrap().insert("wilson", false);
    proc_names_map.lock().unwrap().insert("oconnor", false);
    proc_names_map.lock().unwrap().insert("hobart", false);
    proc_names_map.lock().unwrap().insert("ironside", false);
    proc_names_map.lock().unwrap().insert("horrocks", false);
    proc_names_map.lock().unwrap().insert("morgan", false);
    proc_names_map.lock().unwrap().insert("rommel", false);
    proc_names_map.lock().unwrap().insert("guderian", false);
    proc_names_map.lock().unwrap().insert("manstein", false);
    proc_names_map.lock().unwrap().insert("rundstedt", false);
    proc_names_map.lock().unwrap().insert("keitel", false);
    proc_names_map.lock().unwrap().insert("jodl", false);
    proc_names_map.lock().unwrap().insert("hoth", false);
    proc_names_map.lock().unwrap().insert("model", false);
    proc_names_map.lock().unwrap().insert("kesselring", false);
    proc_names_map.lock().unwrap().insert("bock", false);
    proc_names_map.lock().unwrap().insert("student", false);
    proc_names_map.lock().unwrap().insert("paulus", false);
    proc_names_map.lock().unwrap().insert("dietrich", false);
    proc_names_map.lock().unwrap().insert("busch", false);
    proc_names_map.lock().unwrap().insert("kleist", false);
    proc_names_map.lock().unwrap().insert("tojo", false);
    proc_names_map.lock().unwrap().insert("yamashita", false);
    proc_names_map.lock().unwrap().insert("kuribayashi", false);
    proc_names_map.lock().unwrap().insert("homma", false);
    proc_names_map.lock().unwrap().insert("imamura", false);
    proc_names_map.lock().unwrap().insert("yamamoto", false);
    proc_names_map.lock().unwrap().insert("hata", false);
    proc_names_map.lock().unwrap().insert("anami", false);
    proc_names_map.lock().unwrap().insert("umezu", false);
    proc_names_map.lock().unwrap().insert("ushijima", false);
    proc_names_map.lock().unwrap().insert("zhukov", false);
    proc_names_map.lock().unwrap().insert("konev", false);
    proc_names_map.lock().unwrap().insert("rokossovsky", false);
    proc_names_map.lock().unwrap().insert("vasilevsky", false);
    proc_names_map.lock().unwrap().insert("chuikov", false);
    proc_names_map.lock().unwrap().insert("malinovsky", false);
    proc_names_map.lock().unwrap().insert("yeremenko", false);
    proc_names_map.lock().unwrap().insert("vatutin", false);
    proc_names_map.lock().unwrap().insert("meretskov", false);
    proc_names_map.lock().unwrap().insert("timoshenko", false);
    proc_names_map.lock().unwrap().insert("govorov", false);
    proc_names_map.lock().unwrap().insert("tolbukhin", false);
    proc_names_map.lock().unwrap().insert("batov", false);
    proc_names_map.lock().unwrap().insert("bagramyan", false);
    proc_names_map.lock().unwrap().insert("katukov", false);
    proc_names_map.lock().unwrap().insert("cavallero", false);
    proc_names_map.lock().unwrap().insert("graziani", false);
    proc_names_map.lock().unwrap().insert("badoglio", false);
    proc_names_map.lock().unwrap().insert("messe", false);
    proc_names_map.lock().unwrap().insert("roatta", false);
    proc_names_map.lock().unwrap().insert("degaulle", false);
    proc_names_map.lock().unwrap().insert("delattre", false);
    proc_names_map.lock().unwrap().insert("leclerc", false);
    proc_names_map.lock().unwrap().insert("giraud", false);
    proc_names_map.lock().unwrap().insert("weygand", false);
    proc_names_map.lock().unwrap().insert("chiang", false);
    proc_names_map.lock().unwrap().insert("cheng", false);
    proc_names_map.lock().unwrap().insert("bai", false);
    proc_names_map.lock().unwrap().insert("wei", false);
    proc_names_map.lock().unwrap().insert("li", false);
    proc_names_map.lock().unwrap().insert("anders", false);
    proc_names_map.lock().unwrap().insert("maczek", false);
    proc_names_map.lock().unwrap().insert("sosabowski", false);
    proc_names_map.lock().unwrap().insert("crerar", false);
    proc_names_map.lock().unwrap().insert("simonds", false);
    proc_names_map.lock().unwrap().insert("mcnaughton", false);
    proc_names_map.lock().unwrap().insert("blamey", false);
    proc_names_map.lock().unwrap().insert("sturdee", false);
    proc_names_map.lock().unwrap().insert("morshead", false);
    proc_names_map.lock().unwrap().insert("freyberg", false);
    proc_names_map.lock().unwrap().insert("fleischer", false);
    proc_names_map.lock().unwrap().insert("giraud", false);
    proc_names_map.lock().unwrap().insert("george", false);
    proc_names_map.lock().unwrap().insert("mcauliffe", false);
    proc_names_map.lock().unwrap().insert("lemay", false);
    proc_names_map
});

pub fn draw_name() -> String {
    let mut rng = rng();

    let proc_names_map = PROC_NAMES_MAP.lock().unwrap();
    let filtered_keys: Vec<&str> = proc_names_map
        .iter()
        .filter_map(|(key, &value)| if !value { Some(*key) } else { None })
        .collect();

    filtered_keys.choose(&mut rng).map(|&key| key.to_string()).unwrap_or_default()
}

pub fn release_name(name: String) {
    let mut proc_names_map = PROC_NAMES_MAP.lock().unwrap();
    if let Some(value) = proc_names_map.get_mut(name.as_str()) {
        *value = false;  // Update the value to false
    }
}
