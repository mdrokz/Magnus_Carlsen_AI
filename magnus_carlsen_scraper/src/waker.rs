use std::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::Deref,
    sync::{Arc},
    task::{RawWaker, RawWakerVTable, Wake, Waker},
};


#[derive(Debug)]
pub struct WakerRef<'a> {
    waker: ManuallyDrop<Waker>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> WakerRef<'a> {
    /// Create a new [`WakerRef`] from a [`Waker`] reference.
    pub fn new(waker: &'a Waker) -> Self {
        // copy the underlying (raw) waker without calling a clone,
        // as we won't call Waker::drop either.
        let waker = ManuallyDrop::new(unsafe { core::ptr::read(waker) });
        Self {
            waker,
            _marker: PhantomData,
        }
    }

    /// Create a new [`WakerRef`] from a [`Waker`] that must not be dropped.
    ///
    /// Note: this if for rare cases where the caller created a [`Waker`] in
    /// an unsafe way (that will be valid only for a lifetime to be determined
    /// by the caller), and the [`Waker`] doesn't need to or must not be
    /// destroyed.
    pub fn new_unowned(waker: ManuallyDrop<Waker>) -> Self {
        Self {
            waker,
            _marker: PhantomData,
        }
    }
}

impl Deref for WakerRef<'_> {
    type Target = Waker;

    fn deref(&self) -> &Waker {
        &self.waker
    }
}

pub fn wake_ref<W>(wake: &Arc<W>) -> WakerRef<'_>
where
    W: Wake,
{
    let ptr = (&**wake as *const W) as *const ();

    let waker =
        ManuallyDrop::new(unsafe { Waker::from_raw(RawWaker::new(ptr, waker_vtable::<W>())) });

    WakerRef::new_unowned(waker)
}

fn waker_vtable<W: Wake>() -> &'static RawWakerVTable {
    &RawWakerVTable::new(
        clone_arc_raw::<W>,
        wake_arc_raw::<W>,
        wake_by_ref_arc_raw::<W>,
        drop_arc_raw::<W>,
    )
}

unsafe fn increase_refcount<T: Wake>(data: *const ()) {
    // Retain Arc, but don't touch refcount by wrapping in ManuallyDrop
    let arc = mem::ManuallyDrop::new(Arc::<T>::from_raw(data as *const T));
    // Now increase refcount, but don't drop new refcount either
    let _arc_clone: mem::ManuallyDrop<_> = arc.clone();
}

// used by `waker_ref`
unsafe fn clone_arc_raw<T: Wake>(data: *const ()) -> RawWaker {
    increase_refcount::<T>(data);
    RawWaker::new(data, waker_vtable::<T>())
}

unsafe fn wake_arc_raw<T: Wake>(data: *const ()) {
    let arc: Arc<T> = Arc::from_raw(data as *const T);
    Wake::wake(arc);
}

// used by `waker_ref`
unsafe fn wake_by_ref_arc_raw<T: Wake>(data: *const ()) {
    // Retain Arc, but don't touch refcount by wrapping in ManuallyDrop
    let arc = mem::ManuallyDrop::new(Arc::<T>::from_raw(data as *const T));
    Wake::wake_by_ref(&arc);
}

unsafe fn drop_arc_raw<T: Wake>(data: *const ()) {
    drop(Arc::<T>::from_raw(data as *const T))
}