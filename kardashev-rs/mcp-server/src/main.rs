use kardashev_arg0::arg0_dispatch_or_else;
use kardashev_common::CliConfigOverrides;
use kardashev_mcp_server::run_main;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|kardashev_linux_sandbox_exe| async move {
        run_main(kardashev_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
