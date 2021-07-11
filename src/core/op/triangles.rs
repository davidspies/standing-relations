mod bimap;
mod intersection;

use self::bimap::BiMap;
use crate::{
    core::{op::triangles::intersection::Intersectable, CountMap, Op, Op_},
    Relation,
};
use std::hash::Hash;

pub struct Triangles<X, Y, Z, C1: Op<D = (X, Y)>, C2: Op<D = (X, Z)>, C3: Op<D = (Y, Z)>> {
    c1: C1,
    c2: C2,
    c3: C3,
    mapxy: BiMap<X, Y>,
    mapxz: BiMap<X, Z>,
    mapyz: BiMap<Y, Z>,
}

impl<
        X: Eq + Hash + Clone,
        Y: Eq + Hash + Clone,
        Z: Eq + Hash + Clone,
        C1: Op<D = (X, Y)>,
        C2: Op<D = (X, Z)>,
        C3: Op<D = (Y, Z)>,
    > Op_ for Triangles<X, Y, Z, C1, C2, C3>
{
    type T = ((X, Y, Z), isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let mapxy = &mut self.mapxy;
        let mapxz = &mut self.mapxz;
        let mapyz = &mut self.mapyz;
        self.c1.foreach(|((x, y), count)| {
            let xzs = mapxz.get_forward(&x);
            let yzs = mapyz.get_forward(&y);
            for (z, lcount, rcount) in xzs.intersection(&yzs) {
                continuation(((x.clone(), y.clone(), z.clone()), count * lcount * rcount))
            }
            mapxy.add((x, y), count);
        });
        self.c2.foreach(|((x, z), count)| {
            let xys = mapxy.get_forward(&x);
            let zys = mapyz.get_backward(&z);
            for (y, lcount, rcount) in xys.intersection(&zys) {
                continuation(((x.clone(), y.clone(), z.clone()), count * lcount * rcount))
            }
            mapxz.add((x, z), count);
        });
        self.c3.foreach(|((y, z), count)| {
            let yxs = mapxy.get_backward(&y);
            let zxs = mapxz.get_backward(&z);
            for (x, lcount, rcount) in yxs.intersection(&zxs) {
                continuation(((x.clone(), y.clone(), z.clone()), count * lcount * rcount))
            }
            mapyz.add((y, z), count);
        });
    }
}

impl<X: Clone + Eq + Hash, Y: Clone + Eq + Hash, C1: Op<D = (X, Y)>> Relation<C1> {
    pub fn triangles<Z: Clone + Eq + Hash, C2: Op<D = (X, Z)>, C3: Op<D = (Y, Z)>>(
        self,
        rel2: Relation<C2>,
        rel3: Relation<C3>,
    ) -> Relation<Triangles<X, Y, Z, C1, C2, C3>> {
        assert_eq!(self.context_id, rel2.context_id, "Context mismatch");
        assert_eq!(self.context_id, rel3.context_id, "Context mismatch");
        Relation {
            context_id: self.context_id,
            dirty: self.dirty.or(rel2.dirty).or(rel3.dirty),
            inner: Triangles {
                c1: self.inner,
                c2: rel2.inner,
                c3: rel3.inner,
                mapxy: BiMap::new(),
                mapxz: BiMap::new(),
                mapyz: BiMap::new(),
            },
        }
    }
}
