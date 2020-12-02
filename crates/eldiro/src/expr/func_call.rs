use super::Expr;
use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);

        let (s, params) =
            utils::sequence_req(Expr::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params: params,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();
        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_actual_params = self.params.len();
        if num_expected_params != num_actual_params {
            return Err(format!(
                "Expected {} parameters, got {}",
                num_expected_params, num_actual_params
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_binding(param_name, param_val);
        }

        body.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BindingUsage, Number, Op};
    use super::*;
    use crate::stmt::Stmt;

    #[test]
    fn parse_func_call_with_one_params() {
        assert_eq!(
            FuncCall::new("fac 10"),
            Ok((
                "",
                FuncCall {
                    callee: "fac".to_string(),
                    params: vec![Expr::Number(Number(10))]
                }
            ))
        );
    }

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();
        env.store_func(
            "id".to_string(),
            vec!["x".to_string()],
            Stmt::Expr(Expr::BindingUsage(BindingUsage {
                name: "x".to_string(),
            })),
        );

        assert_eq!(
            FuncCall {
                callee: "id".to_string(),
                params: vec![Expr::Number(Number(3))]
            }
            .eval(&env),
            Ok(Val::Number(3))
        );
    }

    #[test]
    fn eval_non_existing_func_call() {
        let env = Env::default();

        assert_eq!(
            FuncCall {
                callee: "i_dont_exist".to_string(),
                params: vec![Expr::Number(Number(1))]
            }
            .eval(&env),
            Err("Function with name `i_dont_exist` does not exist".to_string())
        );
    }

    #[test]
    fn eval_func_call_with_too_few_params() {
        let mut env = Env::default();
        env.store_func(
            "mul".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Op::Mul,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "mul".to_string(),
                params: vec![Expr::Number(Number(2))]
            }
            .eval(&env),
            Err("Expected 2 parameters, got 1".to_string())
        );
    }

    #[test]
    fn eval_func_call_with_too_many_params() {
        let mut env = Env::default();
        env.store_func(
            "square".to_string(),
            vec!["n".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                op: Op::Mul,
            }),
        );
        assert_eq!(
            FuncCall {
                callee: "square".to_string(),
                params: vec![Expr::Number(Number(4)), Expr::Number(Number(4))]
            }
            .eval(&env),
            Err("Expected 1 parameters, got 2".to_string())
        );
    }
}
