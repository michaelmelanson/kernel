use ringbuffer::RingBuffer;

use kernel::{PlatformEvent};
use crate::X8664Platform;

lazy_static! {
  static ref EVENT_BUFFER: RingBuffer<PlatformEvent<X8664Platform>> = {
    RingBuffer::new_with_capacity(1000)
  };
}

pub(crate) fn push_event(event: PlatformEvent<X8664Platform>) {
  EVENT_BUFFER.push(event);
}


pub(crate) fn poll_event() -> Option<PlatformEvent<X8664Platform>> {
  EVENT_BUFFER.poll()
}