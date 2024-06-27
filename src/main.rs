use std::future::Future;
use std::num::NonZeroU32;
use std::thread::sleep;
use std::time::Duration;
use std::{fmt, thread};

use crate::atomic_future::AtomicFuture;
use crate::futures::U32Future;
use crate::log::{debug, error, info};
use crate::sample_future::SampleFuture;

mod atomic_future;
mod futures;
mod log;
mod sample_future;

async fn wait_fut<T>(prefix: &str, future: T)
where
    T: Future + fmt::Display,
    T::Output: fmt::Display,
{
    info("Starting Async Await!!");
    let x = future.await;
    debug(format!("{} Output {}", prefix, x));
}

fn set_fut_val<T: U32Future>(fut: T) {
    info("Setting the value in output!!");
    sleep(Duration::from_secs(2));
    fut.update_value(NonZeroU32::new(42).unwrap());
    info("Done setting the value in output!!");
}

/// main func
#[tokio::main]
async fn main() {
    // logging samples ...
    info("Hello, world!");
    debug(format!("Hello, world! {}", "again!!"));
    error(format!("Hello, world! {}", "with Error!!"));

    // Sample Future EX 1
    let t = SampleFuture::default();
    info("Created Sample Future!!");
    let t1 = t.clone();
    let t2 = t.clone();
    tokio::spawn(wait_fut("Sample", t));
    let j = thread::Builder::new()
        .name("Sample Future Setter".into())
        .spawn(|| set_fut_val(t1))
        .unwrap();
    j.join().expect("Could not join!!");
    debug(format!("Sample Future with Sample Output {}", t2.await));

    // Atomic Future EX 2
    let a = AtomicFuture::default();
    info("Created Atomic Future!!");
    let a1 = a.clone();
    let a2 = a.clone();
    tokio::spawn(wait_fut("Atomic", a));
    let k = thread::Builder::new()
        .name("Atomic Future Setter".into())
        .spawn(|| set_fut_val(a1))
        .unwrap();
    k.join().expect("Could not join!!");

    debug(format!("Atomic Future with Sample Output {}", a2.await));
}
