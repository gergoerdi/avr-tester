use super::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct SleepFuture {
    duration: CpuDuration,
}

impl SleepFuture {
    pub fn new(duration: CpuDuration) -> Self {
        Self { duration }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        ComponentRuntime::with(|rt| {
            let this = self.get_mut();

            this.duration -= rt.tt();

            if this.duration.is_zero() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }
}
