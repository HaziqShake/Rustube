use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConfigureWindowAux, ConnectionExt, CreateWindowAux, WindowClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Loading,
    Playing,
    Paused,
    Stopped,
    Error,
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    StateChanged(PlayerState),
    PositionChanged(Duration),
    DurationChanged(Duration),
    EndOfFile,
    Error(String),
}

struct X11State {
    child_window: u32,
}

pub struct PlayerController {
    state: Arc<Mutex<PlayerState>>,
    child: Arc<Mutex<Option<Child>>>,
    x11_state: Arc<Mutex<Option<X11State>>>,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PlayerState::Idle)),
            child: Arc::new(Mutex::new(None)),
            x11_state: Arc::new(Mutex::new(None)),
        }
    }

    /// Try to find the top-level Rustube X11 window
    fn find_rustube_window() -> Option<u32> {
        for _ in 0..30 {
            if let Ok((conn, screen_num)) = x11rb::connect(None) {
                let screen = conn.setup().roots.get(screen_num)?;
                if let Ok(tree) = conn.query_tree(screen.root) {
                    if let Ok(reply) = tree.reply() {
                        let net_wm_name_atom = conn.intern_atom(false, b"_NET_WM_NAME").ok().and_then(|c| c.reply().ok()).map(|r| r.atom);
                        let utf8_string_atom = conn.intern_atom(false, b"UTF8_STRING").ok().and_then(|c| c.reply().ok()).map(|r| r.atom);

                        for &win in reply.children.iter().rev() {
                            // Check _NET_WM_NAME
                            if let (Some(net_atom), Some(utf8_atom)) = (net_wm_name_atom, utf8_string_atom) {
                                if let Ok(cookie) = conn.get_property(false, win, net_atom, utf8_atom, 0, 1024) {
                                    if let Ok(reply) = cookie.reply() {
                                        if let Ok(title) = String::from_utf8(reply.value) {
                                            if title.contains("Rustube") {
                                                info!("Found Rustube window by _NET_WM_NAME: {:#x}", win);
                                                return Some(win);
                                            }
                                        }
                                    }
                                }
                            }

                            // Check WM_NAME
                            if let Ok(cookie) = conn.get_property(false, win, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024) {
                                if let Ok(reply) = cookie.reply() {
                                    if let Ok(title) = String::from_utf8(reply.value) {
                                        if title.contains("Rustube") {
                                            info!("Found Rustube window by WM_NAME: {:#x}", win);
                                            return Some(win);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        None
    }

    /// Play YouTube video embedded inside the app window at exact coordinates
    pub fn play_embedded_at(&self, video_id: &str, x: i32, y: i32, width: u32, height: u32) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        *state = PlayerState::Loading;

        // Clean up previous child / process
        self.stop_internal();

        let yt_url = format!("https://www.youtube.com/watch?v={}", video_id);
        info!("Spawning embedded MPV for video: {} at ({}, {}) {}x{}", yt_url, x, y, width, height);

        let mut cmd = Command::new("mpv");
        cmd.arg(&yt_url)
           .arg("--vo=gpu")
           .arg("--gpu-context=x11")
           .arg("--hwdec=auto-safe")
           .arg("--ytdl-format=bestvideo[vcodec^=vp9][height<=1080]+bestaudio/bestvideo[vcodec^=avc1][height<=1080]+bestaudio/best[height<=1080]/best")
           .arg("--keep-open=yes")
           .arg("--input-ipc-server=/tmp/rustube_mpv.sock")
           .arg("--cursor-autohide=1000")
           .arg("--no-border");

        // Try embedding in X11 child window
        let mut embedded = false;
        if let Some(parent_win) = Self::find_rustube_window() {
            if let Ok((conn, screen_num)) = x11rb::connect(None) {
                let screen = &conn.setup().roots[screen_num];
                if let Ok(child_id) = conn.generate_id() {
                    let win_aux = CreateWindowAux::new().background_pixel(0x000000);
                    if conn.create_window(
                        x11rb::COPY_DEPTH_FROM_PARENT,
                        child_id,
                        parent_win,
                        x as i16,
                        y as i16,
                        width as u16,
                        height as u16,
                        0,
                        WindowClass::INPUT_OUTPUT,
                        screen.root_visual,
                        &win_aux,
                    ).is_ok() && conn.map_window(child_id).is_ok() && conn.flush().is_ok() {
                        cmd.arg(format!("--wid={}", child_id));
                        *self.x11_state.lock().unwrap() = Some(X11State { child_window: child_id });
                        embedded = true;
                        info!("Successfully embedded MPV into X11 child window {:#x}", child_id);
                    }
                }
            }
        }

        if !embedded {
            info!("X11 parent window not found; launching docked player fallback");
            cmd.arg("--autofit=960x540")
               .arg("--title=Rustube Player");
        }

        let child_proc = cmd.spawn()?;
        *self.child.lock().unwrap() = Some(child_proc);
        *state = PlayerState::Playing;

        Ok(())
    }

    /// Update child window geometry on resize
    pub fn update_geometry(&self, x: i32, y: i32, width: u32, height: u32) {
        if let Some(x11) = self.x11_state.lock().unwrap().as_ref() {
            if let Ok((conn, _)) = x11rb::connect(None) {
                let aux = ConfigureWindowAux::default()
                    .x(x)
                    .y(y)
                    .width(width)
                    .height(height);
                let _ = conn.configure_window(x11.child_window, &aux);
                let _ = conn.flush();
            }
        }
    }

    /// Legacy compat method
    pub fn play_embedded(&self, video_id: &str) -> anyhow::Result<()> {
        // Default player area coordinates on 1440x900 window
        // Top bar is ~56px + padding ~12px => y=74
        // Sidebar/left margin is 20px => x=20
        // Width for FillPortion(7) on 1440px is approx 920px, height for 16:9 is 518px
        self.play_embedded_at(video_id, 20, 74, 920, 518)
    }

    fn stop_internal(&self) {
        let mut child_lock = self.child.lock().unwrap();
        if let Some(mut child) = child_lock.take() {
            let _ = child.kill();
            let _ = child.wait();
        }

        let mut x11_lock = self.x11_state.lock().unwrap();
        if let Some(x11) = x11_lock.take() {
            if let Ok((conn, _)) = x11rb::connect(None) {
                let _ = conn.destroy_window(x11.child_window);
                let _ = conn.flush();
            }
        }

        let _ = std::fs::remove_file("/tmp/rustube_mpv.sock");
    }

    pub fn stop(&self) {
        self.stop_internal();
        let mut state = self.state.lock().unwrap();
        *state = PlayerState::Stopped;
    }

    pub fn is_playing(&self) -> bool {
        let state = self.state.lock().unwrap();
        *state == PlayerState::Playing
    }

    pub fn toggle_pause(&self) {
        Self::send_ipc(r#"{"command": ["cycle", "pause"]}"#);
    }

    pub fn seek_relative(&self, seconds: f64) {
        Self::send_ipc(&format!(r#"{{"command": ["seek", {}, "relative"]}}"#, seconds));
    }

    pub fn seek_percent(&self, percent: f64) {
        Self::send_ipc(&format!(r#"{{"command": ["seek", {}, "absolute-percent"]}}"#, percent));
    }

    pub fn set_volume(&self, volume: u32) {
        Self::send_ipc(&format!(r#"{{"command": ["set_property", "volume", {}]}}"#, volume));
    }

    pub fn toggle_fullscreen(&self) {
        Self::send_ipc(r#"{"command": ["cycle", "fullscreen"]}"#);
    }

    /// Query live playback state from MPV over the IPC socket.
    /// Returns (position_secs, duration_secs, paused). Each property is a
    /// separate short-lived connection with a 300ms read timeout, so a
    /// dead or busy MPV never blocks the UI thread for long.
    /// `None` per field means "unknown right now", not an error.
    pub fn query_state(&self) -> (Option<f64>, Option<f64>, Option<bool>) {
        (
            Self::get_property_f64("time-pos"),
            Self::get_property_f64("duration"),
            Self::get_property_bool("pause"),
        )
    }

    fn get_property_f64(name: &str) -> Option<f64> {
        Self::get_property(name)?.as_f64()
    }

    fn get_property_bool(name: &str) -> Option<bool> {
        Self::get_property(name)?.as_bool()
    }

    fn get_property(name: &str) -> Option<serde_json::Value> {
        use std::io::{BufRead, BufReader, Write};
        use std::os::unix::net::UnixStream;
        use std::time::Duration as StdDuration;

        let mut stream = UnixStream::connect("/tmp/rustube_mpv.sock").ok()?;
        let _ = stream.set_read_timeout(Some(StdDuration::from_millis(300)));
        let _ = stream.set_write_timeout(Some(StdDuration::from_millis(300)));
        let cmd = format!(
            r#"{{"command": ["get_property", "{}"], "request_id": 1}}"#,
            name
        );
        writeln!(stream, "{}", cmd).ok()?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        // MPV answers each request with exactly one JSON object.
        for _ in 0..5 {
            line.clear();
            if reader.read_line(&mut line).ok()? == 0 {
                return None;
            }
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                if json.get("request_id").and_then(|v| v.as_u64()) == Some(1) {
                    if json.get("error").and_then(|e| e.as_str()) == Some("success") {
                        return json.get("data").cloned();
                    }
                    return None;
                }
            }
        }
        None
    }

    fn send_ipc(cmd: &str) {
        use std::io::Write;
        use std::os::unix::net::UnixStream;

        if let Ok(mut stream) = UnixStream::connect("/tmp/rustube_mpv.sock") {
            let _ = writeln!(stream, "{}", cmd);
        }
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PlayerController {
    fn drop(&mut self) {
        self.stop_internal();
    }
}
