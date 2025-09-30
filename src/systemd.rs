use zbus::{Connection, Result, proxy::Proxy};

/// Systemd D-Bus interface for managing services
pub struct SystemdManager {
    connection: Connection,
}

impl SystemdManager {
    /// Create a new SystemdManager
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        Ok(SystemdManager { connection })
    }

    /// Start a systemd service
    pub async fn start_service(&self, service_name: &str) -> Result<()> {
        let proxy = Proxy::new(
            &self.connection,
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
        ).await?;

        let unit_name = format!("{}.service", service_name);
        let _reply = proxy.call_method("StartUnit", &(unit_name, "replace")).await?;

        println!("Service {} started successfully", service_name);
        Ok(())
    }

    /// Stop a systemd service
    pub async fn stop_service(&self, service_name: &str) -> Result<()> {
        let proxy = Proxy::new(
            &self.connection,
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
        ).await?;

        let unit_name = format!("{}.service", service_name);
        let _reply = proxy.call_method("StopUnit", &(unit_name, "replace")).await?;

        println!("Service {} stopped successfully", service_name);
        Ok(())
    }

    /// Get the status of a systemd service
    pub async fn get_service_status(&self, service_name: &str) -> Result<String> {
        let proxy = Proxy::new(
            &self.connection,
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
        ).await?;

        let unit_name = format!("{}.service", service_name);
        
        // Get the unit object path
        let reply = proxy.call_method("GetUnit", &unit_name).await?;
        let unit_path: zbus::zvariant::OwnedObjectPath = reply.body().deserialize()?;

        // Create a proxy for the unit
        let unit_proxy = Proxy::new(
            &self.connection,
            "org.freedesktop.systemd1",
            &unit_path,
            "org.freedesktop.systemd1.Unit",
        ).await?;

        // Get ActiveState property
        let active_state: String = unit_proxy
            .get_property("ActiveState")
            .await?;

        // Get LoadState property
        let load_state: String = unit_proxy
            .get_property("LoadState")
            .await?;

        // Get SubState property
        let sub_state: String = unit_proxy
            .get_property("SubState")
            .await?;

        Ok(format!(
            "Service: {}\nLoaded: {}\nActive: {} ({})",
            service_name, load_state, active_state, sub_state
        ))
    }
}