use crate::env::Env;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::new(s) {
            let (new_s, _) = utils::extract_whitespace(new_s);
            s = new_s;
            stmts.push(stmt);
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block { stmts: stmts }))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }

        let mut env = env.create_child();
        let stmts_except_last = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut env)?;
        }

        // we can unwrap this safely, coz we already checked if stmts is empty
        self.stmts.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{BindingUsage, Expr, Number, Op};
    use super::*;
    use crate::binding_def::BindingDef;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{ }"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]
                }
            ))
        );
    }

    #[test]
    fn parse_block_with_multiple_stmts() {
        assert_eq!(
            Block::new(
                "{
                let a = 20
                let b = a
                b
            }"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(20))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        }))
                    ]
                }
            ))
        );
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: Vec::new() }.eval(&Env::default()),
            Ok(Val::Unit)
        );
    }

    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(25)))]
            }
            .eval(&Env::default()),
            Ok(Val::Number(25))
        );
    }

    #[test]
    fn eval_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "one".to_string(),
                        val: Expr::Number(Number(1))
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "one".to_string()
                    }))
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(1))
        );
    }

    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::Number(Number(1))
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::Number(Number(2))
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        val: Expr::Number(Number(3))
                    })
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Unit)
        );
    }

    #[test]
    fn eval_block_with_multiple_exprs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Expr(Expr::Number(Number(1))),
                    Stmt::Expr(Expr::Number(Number(2))),
                    Stmt::Expr(Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(7),
                        op: Op::Sub,
                    })
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(3))
        );
    }

    #[test]
    fn eval_block_using_bindings_from_parent_env() {
        // This is what we are trying to acheive
        // let foo = 10
        // let bar = {
        //     let baz = foo
        //     baz
        // }
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        val: Expr::BindingUsage(BindingUsage {
                            name: "foo".to_string()
                        })
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "baz".to_string()
                    }))
                ]
            }
            .eval(&env),
            Ok(Val::Number(10))
        );
    }
}