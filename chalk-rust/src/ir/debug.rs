use std::fmt::{Debug, Formatter, Error};

use super::*;

impl Debug for ItemId {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        with_current_program(|prog| {
            match prog.and_then(|p| p.type_kinds.get(self)) {
                Some(k) => write!(fmt, "{}", k.name),
                None => fmt.debug_struct("ItemId").field("index", &self.index).finish(),
            }
        })
    }
}

impl Debug for UniverseIndex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "U{}", self.counter)
    }
}

impl Debug for TypeName {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeName::ItemId(id) => write!(fmt, "{:?}", id),
            TypeName::ForAll(universe) => write!(fmt, "!{}", universe.counter),
            TypeName::AssociatedType(assoc_ty) => write!(fmt, "{:?}", assoc_ty),
        }
    }
}

impl Debug for AssociatedType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({:?}::{})", self.trait_id, self.name)
    }
}

impl<T: Debug, L: Debug> Debug for ParameterKind<T, L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            ParameterKind::Ty(ref n) => write!(fmt, "{:?}", n),
            ParameterKind::Lifetime(ref n) => write!(fmt, "{:?}", n),
        }
    }
}

impl Debug for Ty {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Ty::Var(depth) => write!(fmt, "?{}", depth),
            Ty::Apply(ref apply) => write!(fmt, "{:?}", apply),
            Ty::Projection(ref proj) => write!(fmt, "{:?}", proj),
            Ty::ForAll(ref quantified_ty) => write!(fmt, "{:?}", quantified_ty),
        }
    }
}

impl Debug for QuantifiedTy {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        // FIXME -- we should introduce some names or something here
        let QuantifiedTy { num_binders, ref ty } = *self;
        write!(fmt, "for<{}> {:?}", num_binders, ty)
    }
}

impl Debug for Lifetime {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Lifetime::Var(depth) => write!(fmt, "'?{}", depth),
            Lifetime::ForAll(universe) => write!(fmt, "'!{}", universe.counter),
        }
    }
}

impl Debug for ApplicationTy {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}{:?}", self.name, Angle(&self.parameters))
    }
}

impl Debug for TraitRef {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt,
               "{:?} as {:?}{:?}",
               self.parameters[0],
               self.trait_id,
               Angle(&self.parameters[1..]))
    }
}

impl Debug for ProjectionTy {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "<{:?}>::{}", self.trait_ref, self.name)
    }
}

pub struct Angle<'a, T: 'a>(pub &'a [T]);

impl<'a, T: Debug> Debug for Angle<'a, T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.0.len() > 0 {
            write!(fmt, "<")?;
            for (index, elem) in self.0.iter().enumerate() {
                if index > 0 {
                    write!(fmt, ", {:?}", elem)?;
                } else {
                    write!(fmt, "{:?}", elem)?;
                }
            }
            write!(fmt, ">")?;
        }
        Ok(())
    }
}

struct Assignment<'a>(Identifier, &'a Ty);

impl<'a> Debug for Assignment<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} = {:?}", self.0, self.1)
    }
}

impl Debug for Normalize {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let assign: &Debug = &Assignment(self.projection.name, &self.ty);
        let args: Vec<_> = self.projection
            .trait_ref
            .parameters
            .iter()
            .skip(1)
            .map(|n| n as &Debug)
            .chain(Some(assign))
            .collect();
        write!(fmt,
               "{:?}: {:?}{:?}",
               self.projection.trait_ref.parameters[0],
               self.projection.trait_ref.trait_id,
               Angle(&args))
    }
}

impl Debug for WhereClause {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            WhereClause::Normalize(ref n) => write!(fmt, "{:?}", n),
            WhereClause::Implemented(ref n) => {
                write!(fmt,
                       "{:?}: {:?}{:?}",
                       n.parameters[0],
                       n.trait_id,
                       Angle(&n.parameters[1..]))
            }
        }
    }
}

impl Debug for WhereClauseGoal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            WhereClauseGoal::Normalize(ref n) => write!(fmt, "{:?}", n),
            WhereClauseGoal::Implemented(ref n) => {
                write!(fmt,
                       "{:?}: {:?}{:?}",
                       n.parameters[0],
                       n.trait_id,
                       Angle(&n.parameters[1..]))
            }
            WhereClauseGoal::UnifyTys(ref n) => write!(fmt, "{:?}", n),
        }
    }
}

impl<T: Debug> Debug for Unify<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({:?} = {:?})", self.a, self.b)
    }
}

impl Debug for Goal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Goal::Quantified(qkind, ParameterKind::Ty(()), ref g) =>
                write!(fmt, "{:?}<type> {{ {:?} }}", qkind, g),
            Goal::Quantified(qkind, ParameterKind::Lifetime(()), ref g) =>
                write!(fmt, "{:?}<type> {{ {:?} }}", qkind, g),
            Goal::Implies(ref wc, ref g) => write!(fmt, "if ({:?}) {{ {:?} }}", wc, g),
            Goal::And(ref g1, ref g2) => write!(fmt, "({:?}, {:?})", g1, g2),
            Goal::Leaf(ref wc) => write!(fmt, "{:?}", wc),
        }
    }
}
