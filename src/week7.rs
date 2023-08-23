use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn fn_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}

// fn foo() {
//     let s = String::from("Hello");
//     thread::spawn(|| {
//         println!("Length: {}", s.len());
//     });
// }

fn scope_thread() {
    // 스레드 범위 밖에 있는 변수
    let s = String::from("Hello");

    // spawn에서는 빌릴 수 없지만 scope에서는 빌릴 수 있다
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}

fn channel() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}

fn unbounded_channels() {
    // mpsc::channel() 함수는 경계가 없는 비동기 채널을 생성합니다:
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}

fn bounded_channels() {
    // 유한한 (동기식) 채널을 사용하면, 전송은 현재 스레드를 차단할 수 있습니다.
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}

fn fn_arc() {
    // Arc<T>는 Arc::clone을 통해 공유 읽기 전용 접근을 허용합니다.
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}

fn fn_mutex() {
    // Mutex<T>를 이용하면 불변 참조를 통해서도 T의 값을 수정할 수가 있으며, 이에 더해서 한 번에 한 스레드만 T의 값을 접근(읽거나 쓰거나)함을 보장해 줍니다:
    let v = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", v.lock().unwrap());

    {
        let mut guard = v.lock().unwrap();
        guard.push(40);
    }

    println!("v: {:?}", v.lock().unwrap());
}

pub fn week7() {
    fn_thread();
    scope_thread();
    channel();
    unbounded_channels();
    bounded_channels();
    fn_arc();
    fn_mutex();
}