//! Utilities for managing many windows in an [`iced`] application.
//!
//! ## Goals
//!
//! Working with multiple windows in iced can become quite painful quite quickly. If you want to introduce a window type with unique behavior, you may have to make additions in more than five places accross your codebase. Oversights are easy, and most of the mistakes you can make aren't caught by the compiler. This library seeks to ease this experince by making defining and working with multiple windows simpler, more intuitive, and harder to screw up.
//!
//! ## Usage
//!
//! The first step is to define the windows that will appear in your app. This is done by creating a corresponding struct and implementing the [`Window`] trait for it. This trait will describe the logic behind that window's content, title, and theme, as well as defining its spawn-time settings.
//!
//! Once you have your windows defined, you have to declare them using the [`multi_window`] macro **at the top level of your crate** (`main.rs` or `lib.rs`)[^1]. This macro will generate an enum that allows for windows to be passed around polymorphically without the use of dynamic dispatch.
//!
//! Finally, add a [`WindowManager`] to your app state. This keeps track of the
//! [`Id`]s and
//! corresponding identities of all of the windows in your app. It will also return the proper
//! view, title, and theme given an [`Id`]. Thus, instead of checking the [`Id`] in these functions, you just call the right method on the [`WindowManager`].  
//! This has one caveat: you have to tell the [`WindowManager`] when a window is closed. This is done by calling the [`closed`](WindowManager::closed) method with the [`Id`] of the window that was closed. The simplest way to do this is to add a message variant for a window closure, then use [`iced::event::listen_with`] to listen for window closures.
//!
//! ```rust
//! fn subscription(&self) -> iced::Subscription<Self::Message> {
//!     iced::event::listen_with(|event, _| {
//!         if let iced::Event::Window(id, iced::window::Event::Closed) = event {
//!             Some(Message::WindowClosed(id))
//!         } else {
//!             None
//!         }
//!     })
//! }
//!
//! fn update(^mut self, message: Self::Message) -> iced::Command<Self::Message> {
//!     match message {
//!         ...
//!         Message::WindowClosed(id) => {
//!             self.windows.closed(id);
//!         }
//!     }
//! }
//! ```
//!
//! When using any function that expects a window (such as [`WindowManager::spawn`]), pass in the
//! [`window!`] macro. This is shorthand for the particular enum variant of the window you want to
//! use.
//!
//! The name of the enum generated by the [`multi_window`] macro is `WindowUnion`, and can be
//! imported if needed.
//!
//! [^1]: The [`window!`] macro assumes that the generated enum is in at the top level to avoid requiring
//! you to import it yourself.

use iced::{
    multi_window::Application,
    window::{self, Id},
    Command,
};
pub use iced_multi_window_macros::multi_window;
use std::{collections::HashMap, marker::PhantomData};

/// Takes in a [`Window`] identifier and returns an enum variant that can be used by various
/// functions.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! window {
    ($window: ident) => {
        crate::WindowUnion::$window($window)
    };
}

/// A struct for managing multiple windows. Keeps track of the windows, their [`Id`]s, and handles
/// spawning and closing them.
#[derive(Default, Debug)]
pub struct WindowManager<App: Application, WindowUnion: Window<App>> {
    windows: HashMap<Id, WindowUnion>,
    phantom: PhantomData<App>,
}

impl<App: Application, WindowUnion: Window<App>> WindowManager<App, WindowUnion> {
    pub fn new(main: WindowUnion) -> Self {
        let mut windows = HashMap::new();
        windows.insert(Id::MAIN, main);
        Self {
            windows,
            phantom: PhantomData,
        }
    }

    /// Spawns the given window. Expects a [`window!`].
    pub fn spawn(&mut self, window: WindowUnion) -> (Id, Command<App::Message>) {
        let (id, command) = window::spawn(window.settings());
        self.windows.insert(id, window);
        (id, command)
    }

    // Returns a command to close all the windows.
    pub fn close_all(&mut self) -> Command<App::Message> {
        let mut commands = Vec::new();
        for id in self.windows.keys() {
            commands.push(window::close(*id));
        }
        Command::batch(commands)
    }

    pub fn any_of(&self, window: WindowUnion) -> bool {
        self.windows.values().any(|w| w == &window)
    }

    pub fn view<'a>(&'a self, app: &'a App, id: Id) -> iced::Element<'_, App::Message, App::Theme> {
        self.windows
            .get(&id)
            .expect(
                "No window found with that Id. Make sure you're only spawning via the window manager!",
            )
            .view(app)
    }

    pub fn title(&self, app: &App, id: Id) -> String {
        self.windows
            .get(&id)
            .expect(
                "No window found with that Id. Make sure you're only spawning via the window manager!",
            )
            .title(app)
    }

    pub fn theme(&self, app: &App, id: Id) -> App::Theme {
        self.windows
            .get(&id)
            .expect(
                "No window found with that Id. Make sure you're only spawning via the window manager!",
            )
            .theme(app)
    }

    pub fn closed(&mut self, id: Id) {
        self.windows.remove(&id);
    }
}

/// Defines the behavior of a window.
pub trait Window<App: Application>: PartialEq + Eq + Clone {
    fn view<'a>(&'a self, app: &'a App) -> iced::Element<'_, App::Message, App::Theme>;
    fn title<'a>(&'a self, app: &'a App) -> String;
    fn theme<'a>(&'a self, app: &'a App) -> App::Theme;
    fn settings(&self) -> window::Settings;
}
