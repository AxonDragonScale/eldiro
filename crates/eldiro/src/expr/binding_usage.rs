use super::FuncCall;

use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        // First try to get binding with given name, if there is none
        // then it may be a func call without parameters
        env.get_binding(&self.name).or_else(|err_msg| {
            if env.get_func(&self.name).is_ok() {
                FuncCall {
                    callee: self.name.clone(),
                    params: Vec::new(),
                }
                .eval(env)
            } else {
                Err(err_msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&env),
            Ok(Val::Number(10))
        );
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string()
            }
            .eval(&empty_env),
            Err("Binding with name `i_dont_exist` does not exist".to_string())
        );
    }
}
