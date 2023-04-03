use std::path;

fn main() {
    // threads
    // channels
    // file I/O

    // 1. thread
    // Spawning native OS threads via the spawn function, the argument of this function is a moving closure.
    // These threads will be scheduled by the OS.
    use std::thread;
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    println!("Final sum result: {}", final_result);

    // 2. channel
    // communication between threads
    use std::sync::mpsc::{Receiver, Sender};

    // 3. path
    // represents file paths in the underlying filesystem

    // 4. file I/O
    // represents a file that has been opened (it wraps a file descriptor)

    // 5. child process
    use std::process::Command;
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
    // 5.1 pipe
    use std::process::Stdio;
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };
    // 5.2 wait
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    // 6. FS operations

    // 7. Program arguments

    // 8. FFI: Foregin Function Interface
    // must be declared inside an extern block
}
