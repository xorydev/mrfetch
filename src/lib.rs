use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Machine {
    pub user: String,
    pub hostname: String,
    pub distro: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub ramused: u32,
    pub ramtotal: u32,
    pub ramavail: u32,
    pub cpu: String,
}

impl Machine {
    pub fn new() -> Machine {
        let user: String = env::var("USER").unwrap();

        let mut hostname = String::new();
        {
            let hostname_file = File::open("/etc/hostname")
                .expect("u forgor the /etc/hostname file u arch-using moronbox");
            let mut hostname_reader = BufReader::new(hostname_file);
            hostname_reader.read_line(&mut hostname).expect("Failed string conversion... EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
            hostname = hostname.trim().to_string();
        }

        // Read release file, AKA get OS name
        let mut release_distro = String::new();
        {
            let release_file = File::open("/etc/os-release").expect("Failed to find release file.");
            let release_reader = BufReader::new(release_file);
            for line in release_reader.lines() {
                let unwrapped_line: String = line.unwrap();
                if unwrapped_line.starts_with("PRETTY_NAME=") {
                  release_distro = unwrapped_line
                }
            }
        }
        let mut distro_name: String = release_distro[12..release_distro.len() - 1].to_string();
        distro_name = distro_name.replace('\"', "");

        // Quick fix for NixOS
        // I know I should change the way the program gets the distro, but I haven't been able to find a
        // fix that doesn't rely on other crates or libraries.
        if distro_name.starts_with("NixOS") {
            distro_name = "NixOS".to_string();
        }
        let distro: String = distro_name;

        let mut kernel = String::new();
        {
            let kernel_file = File::open("/proc/version").expect("Read the README.md you dumbass");
            let mut kernel_reader = BufReader::new(kernel_file);
            kernel_reader
                .read_line(&mut kernel)
                .expect("Failed string conversion");
        }

        let mut kernel_name: String = (kernel[14..kernel.len()]).to_string();
        kernel_name = kernel_name.split_whitespace().next().unwrap().to_string();

        // Read memfile
        #[allow(unused_assignments)]
        let mut uptime_mins: u32 = 0;
        let mut ramused: u32 = 0;
        let mut ramtotal: u32 = 0;
        let mut ramavail: u32 = 0;
        if let Ok(file) = File::open("/proc/meminfo") {
            // Reader & Iterator
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            // Read 1st & 2nd line
            if let Some(Ok(line)) = lines.next() {
                if let Some(idx) = line.find(char::is_whitespace) {
                    // Reading & Parsing
                    let mut line_processed = line[idx..].trim();
                    line_processed = &line_processed[0..line_processed.len() - 3];
                    // mafs
                    let mut ram_gb: u32 = line_processed.parse().unwrap();
                    ram_gb /= 1048576;
                    ramtotal = ram_gb;
                }
            }

            lines.next();

            if let Some(Ok(line)) = lines.next() {
                if let Some(idx) = line.find(char::is_whitespace) {
                    // Reading & Parsing
                    let mut line_processed = line[idx..].trim();
                    line_processed = &line_processed[0..line_processed.len() - 3];
                    // mafs
                    let mut ram_gb: u32 = line_processed.parse().unwrap();
                    ram_gb /= 1048576;
                    ramavail = ram_gb;
                }
            }

            ramused = ramtotal - ramavail;

        }

            {
                // This took me unusually long.

                // Generic file stuff
                let mut uptime = String::new();
                let uptime_file = File::open("/proc/uptime").expect(":skull:");
                let mut uptime_reader = BufReader::new(uptime_file);
                uptime_reader.read_line(&mut uptime).expect("what");
                let mut iterator = uptime.split_whitespace();
                uptime = iterator
                    .next()
                    .expect("*screeches at the top of his lungs*")
                    .to_string();

                // was never expecting rounding to be this difficult
                let uptimeint = uptime.parse::<f32>();
                let roundeduptimeint: u32 = uptimeint.expect("phoque").round() as u32;
                uptime_mins = roundeduptimeint / 60;
            }

            // Get shell
            let mut shell = String::new();
            let shell_raw = env::var("SHELL").expect("Could not read $SHELL variable");

            // Split the path using '/' as the separator
            // Thanks ChatGPT
            let parts: Vec<&str> = shell_raw.rsplitn(2, '/').collect();

            // Check if the path contains at least one '/'
            if parts.len() > 1 {
                shell = parts[0].to_string();
            }


            let mut cpu = String::new();
            // Time for a challenge, Get CPU model!
            {
                let file = File::open("/proc/cpuinfo").expect("Could not read /proc/cpuinfo");
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                // Read up until the 5th line
                let mut i = 1;
                while i < 5 {
                    lines.next();
                    i += 1;
                }

                if let Some(Ok(line)) = lines.next() {
                    cpu = line
                        .split(':')
                        .nth(1)
                        .expect("Failed to parse CPU Info")
                        .trim()
                        .to_string();
                }
            }


        // Fucking finally.
        Machine { user, hostname, distro, kernel: kernel_name, uptime: uptime_mins.to_string(), shell, ramused, ramtotal, ramavail, cpu }

    }

}

impl Default for Machine {
  fn default() -> Self {
    Self::new()
  }
}
