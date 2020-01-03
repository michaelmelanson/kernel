use ringbuffer::RingBuffer;

use kernel::PlatformEvent;
use crate::DeviceID;

lazy_static! {
  static ref EVENT_BUFFER: RingBuffer<PlatformEvent<DeviceID>> = {
    RingBuffer::new_with_capacity(1000)
  };
}

pub(crate) fn push_event(event: PlatformEvent<DeviceID>) {
  EVENT_BUFFER.push(event);
}


pub(crate) fn poll_event() -> Option<PlatformEvent<DeviceID>> {
  EVENT_BUFFER.poll()
}