use std::collections::HashMap;
use crate::ast::{BinaryOp, Expr, Stmt, Value};

// Ограничение итераций для защиты от бесконечного цикла while (п. 4 задания)
const MAX_ITERATIONS: usize = 10_000;

pub struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    // Выполняет всю программу (список инструкций)
    pub fn eval_program(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    // Выполнение одной инструкции
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                if !self.variables.contains_key(name) {
                    return Err(format!(
                        "Ошибка выполнения: попытка изменить необъявленную переменную '{}'",
                        name
                    ));
                }
                let val = self.eval_expr(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(condition)?;
                match cond_val {
                    Value::Bool(true) => self.eval_program(then_branch),
                    Value::Bool(false) => {
                        if let Some(else_stmts) = else_branch {
                            self.eval_program(else_stmts)
                        } else {
                            Ok(())
                        }
                    }
                    Value::Number(n) => Err(format!(
                        "Ошибка типа: условие в 'if' должно быть логическим (true/false), но получено число '{}'",
                        n
                    )),
                }
            }
            Stmt::While { condition, body } => {
                let mut iterations = 0;
                loop {
                    iterations += 1;
                    if iterations > MAX_ITERATIONS {
                        return Err(format!(
                            "Ошибка выполнения: превышен лимит итераций цикла ({}) — возможен бесконечный цикл",
                            MAX_ITERATIONS
                        ));
                    }

                    let cond_val = self.eval_expr(condition)?;
                    match cond_val {
                        Value::Bool(true) => self.eval_program(body)?,
                        Value::Bool(false) => break,
                        Value::Number(n) => {
                            return Err(format!(
                                "Ошибка типа: условие в 'while' должно быть логическим (true/false), но получено число '{}'",
                                n
                            ))
                        }
                    }
                }
                Ok(())
            }
        }
    }

    // Вычисление выражения
    fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Variable(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Ошибка выполнения: переменная '{}' не найдена", name)),
            Expr::UnaryMinus(inner) => match self.eval_expr(inner)? {
                Value::Number(n) => Ok(Value::Number(-n)),
                Value::Bool(_) => {
                    Err("Ошибка типа: унарный минус нельзя применить к логическому значению".to_string())
                }
            },
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary(left_val, op, right_val)
            }
        }
    }

    // Вычисление бинарных операций (математика и сравнения)
    fn eval_binary(&self, left: Value, op: &BinaryOp, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => match op {
                BinaryOp::Add => Ok(Value::Number(a + b)),
                BinaryOp::Sub => Ok(Value::Number(a - b)),
                BinaryOp::Mul => Ok(Value::Number(a * b)),
                BinaryOp::Div => {
                    if b == 0.0 {
                        Err("Ошибка выполнения: деление на ноль".to_string())
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                BinaryOp::Mod => {
                    if b == 0.0 {
                        Err("Ошибка выполнения: остаток от деления на ноль".to_string())
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                BinaryOp::Eq => Ok(Value::Bool((a - b).abs() < f64::EPSILON)),
                BinaryOp::NotEq => Ok(Value::Bool((a - b).abs() >= f64::EPSILON)),
                BinaryOp::Gt => Ok(Value::Bool(a > b)),
                BinaryOp::Lt => Ok(Value::Bool(a < b)),
                BinaryOp::GtEq => Ok(Value::Bool(a >= b)),
                BinaryOp::LtEq => Ok(Value::Bool(a <= b)),
            },
            (Value::Bool(a), Value::Bool(b)) => match op {
                BinaryOp::Eq => Ok(Value::Bool(a == b)),
                BinaryOp::NotEq => Ok(Value::Bool(a != b)),
                _ => Err("Ошибка типа: к логическим значениям применимы только '==' и '!='".to_string()),
            },
            (l, r) => Err(format!(
                "Ошибка типа: нельзя применить операцию {:?} к разным типам ({:?} и {:?})",
                op, l, r
            )),
        }
    }
}
