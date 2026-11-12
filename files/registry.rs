//! Windows registry read helpers for Nemetschek Allplan.

pub const APP_KEY: &str = "Software\\Nemetschek\\Nemetschek Allplan";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Nemetschek Allplan";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Nemetschek Allplan") },
        RegValue { name: "Version",     value: String::from("v4.7.25") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Nemetschek Allplan") },
        RegValue { name: "Publisher",       value: String::from("Nemetschek") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Nemetschek Allplan\\uninstall.exe\"") },
    ]
}
