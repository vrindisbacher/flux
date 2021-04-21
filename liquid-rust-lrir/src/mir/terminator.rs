use crate::{
    mir::{BasicBlock, Operand, Place, Span},
    ty::BaseTy,
};

pub struct Terminator {
    pub kind: TerminatorKind,
    pub span: Span,
}

pub enum TerminatorKind {
    Goto {
        target: BasicBlock,
    },
    SwitchInt {
        discr: Operand,
        switch_ty: BaseTy,
        targets: SwitchTargets,
    },
    Return,
    Call {
        func: Operand,
        args: Vec<Operand>,
        destination: (Place, BasicBlock),
    },
    Assert {
        cond: Operand,
        expected: bool,
        target: BasicBlock,
    },
}

pub struct SwitchTargets {
    values: Vec<u128>,
    targets: Vec<BasicBlock>,
}

impl SwitchTargets {
    pub fn new(targets: impl Iterator<Item = (u128, BasicBlock)>, otherwise: BasicBlock) -> Self {
        let (values, mut targets) = targets.unzip::<u128, BasicBlock, Vec<_>, Vec<_>>();
        targets.push(otherwise);

        Self { values, targets }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u128, BasicBlock)> + '_ {
        self.values
            .iter()
            .zip(&self.targets)
            .map(|(bits, target)| (*bits, *target))
    }

    pub fn otherwise(&self) -> BasicBlock {
        *self.targets.last().unwrap()
    }
}
