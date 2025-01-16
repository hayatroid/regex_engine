use thiserror::Error;

use crate::helper::safe_add;

use super::{parser::AST, Instruction};

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("CodeGenError: PCOverFlow")]
    PCOverFlow,
    #[error("CodeGenError: FailStar")]
    FailStar,
    #[error("CodeGenError: FailOr")]
    FailOr,
    #[error("CodeGenError: FailQuestion")]
    FailQuestion,
}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

impl Generator {
    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;

        self.inc_pc()?;
        self.insts.push(Instruction::Match);

        Ok(())
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e) => self.gen_star(e)?,
            AST::Question(e) => self.gen_question(e)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }

        Ok(())
    }

    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        self.inc_pc()?;
        self.insts.push(Instruction::Char(c));

        Ok(())
    }

    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        //     split L1, L2
        // L1: e1 のコード
        //     jump L3
        // L2: e2 のコード
        // L3:

        let split_addr = self.pc;
        self.inc_pc()?;
        self.insts.push(Instruction::Split(self.pc, 0)); // とりあえず split L1, 0 とする

        self.gen_expr(e1)?;

        let jump_addr = self.pc;
        self.inc_pc()?;
        self.insts.push(Instruction::Jump(0)); // とりあえず jump 0 とする

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc; // split L1, L2 に改める
        } else {
            return Err(CodeGenError::FailOr);
        }

        self.gen_expr(e2)?;

        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jump_addr) {
            *l3 = self.pc; // jump L3 に改める
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: e のコード
        //     split L1, L2
        // L2:

        let l1 = self.pc;

        self.gen_expr(e)?;

        self.inc_pc()?;
        self.insts.push(Instruction::Split(l1, self.pc));

        Ok(())
    }

    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // L1: split L2, L3
        // L2: e のコード
        //     jump L1
        // L3:

        let l1 = self.pc;
        self.inc_pc()?;
        self.insts.push(Instruction::Split(self.pc, 0)); // とりあえず split L2, 0 とする

        self.gen_expr(e)?;

        self.inc_pc()?;
        self.insts.push(Instruction::Jump(l1));

        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc; // split L2, L3 に改める
        } else {
            return Err(CodeGenError::FailStar);
        }

        Ok(())
    }

    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        //     split L1, L2
        // L1: e のコード
        // L2:

        let split_addr = self.pc;
        self.inc_pc()?;
        self.insts.push(Instruction::Split(self.pc, 0)); // とりあえず split L1, 0 とする

        self.gen_expr(e)?;

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc; // split L1, L2 に改める
        } else {
            return Err(CodeGenError::FailQuestion);
        }

        Ok(())
    }

    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e)?;
        }

        Ok(())
    }
}
