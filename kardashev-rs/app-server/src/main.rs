use kardashev_app_server::run_main;
use kardashev_arg0::arg0_dispatch_or_else;
use kardashev_common::CliConfigOverrides;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|kardashev_linux_sandbox_exe| async move {
        run_main(kardashev_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
