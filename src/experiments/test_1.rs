#![allow(unused)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BaseWidgetVT<D> {
   pub on_click: fn(&mut D),
}

pub struct BaseWidget<D> {
   pub derive: D,
   vtable: BaseWidgetVT<Self>,
}

impl<D> BaseWidget<D> {
   pub fn new(derive: D, cb: impl Fn(&mut BaseWidgetVT<Self>)) -> Rc<RefCell<Self>> {
      let mut out = Self { derive, vtable: BaseWidgetVT { on_click: Self::on_click } };
      cb(&mut out.vtable);
      let out = Rc::new(RefCell::new(out));
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

pub struct AbstractButton<D> {
   pub derive: D,
   self_ref: Option<Weak<RefCell<BaseWidget<Self>>>>,
   vtable: AbstractButtonVT<BaseWidget<Self>>,
}

impl<D> AbstractButton<D> {
   pub fn new(derive: D) -> Rc<RefCell<BaseWidget<Self>>> {
      let s = Self { derive, self_ref: None, vtable: AbstractButtonVT { on_down: Self::on_down } };

      let mut out = BaseWidget::new(s, |vt| {
         vt.on_click = Self::on_click;
      });

      let self_ref = Rc::downgrade(&out);
      out.borrow_mut().derive.self_ref = Some(self_ref);

      out
   }

   fn on_click(b: &mut BaseWidget<Self>) {
      println!("[{}] on_click", std::any::type_name::<Self>());
      Self::on_down(b);
   }

   fn on_down(_b: &mut BaseWidget<Self>) {
      println!("[{}] on_down", std::any::type_name::<Self>());
   }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// pub struct PushButton {
//    self_ref: Option<Weak<RefCell<BaseWidget<Self>>>>,
// }
//
// impl PushButton {
//    pub fn new() -> Rc<RefCell<BaseWidget<Self>>> {
//       BaseWidget::new(AbstractButton::new(Self { self_ref: None }))
//    }
//
//    fn on_test(_b: &mut Base<Self>) {
//       println!("gav gav")
//    }
// }
//
// impl SetupDerive<Base<Self>> for PushButton {
//    fn setup_derive(&mut self, base: Weak<RefCell<Base<Self>>>) {
//       self.self_ref = Some(base);
//       if let Some(s) = self.self_ref.upgrade() {
//          let mut b = s.borrow_mut();
//          b.derive.vtable.on_test2 = Self::on_test;
//       }
//    }
// }
//
// impl SetupDerive<DeriveGav<Self>> for PushButton {
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
         let b = BaseWidget::new((), |_| {});
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
