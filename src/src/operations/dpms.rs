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
    outputs: Vec<(u32, wl_output::WlOutput)>,
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
          let output = registry.bind::<wl_output::WlOutput, _, _>(name, 2, qh, ());
          s.outputs.push((name, output));
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
      let (idx, (_, o)) = s.outputs.iter().enumerate().find(|&(_, (oname, _))| *oname == name).unwrap();
      o.release();
      s.outputs.remove(idx);
    }
  }
}

delegate_noop!(DpmsState: ignore wl_seat::WlSeat);
delegate_noop!(DpmsState: ignore wl_output::WlOutput);
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
    event_queue.roundtrip(&mut state).expect("Wayland communication error");

    if let Some(ref manager) = state.manager {
        let mut applied = false;
        for (name, output) in &state.outputs {
            if state.target_name == "all" || state.target_name == name.to_string() {
                let power_ctrl = manager.get_output_power(&output, &qh, ());
                power_ctrl.set_mode(state.target_state);
                applied = true;
            }
        }
        if !applied {
            eprintln!("Error: target '{}' not found", args.target);
            eprintln!("Available outputs:");
            for (_, output) in &state.outputs {
                eprintln!(" - {:?}", output);
            }
            std::process::exit(1);
        }
        event_queue.roundtrip(&mut state).expect("Failed to sync DPMS command");
        println!("DPMS command sent for target: {}", args.target);
    } else {
        eprintln!("Compositor does not support wlr-output-power-management-v1");
    }
}