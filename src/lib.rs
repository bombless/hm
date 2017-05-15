#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Clone, Copy)]
struct TypeIdx(u32);
#[derive(Clone, Copy)]
struct ExprIdx(u32);

impl From<u32> for TypeIdx {
    fn from(x: u32) -> TypeIdx { TypeIdx(x) }
}

impl From<u32> for ExprIdx {
    fn from(x: u32) -> TypeIdx { ExprIdx(x) }
}

impl From TypeIdx for u32 {
    fn from(TypeIdx(x): TypeIdx) -> u32 { x }
}

impl From ExprIdx for u32 {
    fn from(ExprIdx(x): ExprIdx) -> u32 { x }
}

#[derive(Clone)]
enum Expr {
    Var(ExprIdx, String),
    App(ExprIdx, Box<Expr>, Box<Expr>),
    Abs(ExprIdx, (ExprIdx, String), Expr),
    Let(ExprIdx, (ExprIdx, String), Expr, Expr),
}

impl Expr {
    fn id(&self) -> ExprIdx {
        use Self::*;
        match self {
            Var(id, ..) |
            App(id, ..) |
            Abs(id, ..) |
            Let(id, ..) => id
        }
    }
}

#[derive(Clone)]
enum Type {
    Plain(TypeIdx),
    Apply(TypeIdx, Box<Type>, Box<Type>),
    Lambda(TypeIdx, Box<Type>, Box<Type>),
    Hole,
}

enum CompoundType {
    Apply(TypeIdx, TypeIdx),
    Lambda(TypeIdx, TypeIdx),    
}

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone)]
struct KnowledgeBase {
    knowledge: HashMap<ExprIdx, TypeIdx>,
    compound_types: RefCell<HashMap<CompoundType, TypeIdx>>,
    next_idx: RefCell<TypeIdx>,
}

impl KnowledgeBase {
    fn new() -> Self {
        KnowledgeBase {
            knowledge: HashMap::new(),
            compound_types: RefCell::new(HashMap::new()),
            next_idx: 0.into(),
        }
    }

    fn apply(&self, t1: TypeIdx, t2: TypeIdx) -> TypeIdx {
        use CompoundType::*;
        let app = Apply(t1, t2);
        if let Some(idx) = self.compound_types.get(&app) {
            return idx
        }
        let p = self.compound_types.get_mut().unwrap();
        let curr_idx = std::mem::replace(self.next_idx.get_mut().unwrap(), TypeIdx(new_idx + 1));
        p.insert(app, curr_idx).unwrap()
    }

    fn lambda(&self, arg: TypeIdx, expr: TypeIdx) -> TypeIdx {
        use CompoundType::*;
        let l = Lambda(arg, expr);
        if let Some(idx) = self.compound_types.get(&l) {
            return idx
        }
        let p = self.compound_types.get_mut().unwrap();
        let curr_idx = std::mem::replace(self.next_idx.get_mut().unwrap(), TypeIdx(new_idx + 1));
        p.insert(l, curr_idx).unwrap()
    }

    fn get(&self, eid: &TypeIdx) -> Option<TypeIdx> {
        self.knowledge.get(eid)
    }
}

trait Visitor {
    fn visit(&mut self, e: Expr);
}




fn resolve(e: Expr, k: &KnowledgeBase) -> Option<(ExprIdx, TypeIdx)> {
    use Expr::*;
    match e {
        Var(eid, _) => match k.get(eid).map(t => (eid, t)),
        App(eid, e1, e2) => match (k.get(&e1.id()), k.get(&e2.id())) {
            (Some(tid1), Some(tid2)) => Some((eid, k.apply(tid1, tid2))),
            _ => None,
        },
        Abs(eid, (arg, _), expr) => Some((eid, k.lambda(k.get(&arg), k.get(&expr)))),
        Let(eid, (var, _), val, env) => 
    }
}