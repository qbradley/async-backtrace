use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::future::FusedFuture;
use std::marker::PhantomPinned;

use crate::frame::Frame;
use crate::location::Location;

use pin_project_lite::pin_project;

pin_project! {
    /// A future whose [`Location`] is included in [taskdumps][crate::tasks] and [backtraces][crate::backtrace].
    pub struct Framed<F> {
        // The wrapped future.
        #[pin]
        future: F,
        // Metadata about the wrapped future.
        #[pin]
        frame: Frame,
        _pinned: PhantomPinned,
    }
}

impl<F: core::panic::UnwindSafe> core::panic::UnwindSafe for Framed<F> {}

impl<F> Framed<F> {
    /// Include the given `future` in taskdumps and
    /// backtraces with the given `location`.
    pub fn new(future: F, location: Location) -> Self {
        Self {
            future,
            frame: Frame::new(location),
            _pinned: PhantomPinned,
        }
    }
}

impl<F> Future for Framed<F>
where
    F: Future,
{
    type Output = <F as Future>::Output;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        let this = self.project();
        let frame = this.frame;
        let future = this.future;
        frame.in_scope(|| future.poll(cx))
    }
}

impl<F: FusedFuture> FusedFuture for Framed<F> {
    fn is_terminated(&self) -> bool {
        self.future.is_terminated()
    }
}
