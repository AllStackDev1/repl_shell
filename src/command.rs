use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum Command {
    Exit,
    Echo(String),
    Ls,
    Pwd,
    Cd(String),
    Touch(String),
    Rm(String),
    Cat(String),
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value: Vec<&str> = value.split_whitespace().collect();

        match split_value.as_slice() {
            ["exit"] => Ok(Command::Exit),
            ["ls"] => Ok(Command::Ls),
            ["echo", rest @ ..] => {
                if rest.is_empty() {
                    Err(anyhow!("echo command requires an argument"))
                } else {
                    Ok(Command::Echo(rest.join(" ")))
                }
            }
            ["pwd"] => Ok(Command::Pwd),
            ["cd", path] => {
                if path.is_empty() {
                    Err(anyhow!("cd command requires an argument"))
                } else {
                    Ok(Command::Cd(path.to_string()))
                }
            }
            ["touch", filename] => {
                if filename.is_empty() {
                    Err(anyhow!("touch command requires an argument"))
                } else {
                    Ok(Command::Touch(filename.to_string()))
                }
            }
            ["rm", filename] => {
                if filename.is_empty() {
                    Err(anyhow!("rm command requires an argument"))
                } else {
                    Ok(Command::Rm(filename.to_string()))
                }
            }
            ["cat", filename] => {
                if filename.is_empty() {
                    Err(anyhow!("cat command requires an argument"))
                } else {
                    Ok(Command::Cat(filename.to_string()))
                }
            }
            _ => Err(anyhow!("Invalid command")),
        }
    }
}
