use std::fmt::{Display, Formatter};
use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use crate::futures::U32Future;

/// Sample Output Struct that future will produce
#[derive(Debug, Copy, Clone)]
pub struct SampleOutput {
    id: u32,
}

impl Default for SampleOutput {
    fn default() -> Self {
        SampleOutput { id: 0 }
    }
}

impl Display for SampleOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.id).as_str())
    }
}

/// Sample Future
#[derive(Debug, Clone)]
pub struct SampleFuture {
    output: Arc<Mutex<SampleOutput>>,
}

impl Default for SampleFuture {
    fn default() -> Self {
        SampleFuture {
            output: Arc::new(Mutex::new(SampleOutput::default())),
        }
    }
}

impl Display for SampleFuture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.output.lock().expect("failed to lock!!").id).as_str())
    }
}

impl U32Future for SampleFuture {
    fn update_value(&self, val: NonZeroU32) {
        self.output.lock().expect("failed to lock!!").id = val.get();
    }
}

/// Actually implementing Future Trait
impl Future for SampleFuture {
    type Output = SampleOutput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.output.lock().unwrap().id == 0 {
            // it is important to call wake as this will ensure that future is polled at least once more
            // it is also important to clone the waker.
            cx.waker().clone().wake();
            Poll::Pending
        } else {
            // Poll::Ready(SampleOutput {
            //     id: self.output.lock().unwrap().id,
            // })
            Poll::Ready(self.output.lock().unwrap().clone())
        }
    }
}
