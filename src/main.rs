use mrfetch::Machine;
use std::process::Command;

fn main() {
  let machine = Machine::new();

  let figlet = Command::new("figlet")
    .args(["-f", "smslant", machine.distro.as_str()])
    .output()
    .expect("AS OF THE UPDATE, FIGLET IS *NECESSARY*.");

  let figlet: String = String::from_utf8(figlet.stdout).unwrap().trim_end().to_string(); // No possible way the shell can return non-UTF-8... right? RIGHT!?

  println!("{}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
\x1B[38:5:15m┐\x1B[38:5:196m • User: {}
\x1B[38:5:15m│\x1B[38:5:226m • Hostname: {}
\x1B[38:5:15m│\x1B[38:5:46m • Kernel Version: {}
\x1B[38:5:15m│\x1B[38:5:45m • Uptime (minutes): {}
\x1B[38:5:15m│\x1B[38:5:165m • Shell: {}
\x1B[38:5:15m│\x1B[38:5:201m • RAM: {}GB
\x1B[38:5:15m┘\x1B[38:5:219m • CPU: {} ",
  figlet, machine.user, machine.hostname, machine.kernel, machine.uptime, machine.shell, machine.ramtotal, machine.cpu);

}
