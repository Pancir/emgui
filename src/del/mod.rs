use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SetupDerive<REF, VT> {
   fn set_self_ref(&mut self, r: Weak<RefCell<REF>>);
   fn setup_vt(&mut self, vt: &mut VT);
}

impl<REF, VT> SetupDerive<REF, VT> for () {
   fn set_self_ref(&mut self, _vt: Weak<RefCell<REF>>) {}
   fn setup_vt(&mut self, _vt: &mut VT) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BaseWidgetVT<D> {
   pub on_click: fn(&mut D),
}

pub struct BaseWidget<D>
where
   D: SetupDerive<Self, BaseWidgetVT<Self>>,
{
   pub derive: D,
   vtable: BaseWidgetVT<Self>,
}

impl<D> BaseWidget<D>
where
   D: SetupDerive<Self, BaseWidgetVT<Self>>,
{
   pub fn new(derive: D) -> Rc<RefCell<Self>> {
      let mut out = Self { derive, vtable: BaseWidgetVT { on_click: Self::on_click } };
      out.derive.setup_vt(&mut out.vtable);

      let out = Rc::new(RefCell::new(out));
      let w = Rc::downgrade(&out);
      {
         let mut r2 = out.borrow_mut();
         r2.derive.set_self_ref(w);
      }

      out
   }

   pub fn emit_on_click(&mut self) {
      (self.vtable.on_click)(self)
   }

   fn on_click(_b: &mut Self) {
      println!("[{}] on_click", std::any::type_name::<Self>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct AbstractButtonVT<D> {
   pub on_down: fn(&mut D),
}

pub struct AbstractButton<D>
where
   D: SetupDerive<BaseWidget<Self>, AbstractButtonVT<Self>>,
{
   pub derive: D,
   self_ref: Option<Weak<RefCell<BaseWidget<Self>>>>,
   vtable: AbstractButtonVT<Self>,
}

impl<D> AbstractButton<D>
where
   D: SetupDerive<BaseWidget<Self>, AbstractButtonVT<Self>>,
{
   pub fn new(derive: D) -> Rc<RefCell<BaseWidget<Self>>> {
      BaseWidget::new(Self {
         derive,
         self_ref: None,
         vtable: AbstractButtonVT { on_down: Self::on_down },
      })
   }

   fn on_click(b: &mut BaseWidget<Self>) {
      println!("[{}] on_click", std::any::type_name::<Self>());
      Self::on_down(&mut b.derive);
   }

   fn on_down(_b: &mut Self) {
      println!("[{}] on_down", std::any::type_name::<Self>());
   }
}

impl<D> SetupDerive<BaseWidget<Self>, BaseWidgetVT<BaseWidget<Self>>> for AbstractButton<D>
where
   D: SetupDerive<BaseWidget<Self>, AbstractButtonVT<Self>>,
{
   fn set_self_ref(&mut self, base: Weak<RefCell<BaseWidget<Self>>>) {
      self.self_ref = Some(base);
   }

   fn setup_vt(&mut self, vt: &mut BaseWidgetVT<BaseWidget<Self>>) {
      vt.on_click = Self::on_click;
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
         let b = BaseWidget::new(());
         let mut bor = b.borrow_mut();
         bor.emit_on_click();
      }

      {
         let b = AbstractButton::new(());
         let mut bor = b.borrow_mut();
         bor.emit_on_click();
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
