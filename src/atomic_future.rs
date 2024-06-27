use std::fmt::{Display, Formatter};
use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::futures::U32Future;

/// Atomic Output Struct that future will produce
#[derive(Debug, Clone)]
pub struct AtomicOutput {
    id: Arc<AtomicU32>,
}

impl Default for AtomicOutput {
    fn default() -> Self {
        AtomicOutput {
            id: Arc::new(AtomicU32::new(0)),
        }
    }
}

impl Display for AtomicOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.id.load(Ordering::SeqCst)).as_str())
    }
}

/// Atomic Future
#[derive(Debug, Clone)]
pub struct AtomicFuture {
    output: AtomicOutput,
}

impl Default for AtomicFuture {
    fn default() -> Self {
        AtomicFuture {
            output: AtomicOutput::default(),
        }
    }
}

impl Display for AtomicFuture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.output.id.load(Ordering::SeqCst)).as_str())
    }
}

impl U32Future for AtomicFuture {
    fn update_value(&self, val: NonZeroU32) {
        let _ = self
            .output
            .id
            .compare_exchange(0, val.get(), Ordering::SeqCst, Ordering::SeqCst);
    }
}

/// Actually implementing Future Trait
impl Future for AtomicFuture {
    type Output = AtomicOutput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.output.id.load(Ordering::SeqCst) == 0 {
            // it is important to call wake as this will ensure that future is polled at least once more
            // it is also important to clone the waker.
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            Poll::Ready(self.output.clone())
        }
    }
}
