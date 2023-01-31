use dll_syringe::Syringe;
use dll_syringe::process::OwnedProcess;
use tracing::metadata::LevelFilter;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    let _ = color_eyre::install();
    tracing_subscriber::fmt().with_max_level(LevelFilter::INFO).init();

    info!("Finding target process");

    let target_process = OwnedProcess::find_first_by_name("PlantsVsZombies").unwrap();
    info!("Found target process");

    let syringe = Syringe::for_process(target_process);
    info!("Created syringe");

    let _injected_payload = syringe.inject("./target/i686-pc-windows-msvc/debug/pvzmod.dll").unwrap();
    // let _injected_payload = syringe.inject("./target/debug/pvzmod.dll").unwrap();
    info!("DLL injected");

    println!("done.");

    // syringe.eject(injected_payload).unwrap();

    Ok(())
}
