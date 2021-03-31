use std::fs;
use std::path::PathBuf;
use std::{borrow::Borrow, env};

pub fn read_env_dir_or_fallback_to_etc<T>(
    env_var: &str,
    dir: &str,
    filter: Option<T>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>>
where
    T: Fn(fs::ReadDir) -> Vec<PathBuf>,
{
    let mut dir_content_wide: Option<Vec<PathBuf>> = None;
    let found: bool = match env::var(env_var) {
        Ok(x) => {
            // Use closure to return booleans that will then be returned by match
            // Normally, return statement inside match referes to a whole function
            let closure = || {
                let mut env_path = PathBuf::from(x);
                env_path.push(dir);
                if !env_path.exists() {
                    return false;
                };
                debug!("Looking up {:?}", env_path);
                let dir_content_all = fs::read_dir(env_path).unwrap();
                let dir_content = match filter.borrow() {
                    Some(f) => f(dir_content_all),
                    None => read_dir_to_pathbuf(dir_content_all),
                };
                if dir_content.len() == 0 {
                    debug!("No yaml files found!");
                    return false;
                }
                dir_content_wide = Some(dir_content);
                true
            };
            closure()
        }
        _ => false,
    };
    if !found {
        debug!("Looking up /etc/ontime/");
        let etc_path = PathBuf::from("/etc/ontime/");
        if !etc_path.exists() {
            return Err("No configuration folder, make sure you have ontime/ directory in either /etc/ or XDG_CONFIG_HOME".into());
        }
        let dir_content_all = fs::read_dir(&etc_path)?;
        let dir_content = match filter {
            Some(f) => f(dir_content_all),
            None => read_dir_to_pathbuf(dir_content_all),
        };
        if dir_content.len() == 0 {
            return Err("No yaml files found in /etc/ontime".into());
        }
        dir_content_wide = Some(dir_content);
    }
    Ok(dir_content_wide.unwrap())
}
fn read_dir_to_pathbuf(dir_content: fs::ReadDir) -> Vec<PathBuf> {
    dir_content
        .into_iter()
        .map(|item| item.unwrap().path())
        .collect()
}
pub fn find_env_dir_or_etc(
    env_var: &str,
    dir: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    let found: bool = match env::var(env_var) {
        Ok(x) => {
            path.push(x);
            path.push(dir);
            path.exists()
        }
        _ => false,
    };
    if !found {
        path = PathBuf::from("/etc/");
        path.push(dir);
    }
    Ok(path)
}
pub fn filter_dir_content(content: fs::ReadDir) -> Vec<PathBuf> {
    return content
        .into_iter()
        .filter(|element_res| {
            return match element_res {
                Ok(element) => {
                    let file_name_osstr = element.file_name();
                    let file_name = file_name_osstr.to_str().unwrap();
                    element.file_type().unwrap().is_file()
                        && (file_name.ends_with(".yml") || file_name.ends_with(".yaml"))
                }
                _ => false,
            };
        })
        .map(|element| element.unwrap().path())
        .collect();
}
