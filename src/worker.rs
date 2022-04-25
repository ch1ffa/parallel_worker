use std::sync::mpsc::channel;
use crossbeam_utils::thread;

struct WorkerResult<T> {
  thread_idx: usize,
  data: T,
}

pub fn worker<T, F, R>(list: &[T], f: F) -> Vec<R>
where
  T: Copy + Send + Sync,
  F: (Fn(T) -> R) + Copy + Sync + Send,
  R: Send + Sync,
{
  let threshold = 4;

  if list.len() < threshold {
    list.iter().map(|&x| f(x)).collect()
  } else {
    let chunks = list.chunks((list.len() / num_cpus::get()).max(1));
    let (tx, rx) = channel();

    thread::scope(|scope| {
      for (idx, chunk) in chunks.enumerate() {
        let tx = tx.clone();
        scope.spawn(move |_| {
          let res = WorkerResult{
            thread_idx: idx,
            data: chunk.iter().map(|&x| f(x)).collect::<Vec<_>>(),
          };
          tx.send(res).unwrap();
        });
      }
    }).unwrap();

    drop(tx);

    let mut result = Vec::new();

    for received in rx {
      result.push(received);
    }

    result.sort_by_key(|x| x.thread_idx);

    result.into_iter().map(|x| x.data).flatten().collect::<Vec<_>>()
  }
}
