#![no_std]

extern crate alloc;
extern crate spin;

use self::alloc::vec::Vec;
use self::alloc::sync::Arc;
use spin::Mutex;

/// A thread-safe circular buffer.
///
/// # Basic usage
///
/// You can write items to the buffer...
///
/// ```
/// # use ringbuffer::RingBuffer;
/// let ringbuffer = RingBuffer::new_with_capacity(10);
///
/// // It starts off empty.
/// assert_eq!(0, ringbuffer.size());
///
/// // Push some data into it.
/// ringbuffer.push(55);
/// ringbuffer.push(42);
///
/// // It now has data in it.
/// assert_eq!(2, ringbuffer.size());
/// ```
///
/// ... and read them out again in order ...
///
/// ```
/// # use ringbuffer::RingBuffer;
/// # let ringbuffer = RingBuffer::new_with_capacity(10);
/// # ringbuffer.push(55);
/// # ringbuffer.push(42);
/// let first = ringbuffer.pop();
/// let second = ringbuffer.pop();
///
/// assert_eq!(55, first);
/// assert_eq!(42, second);
/// ```
///
/// # Overflow / Underflow
///
/// When you put too many items in, it starts overwriting the oldest. This
/// means that readers can never block writers by failing to consume the
/// data.
///
/// ```
/// # use ringbuffer::RingBuffer;
/// let ringbuffer = RingBuffer::new_with_capacity(5);
///
/// // Push some data into it.
/// ringbuffer.push(1);
/// ringbuffer.push(2);
/// ringbuffer.push(3);
/// ringbuffer.push(4);
/// ringbuffer.push(5);
///
/// // It's now full, with five items.
/// assert_eq!(5, ringbuffer.size());
///
/// // Put another item in.
/// ringbuffer.push(6);
///
/// // There's still five items.
/// assert_eq!(5, ringbuffer.size());
///
/// // The first item is '2' -- the '1' was dropped -- and it continues from
/// // there.
/// assert_eq!(2, ringbuffer.pop());
/// assert_eq!(3, ringbuffer.pop());
/// assert_eq!(4, ringbuffer.pop());
/// assert_eq!(5, ringbuffer.pop());
/// assert_eq!(6, ringbuffer.pop());
/// ```
///
/// When you read too fast, it spins until data is available:
///
/// ```
/// # use ringbuffer::RingBuffer;
/// let ringbuffer = RingBuffer::new_with_capacity(10);
///
/// let reader_lock = ringbuffer.clone();
/// let reader = std::thread::spawn(move || {
///   // No data is available yet.
///   assert_eq!(None, reader_lock.poll());
///
///   // Block until we get data then return it
///   assert_eq!(55, reader_lock.pop());
/// });
///
/// std::thread::sleep(std::time::Duration::from_millis(100));
/// ringbuffer.push(55);
///
/// reader.join().expect("oh no the reader failed");
/// ```
///

#[derive(Clone)]
pub struct RingBuffer<Item: Copy> {
  mutex: Arc<Mutex<RingBufferData<Item>>>
}

/// The thread-unsafe innards.
struct RingBufferData<Item: Copy> {
  array: Vec<Item>,
  start: usize,
  count: usize
}

impl <Item: Copy> RingBuffer<Item> {
  pub fn new_with_capacity(capacity: usize) -> Self {
    RingBuffer {
      mutex: Arc::new(
        Mutex::new(RingBufferData {
          array: Vec::with_capacity(capacity),
          start: 0,
          count: 0
        })
      )
    }
  }


  #[inline]
  pub fn capacity(&self) -> usize {
    self.mutex.lock().capacity()
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.mutex.lock().size()
  }

  pub fn push(&self, item: Item) {
    let mut inner = self.mutex.lock();
    inner.push(item)
  }

  pub fn poll(&self) -> Option<Item> {
    let mut inner = self.mutex.lock();
    inner.poll()
  }

  pub fn pop(&self) -> Item {
    loop {
      match self.poll() {
        Some(item) => { return item; },
        None => {}
      }
    }
  }
}

/// The thread-unsafe innards of the ring buffer.
impl <Item: Copy> RingBufferData<Item> {

  #[inline]
  pub fn capacity(&self) -> usize {
    self.array.capacity()
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.count
  }

  #[inline]
  fn is_full(&self) -> bool {
    self.count == self.capacity()
  }

  #[inline]
  fn is_empty(&self) -> bool {
    self.count == 0
  }

  #[inline]
  fn wrap_index(&self, index: usize) -> usize {
    index % self.capacity()
  }

  #[inline]
  fn write_position(&self) -> usize {
    self.wrap_index(self.start + self.count)
  }

  #[inline]
  fn read_position(&self) -> usize {
    self.start
  }



  pub fn push(&mut self, item: Item) {
    let index = self.write_position();

    if index == self.array.len() {
      self.array.push(item);
    } else {
      self.array[index] = item;
    }

    if self.is_full() {
      self.start = self.wrap_index(self.start + 1);
    } else {
      self.count += 1;
    }
  }

  pub fn poll(&mut self) -> Option<Item> {
    if self.is_empty() {
      None
    } else {
      let index = self.read_position();
      let item = self.array[index];
      self.start = self.wrap_index(self.start + 1);
      self.count -= 1;
      Some(item)
    }
  }
}
