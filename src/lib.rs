pub use iced_multi_window_macros::multi_window;

use iced::{
    multi_window::Application,
    window::{self, Id},
    Command,
};
use std::{collections::HashMap, marker::PhantomData};

/// Takes in a `Window` identifier and returns an enum variant that can be used by various
/// functions.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! window {
    ($window: ident) => {
        crate::WindowUnion::$window($window)
    };
}

/// A struct for managing multiple windows. Keeps track of the windows, their `Id`s, and handles
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

    /// Spawns the given window. Expects a `window!`.
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
            .view(app, id)
    }

    pub fn title(&self, app: &App, id: Id) -> String {
        self.windows
            .get(&id)
            .expect(
                "No window found with that Id. Make sure you're only spawning via the window manager!",
            )
            .title(app, id)
    }

    pub fn theme(&self, app: &App, id: Id) -> App::Theme {
        self.windows
            .get(&id)
            .expect(
                "No window found with that Id. Make sure you're only spawning via the window manager!",
            )
            .theme(app, id)
    }

    pub fn closed(&mut self, id: Id) {
        self.windows.remove(&id);
    }
}

/// Defines the behavior of a window.
pub trait Window<App: Application>: PartialEq + Eq + Clone {
    fn view<'a>(&'a self, app: &'a App, id: Id) -> iced::Element<'_, App::Message, App::Theme>;
    fn title<'a>(&'a self, app: &'a App, id: Id) -> String;
    fn theme<'a>(&'a self, app: &'a App, id: Id) -> App::Theme;
    fn settings(&self) -> window::Settings;
}
