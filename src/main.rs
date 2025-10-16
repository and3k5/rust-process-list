use std::time::Duration;

use crossterm_cursor::cursor;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

struct ProcessData {
    name: String,
    cpu: f32,
    processes: u8,
}

fn main() {
    let mut s = System::new_all();
    let mut cursor = cursor();

    loop {
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        if sysinfo::MINIMUM_CPU_UPDATE_INTERVAL < Duration::from_secs(1) {
            std::thread::sleep(std::time::Duration::from_millis(1000 - sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.as_millis() as u64));
        }
        // Refresh CPU usage to get actual value.
        s.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu(),
        );
        let process_output = create_grouped_process_data(&s);

        let console_height = match term_size::dimensions() {
            Some((_, h)) => h,
            None => 24, // default height if unable to determine
        };

        let mut line_output: Vec<String> = Vec::with_capacity(console_height);

        let mut lines = 0;

        line_output.push(format!("{:<40} {:>8} {:>7}", "Process", "CPU", "# proc"));
        lines += 1;

        let mut total_processes: u128 = 0;
        let mut total_cpu = 0.0;

        for proc_out in process_output {
            let name = proc_out.name;
            let cpu = proc_out.cpu;
            let processes = proc_out.processes;
            total_processes += processes as u128;
            total_cpu += cpu;
            if lines >= (console_height - 1) {
                continue;
            } else {
                line_output.push(format!("{:<40} {:>8.2} {:>7}", name, cpu, processes));
                lines += 1;
                println!();
            }
        }

        line_output.push(format!("{:<40} {:>8.2} {:>7}", "Total", total_cpu, total_processes));
        lines += 1;

        print!("{}", line_output.join("\n"));
        cursor.move_up(lines as u16).unwrap();
    }
}

fn create_grouped_process_data(s: &System) -> Vec<ProcessData> {
    let mut process_output: Vec<ProcessData> = Vec::with_capacity(10);

    for process in s.processes() {
        let cmd = process.1.cmd();
        if cmd.len() == 0 {
            continue;
        }
        let name = process.1.name().to_string_lossy();
        let cpu = process.1.cpu_usage().clone();
        let mut did_update = false;
        for proc_out in process_output.iter_mut() {
            if proc_out.name == name {
                proc_out.cpu += cpu;
                proc_out.processes += 1;
                did_update = true;
                break;
            }
        }
        if did_update == false {
            process_output.push(ProcessData {
                name: name.to_string(),
                cpu: cpu,
                processes: 1,
            });
        }
    }

    process_output.sort_by(|a, b| {
        b.cpu
            .partial_cmp(&a.cpu)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    return process_output;
}
