mod bimap;
mod intersection;

use std::hash::Hash;

use crate::core::{
    op::triangles::intersection::Intersectable, relation::RelationInner, CountMap, Op, Op_,
    Relation,
};

use self::bimap::BiMap;

pub struct Triangles<X, Y, Z, C1: Op<D = (X, Y)>, C2: Op<D = (X, Z)>, C3: Op<D = (Y, Z)>> {
    c1: RelationInner<C1>,
    c2: RelationInner<C2>,
    c3: RelationInner<C3>,
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

    fn get_type_name() -> &'static str {
        "triangles"
    }
}

impl<X: Clone + Eq + Hash, Y: Clone + Eq + Hash, C1: Op<D = (X, Y)>> Relation<C1> {
    pub fn triangles<Z: Clone + Eq + Hash, C2: Op<D = (X, Z)>, C3: Op<D = (Y, Z)>>(
        self,
        rel2: Relation<C2>,
        rel3: Relation<C3>,
    ) -> Relation<Triangles<X, Y, Z, C1, C2, C3>> {
        assert_eq!(
            self.context_tracker, rel2.context_tracker,
            "Context mismatch"
        );
        assert_eq!(
            self.context_tracker, rel3.context_tracker,
            "Context mismatch"
        );
        self.context_tracker.add_relation(
            self.dirty.or(rel2.dirty).or(rel3.dirty),
            Triangles {
                c1: self.inner,
                c2: rel2.inner,
                c3: rel3.inner,
                mapxy: BiMap::new(),
                mapxz: BiMap::new(),
                mapyz: BiMap::new(),
            },
            vec![
                self.tracking_index,
                rel2.tracking_index,
                rel3.tracking_index,
            ],
        )
    }
}
