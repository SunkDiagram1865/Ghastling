use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::task::JoinHandle;
use tracing::warn;

#[derive(Default)]
pub(super) struct MinecraftLanAnnouncer {
    active: Option<ActiveAnnouncement>,
}

struct ActiveAnnouncement {
    port: u16,
    task: JoinHandle<()>,
}

impl MinecraftLanAnnouncer {
    pub(super) fn sync(&mut self, port: Option<u16>) {
        if self.active.as_ref().is_some_and(|active| {
            Some(active.port) == port && !active.task.is_finished()
        }) {
            return;
        }

        if let Some(active) = self.active.take() {
            active.task.abort();
        }
        if let Some(port) = port {
            self.active = Some(ActiveAnnouncement {
                port,
                task: tokio::spawn(run(port)),
            });
        }
    }
}

impl Drop for MinecraftLanAnnouncer {
    fn drop(&mut self) {
        if let Some(active) = self.active.take() {
            active.task.abort();
        }
    }
}

fn create_announcer_socket() -> std::io::Result<Socket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_multicast_loop_v4(true)?;
    socket.set_multicast_ttl_v4(4)?;
    socket
        .bind(&SockAddr::from(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)))?;
    Ok(socket)
}

async fn run(port: u16) {
    let socket = match create_announcer_socket() {
        Ok(socket) => socket,
        Err(error) => {
            warn!("failed to create Minecraft LAN announcer socket: {error}");
            return;
        }
    };

    let target =
        SockAddr::from(SocketAddrV4::new(Ipv4Addr::new(224, 0, 2, 60), 4445));
    let message = format!("[MOTD]Ghastling Multiplayer[/MOTD][AD]{port}[/AD]");
    let mut interval =
        tokio::time::interval(std::time::Duration::from_millis(1500));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;
        match socket.send_to(message.as_bytes(), &target) {
            Ok(_) => {}
            Err(error) => {
                tracing::debug!(
                    target: "terracotta",
                    "failed to send Minecraft LAN announcement: {error}"
                );
            }
        }
    }
}
