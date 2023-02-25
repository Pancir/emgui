use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SetupDerive<REF> {
   fn setup_derive(&mut self, vt: Weak<RefCell<REF>>);
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
      let out =
         Rc::new(RefCell::new(Self { derive, vtable: BaseVTable { on_test: Self::on_test } }));
      let w = Rc::downgrade(&out);
      match out.try_borrow_mut() {
         Ok(mut r) => r.derive.setup_derive(w),
         Err(_) => {
            unreachable!()
         }
      }
      out
   }

   fn on_test(_b: &mut Self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DeriveGav {
   self_ref: Weak<RefCell<Base<Self>>>,
}

impl DeriveGav {
   fn on_test(_b: &mut Base<Self>) {}
}

impl SetupDerive<Base<Self>> for DeriveGav {
   fn setup_derive(&mut self, base: Weak<RefCell<Base<Self>>>) {
      self.self_ref = base;
      if let Some(s) = self.self_ref.upgrade() {
         let mut b = s.borrow_mut();
         b.vtable.on_test = Self::on_test;
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DeriveGavGav {
   self_ref: Weak<RefCell<Base<Self>>>,
}

impl DeriveGavGav {
   fn on_test(_b: &mut Base<Self>) {}
}

impl SetupDerive<Base<Self>> for DeriveGavGav {
   fn setup_derive(&mut self, base: Weak<RefCell<Base<Self>>>) {
      self.self_ref = base;
      if let Some(s) = self.self_ref.upgrade() {
         let mut b = s.borrow_mut();
         b.vtable.on_test = Self::on_test;
      }
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
