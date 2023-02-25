////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SetupDerive<REF, VT> {
   fn setup_ref(&mut self, vt: REF);
   fn setup_vtable(&mut self, vt: &mut VT);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BaseVTable<D> {
   pub on_test: fn(&mut D),
}

pub struct Base<D>
where
   D: SetupDerive<Self, BaseVTable<Self>>,
{
   vtable: BaseVTable<Self>,
}

impl<D> Base<D>
where
   D: SetupDerive<Self, BaseVTable<Self>>,
{
   pub fn new() -> Self {
      Self { vtable: BaseVTable { on_test: Self::on_test } }
   }

   fn on_test(b: &mut Self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Derive {}

impl SetupDerive<Base<Self>, BaseVTable<Base<Self>>> for Derive {
   fn setup_ref(&mut self, vt: Base<Self>) {}
   fn setup_vtable(&mut self, vt: &mut BaseVTable<Base<Self>>) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
