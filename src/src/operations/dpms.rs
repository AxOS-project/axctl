use wayland_client::{
    Connection, Dispatch, QueueHandle, delegate_noop, protocol::{wl_output, wl_registry, wl_seat}
};
use wayland_protocols_wlr::output_power_management::v1::client::{
    zwlr_output_power_manager_v1::{self, ZwlrOutputPowerManagerV1},
    zwlr_output_power_v1::{self},
};
use crate::args::DpmsArgs;

struct DpmsState {
    target_name: String,
    target_state: zwlr_output_power_v1::Mode,
    manager: Option<ZwlrOutputPowerManagerV1>,
    outputs: Vec<(u32, wl_output::WlOutput, String)>,
    seat: Option<wl_seat::WlSeat>,
}
impl Dispatch<wl_registry::WlRegistry, ()> for DpmsState {
    fn event(
    s: &mut Self,
    registry: &wl_registry::WlRegistry,
    event: wl_registry::Event,
    _: &(),
    _: &Connection,
    qh: &QueueHandle<DpmsState>,
  ) {
    if let wl_registry::Event::Global { name, interface, .. } = event {
        match &interface[..] {
            "wl_output" => {
            let output = registry.bind::<wl_output::WlOutput, _, _>(name, 4, qh, ());
            s.outputs.push((name, output, String::new()));
            }
            "wl_seat" => {
            s.seat = Some(registry.bind::<wl_seat::WlSeat, _, _>(name, 7, qh, ()));
            }
            "zwlr_output_power_manager_v1" => {
            s.manager =
                Some(registry.bind::<zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1, _, _>(name, 1, qh, ()));
            }
            _ => {}
        }
        } else if let wl_registry::Event::GlobalRemove { name } = event {
            if let Some(idx) = s.outputs.iter().position(|(id, _, _)| *id == name) {
                let (_, o, _) = s.outputs.remove(idx);
                o.release();
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for DpmsState {
    fn event(
        state: &mut Self,
        proxy: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<DpmsState>,
    ) {
        if let wl_output::Event::Name { name } = event {
            if let Some((_, _, stored_name)) = state.outputs.iter_mut().find(|(_, o, _)| o == proxy) {
                *stored_name = name;
            }
        }
    }
}

delegate_noop!(DpmsState: ignore wl_seat::WlSeat);
delegate_noop!(DpmsState: zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1);
delegate_noop!(DpmsState: ignore zwlr_output_power_v1::ZwlrOutputPowerV1);

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
        seat: None,
    };

    let _registry = conn.display().get_registry(&qh, ());
    
    // Roundtrip 1: Get globals and bind them
    event_queue.roundtrip(&mut state).expect("Wayland communication error");
    // Roundtrip 2: Receive the events (specifically Name) from the bound globals
    event_queue.roundtrip(&mut state).expect("Wayland communication error");

    if let Some(ref manager) = state.manager {
        let mut applied = false;
        for (_, output, name) in &state.outputs {
            // Now we match against the actual string name
            if state.target_name == "all" || state.target_name == *name {
                let power_ctrl = manager.get_output_power(&output, &qh, ());
                power_ctrl.set_mode(state.target_state);
                applied = true;
            }
        }

        if !applied {
            eprintln!("Error: target '{}' not found", args.target);
            eprintln!("Available outputs:");
            for (_, _, name) in &state.outputs {
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