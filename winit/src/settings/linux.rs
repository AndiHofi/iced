//! Platform specific settings for linux; Wayland and X11.

/// The platform specific window settings of an application.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlatformSpecific {
    /// window id for the compositor
    pub id: Option<WindowId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WindowId {
    pub instance: String,
    pub general: String,
}