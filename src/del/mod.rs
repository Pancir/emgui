use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SetupDerive<REF> {
   fn set_self_ref(&mut self, r: Weak<RefCell<REF>>);
   fn setup_derive(&mut self, vt: &mut REF);
}

impl<REF> SetupDerive<REF> for () {
   fn set_self_ref(&mut self, _vt: Weak<RefCell<REF>>) {}
   fn setup_derive(&mut self, _vt: &mut REF) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BaseVTable<D> {
   pub on_test: fn(&mut D),
}

pub struct Base<D>
where
   D: SetupDerive<Self>,
{
   pub derive: D,
   vtable: BaseVTable<Self>,
}

impl<D> Base<D>
where
   D: SetupDerive<Self>,
{
   pub fn new(derive: D) -> Rc<RefCell<Self>> {
      let mut out = Self { derive, vtable: BaseVTable { on_test: Self::on_test } };
      let force = &mut out as *mut Self;

      unsafe { (*force).derive.setup_derive(&mut out) };

      let out = Rc::new(RefCell::new(out));
      let w = Rc::downgrade(&out);
      {
         let mut r2 = out.borrow_mut();
         r2.derive.set_self_ref(w);
      }

      out
   }

   pub fn run_test(&mut self) {
      (self.vtable.on_test)(self)
   }

   fn on_test(_b: &mut Self) {
      println!("b")
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DeriveGavVTable<D> {
   pub on_test2: fn(&mut D),
}

pub struct DeriveGav<D>
where
   D: SetupDerive<Self>,
{
   pub derive: D,
   self_ref: Option<Weak<RefCell<Base<Self>>>>,
   vtable: DeriveGavVTable<Self>,
}

impl<D> DeriveGav<D>
where
   D: SetupDerive<Self>,
{
   pub fn new(derive: D) -> Rc<RefCell<Base<Self>>> {
      Base::new(Self {
         derive,
         self_ref: None,
         vtable: DeriveGavVTable { on_test2: Self::on_test2 },
      })
   }

   fn on_test(_b: &mut Base<Self>) {
      println!("gav")
   }

   fn on_test2(_b: &mut Self) {
      println!("gav")
   }
}

impl<D> SetupDerive<Base<Self>> for DeriveGav<D>
where
   D: SetupDerive<Self>,
{
   fn set_self_ref(&mut self, base: Weak<RefCell<Base<Self>>>) {
      self.self_ref = Some(base);
   }

   fn setup_derive(&mut self, vt: &mut Base<Self>) {
      vt.vtable.on_test = Self::on_test;
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct DeriveGavGav {
//    self_ref: Option<Weak<RefCell<Base<Self>>>>,
// }
//
// impl DeriveGavGav {
//    pub fn new() -> Rc<RefCell<Base<Self>>> {
//       Base::new(DeriveGav::new(Self { self_ref: None }))
//    }
//
//    fn on_test(_b: &mut Base<Self>) {
//       println!("gav gav")
//    }
// }
//
// impl SetupDerive<Base<Self>> for DeriveGavGav {
//    fn setup_derive(&mut self, base: Weak<RefCell<Base<Self>>>) {
//       self.self_ref = Some(base);
//       if let Some(s) = self.self_ref.upgrade() {
//          let mut b = s.borrow_mut();
//          b.derive.vtable.on_test2 = Self::on_test;
//       }
//    }
// }
//
// impl SetupDerive<DeriveGav<Self>> for DeriveGavGav {
//    fn setup_derive(&mut self, base: Weak<RefCell<DeriveGav<Self>>>) {
//       self.self_ref = Some(base);
//       if let Some(s) = self.self_ref.upgrade() {
//          let mut b = s.borrow_mut();
//          b.derive.vtable.on_test2 = Self::on_test;
//       }
//    }
// }

/////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test() {
      {
         let b = Base::<()>::new(());
         let mut bor = b.borrow_mut();
         bor.run_test();
      }

      {
         let b = DeriveGav::<()>::new(());
         let mut bor = b.borrow_mut();
         bor.run_test();
      }
      //
      // {
      //    let b = DeriveGavGav::<()>::new(());
      //    let mut bor = b.borrow_mut();
      //    bor.run_test();
      // }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
