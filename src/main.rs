use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

struct ProcessData {
    name: String,
    cpu: f32,
    processes: u8,
}

fn main() {
    let mut s = System::new_all();

    loop {
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
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

        let mut lines = 0;

        clearscreen::clear().unwrap();

        for proc_out in process_output {
            let name = proc_out.name;
            let cpu = proc_out.cpu;
            let processes = proc_out.processes;
            print!("{:<40} {:>8.2} {:>4}", name, cpu, processes);
            lines += 1;
            if lines >= console_height {
                break;
            } else {
                println!();
            }
        }
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
