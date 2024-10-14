use serde::Deserialize;
use serde::Serialize;
use std::process::Command;

fn command(revset: &str, template: &str) -> Result<Vec<u8>, String> {
    let result = Command::new("sl")
        .arg("log")
        .arg("-r")
        .arg(revset)
        .arg("-T")
        .arg(template)
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn root() -> Result<String, String> {
    let result = Command::new("sl")
        .arg("root")
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(String::from_utf8(result.stdout)
            .map_err(|e| e.to_string())?
            .trim()
            .to_string())
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn add_stack_note<T>(node: &str, change: T) -> Result<(), String>
where
    T: Serialize,
{
    let result = Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.sl/store/git", root()?))
        .arg("notes")
        .arg("--ref")
        .arg("refs/notes/book-stack")
        .arg("add")
        .arg("-f")
        .arg(node)
        .arg("-m")
        .arg(serde_json::to_string(&change).map_err(|e| e.to_string())?)
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn notes(node: &str) -> Result<String, String> {
    let result = Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.sl/store/git", root()?))
        .arg("notes")
        .arg("--ref")
        .arg("refs/notes/book-stack")
        .arg("show")
        .arg(node)
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(String::from_utf8(result.stdout)
            .map_err(|e| e.to_string())?
            .trim()
            .to_string())
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn push(node: &str, bookmark: &str) -> Result<String, String> {
    let result = Command::new("sl")
        .arg("push")
        .arg("-f")
        .arg("-r")
        .arg(node)
        .arg("--to")
        .arg(format!("remote/{}", bookmark))
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(String::from_utf8(result.stdout)
            .map_err(|e| e.to_string())?
            .trim()
            .to_string())
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn config_value(key: &str) -> Result<String, String> {
    let result = Command::new("sl")
        .arg("config")
        .arg(key)
        .output()
        .expect("Failed to execute command");

    if result.status.success() {
        Ok(String::from_utf8(result.stdout)
            .map_err(|e| e.to_string())?
            .trim()
            .to_string())
    } else {
        Err(String::from_utf8(result.stderr).map_err(|e| e.to_string())?)
    }
}

pub fn sl<T>(revset: &str, template: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let result = command(revset, template)?;
    let de: T = serde_json::from_slice(&result).map_err(|e| e.to_string())?;

    Ok(de)
}

pub fn sl_list<T>(revset: &str, template: &str) -> Result<Vec<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    let mut output = Vec::new();
    let result = command(revset, &format!("{template}\n"))?;

    for line in String::from_utf8(result).unwrap().lines() {
        let json: T = match serde_json::from_str(line) {
            Ok(json) => json,
            Err(e) => return Err(e.to_string()),
        };

        output.push(json);
    }

    Ok(output)
}
