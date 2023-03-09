use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let mut workers = Vec::new();
    let mut remaining = input.len();
    let mut index = 0;
    while remaining > 0 {
        let mut worker = None;
        if workers.len() < worker_count {
            worker = Some(index);
            index += 1;
        }
        let worker = worker.unwrap();
        let mut count = 0;
        let mut start = 0;
        if remaining < worker_count {
            count = remaining;
        } else {
            count = worker_count;
        }
        remaining -= count;
        let end = start + count;
        let slice = &input[start..end];
        workers.push(Worker {
            index: worker,
            slice: slice.to_vec(),
        });
    }
    for worker in workers {
        let mut counts = worker.counts();
        for (key, value) in counts.iter_mut() {
            let count = counts.get(key).unwrap();
            let current = counts.get_mut(key).unwrap();
            *current += *count;
        }
    }
    counts
}


struct Worker {
    index: usize,
    slice: Vec<&str>,
    
}
