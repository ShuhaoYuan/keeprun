use std::io::Error;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use std::env;


fn keeprun(command: &mut Command) -> Result<Child, Error> {
    let mut child: Child = command.spawn()?;

    loop {
        let status = child.wait();

        match status {
            Ok(status) => {
                if status.success() {
                    // 子进程正常退出
                    println!("The program exited normally.");
                    break;
                } else {
                    // 子进程异常退出，等待一段时间后重新启动
                    println!("The program exited abnormally. Will restart in 1 second");
                    thread::sleep(Duration::from_secs(1));
                    child = command.spawn()?;
                }
            },
            Err(_) => {
                // 子进程崩溃，等待一段时间后重新启动
                println!("The program crashed. Will restart in 1 second");
                thread::sleep(Duration::from_secs(1));
                child = command.spawn()?;
            }
        }
    }

    return Ok(child);

}


fn main() {

    // build command
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No program was executed.");
        return;
    }

    let program_s = args[1..].join(" ");
    println!("{}", program_s);

    let mut command = Command::new(&args[1]);
    if args.len() > 2 {
        command.args(&args[2..]);
    }

    // keep run the program
    match keeprun(&mut command) {
        Ok(_child) => {},
        Err(error) => {
            println!("{}", error);
            return;
        }
    }
}

