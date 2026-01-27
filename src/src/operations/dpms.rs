use wayland_client::{
    protocol::{wl_output, wl_registry},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols_wlr::output_power_management::v1::client::{
    zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1,
    zwlr_output_power_v1::{self, ZwlrOutputPowerV1},
};
use crate::args::DpmsArgs;

struct DpmsState {
    target_name: String,
    target_state: zwlr_output_power_v1::Mode,
    manager: Option<ZwlrOutputPowerManagerV1>,
    outputs: Vec<(wl_output::WlOutput, String)>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for DpmsState {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_registry::Event::Global { name, interface, .. } = event {
            match &interface[..] {
                "wl_output" => {
                    registry.bind::<wl_output::WlOutput, _, _>(name, 1, qh, ());
                }
                "zwlr_output_power_manager_v1" => {
                    state.manager = Some(registry.bind::<ZwlrOutputPowerManagerV1, _, _>(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for DpmsState {
    fn event(state: &mut Self, output: &wl_output::WlOutput, event: wl_output::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        if let wl_output::Event::Name { name } = event {
            state.outputs.push((output.clone(), name));
        }
    }
}

impl Dispatch<ZwlrOutputPowerManagerV1, ()> for DpmsState {
    fn event(_: &mut Self, _: &ZwlrOutputPowerManagerV1, _: <ZwlrOutputPowerManagerV1 as wayland_client::Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<ZwlrOutputPowerV1, ()> for DpmsState {
    fn event(_: &mut Self, _: &ZwlrOutputPowerV1, _: <ZwlrOutputPowerV1 as wayland_client::Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

pub fn dpms_control(args: &DpmsArgs) {
    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland");
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let mode = match args.state.to_lowercase().as_str() {
        "on" | "1" => zwlr_output_power_v1::Mode::On,
        "off" | "0" => zwlr_output_power_v1::Mode::Off,
        _ => {
            eprintln!("Invalid state. Use 'on', 'off', '1', or '0'.");
            return;
        }
    };

    let mut state = DpmsState {
        target_name: args.target.clone(),
        target_state: mode,
        manager: None,
        outputs: Vec::new(),
    };

    let _registry = conn.display().get_registry(&qh, ());
    event_queue.roundtrip(&mut state).expect("Wayland communication error");

    if let Some(ref manager) = state.manager {
        let mut applied = false;
        for (output, name) in &state.outputs {
            if state.target_name == "all" || &state.target_name == name {
                let power_ctrl = manager.get_output_power(&output, &qh, ());
                power_ctrl.set_mode(state.target_state);
                applied = true;
            }
        }
        if !applied {
            eprintln!("Error: target '{}' not found", args.target);
            eprintln!("Available outputs:");
            for (_, name) in &state.outputs {
                eprintln!(" - {}", name);
            }
            std::process::exit(1);
        }
        event_queue.roundtrip(&mut state).expect("Failed to sync DPMS command");
        println!("DPMS command sent for target: {}", args.target);
    } else {
        eprintln!("Compositor does not support wlr-output-power-management-v1");
    }
}