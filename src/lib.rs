//! # `iced-multi-window`
//!
//! Utilities for managing many windows in an `iced` application.
//!
//! ## Goals
//!
//! Working with multiple windows in iced can become quite painful quite quickly. If you want to introduce a window type with unique behavior, you may have to make additions in more than five places accross your codebase. Oversights are easy, and most of the mistakes you can make aren't caught by the compiler. This library seeks to ease this experince by making defining and working with multiple windows simpler, more intuitive, and harder to screw up.
//!
//! ## Usage
//!
//! The first step is to define the windows that will appear in your app. This is done by creating a corresponding struct and implementing the `Window` trait for it. This trait will describe the logic behind that window's content, title, and theme, as well as defining its spawn-time settings.
//!
//! Next, add a `WindowManager` to your app's state. It keeps track of all of the `Id`s and corresponding `Window`s that are currently open. It also provides `view`, `theme`, and `title` methods that return the proper output for the specified `Id`.
//!
//! You have to manually inform the `WindowManager` when a window is closed. This can be done by subscribing to `iced::window::close_events()` and passing the `Id` of each closed window to `WindowManager::was_closed()`.

use iced::{
    window::{self, Id},
    Element, Task,
};
use std::collections::HashMap;

#[allow(private_bounds)]
pub trait Window<App, Theme, Message, Renderer = iced::Renderer>:
    Send + std::fmt::Debug + WindowClone<App, Theme, Message, Renderer>
{
    fn view<'a>(&self, app: &'a App) -> iced::Element<'a, Message, Theme, Renderer>;
    fn title(&self, app: &App) -> String;
    fn theme(&self, app: &App) -> Theme;
    fn settings(&self) -> window::Settings;
    fn id(&self) -> &'static str;
    fn eq(&self, other: &dyn Window<App, Theme, Message, Renderer>) -> bool {
        self.id() == other.id()
    }
}

trait WindowClone<App, Theme, Message, Renderer> {
    fn clone_box(&self) -> Box<dyn Window<App, Theme, Message, Renderer>>;
}

impl<App, Theme, Message, Renderer> Clone for Box<dyn Window<App, Theme, Message, Renderer>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<App, Theme, Message, Renderer, T: 'static + Window<App, Theme, Message, Renderer> + Clone>
    WindowClone<App, Theme, Message, Renderer> for T
{
    fn clone_box(&self) -> Box<dyn Window<App, Theme, Message, Renderer>> {
        Box::new(self.clone())
    }
}

pub struct WindowManager<App, Theme, Message, Renderer = iced::Renderer> {
    windows: HashMap<Id, Box<dyn Window<App, Theme, Message, Renderer>>>,
}

impl<App, Theme, Message, Renderer> WindowManager<App, Theme, Message, Renderer> {
    /// Returns the window associated with the given Id, panicking if it doesn't exist.
    #[allow(clippy::borrowed_box)]
    fn get(&self, id: Id) -> &Box<dyn Window<App, Theme, Message, Renderer>> {
        self.windows
            .get(&id)
            .expect("No window found with given Id")
    }

    pub fn view<'a>(&self, app: &'a App, id: Id) -> Element<'a, Message, Theme, Renderer> {
        self.get(id).view(app)
    }

    pub fn title(&self, app: &App, id: Id) -> String {
        self.get(id).title(app)
    }

    pub fn theme(&self, app: &App, id: Id) -> Theme {
        self.get(id).theme(app)
    }

    pub fn open(
        &mut self,
        window: Box<dyn Window<App, Theme, Message, Renderer>>,
    ) -> (Id, Task<Id>) {
        let (id, task) = window::open(window.settings());
        self.windows.insert(id, window);
        (id, task)
    }

    pub fn close_all(&mut self) -> Task<Id> {
        let mut tasks = Vec::new();
        for id in self.windows.keys() {
            tasks.push(window::close(*id));
        }
        Task::batch(tasks)
    }

    /// Checks for any open instances of the given window.
    pub fn any_of(&self, window: &dyn Window<App, Theme, Message, Renderer>) -> bool {
        self.windows.values().any(|w| w.eq(window))
    }

    /// Updates internal state to reflect that the given window Id  was closed.
    pub fn was_closed(&mut self, id: Id) {
        self.windows.remove(&id);
    }

    /// Returns all instances of the given window and their associated Ids.
    #[allow(clippy::type_complexity)]
    pub fn instances_of(
        &self,
        window: &dyn Window<App, Theme, Message, Renderer>,
    ) -> Vec<(&Id, &Box<dyn Window<App, Theme, Message, Renderer>>)> {
        self.windows.iter().filter(|(_, w)| w.eq(window)).collect()
    }

    pub fn empty(&self) -> bool {
        self.windows.is_empty()
    }
}

impl<App, Theme, Message, Renderer> Default for WindowManager<App, Theme, Message, Renderer> {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
}
