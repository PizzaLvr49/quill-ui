use std::{
    cell::RefCell,
    num::NonZeroU8,
    process::{ExitCode, Termination},
    rc::{Rc, Weak},
};

use async_executor::LocalExecutor;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

pub struct Application {
    inner: Rc<RefCell<App>>,
}

impl Application {
    #[must_use]
    pub fn new() -> Self {
        let inner = Rc::new(RefCell::new(App::new()));

        inner.borrow_mut().this = Rc::downgrade(&inner);

        Self { inner }
    }

    pub fn run(self, f: impl FnOnce(&mut App)) -> ApplicationExit {
        let mut app = self.inner.borrow_mut();

        f(&mut app);

        ApplicationExit::Success
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

pub struct App {
    this: Weak<RefCell<Self>>,
    executor: LocalExecutor<'static>,
}

impl App {
    #[must_use]
    pub const fn executor(&self) -> &LocalExecutor<'static> {
        &self.executor
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            this: Weak::new(),
            executor: LocalExecutor::new(),
        }
    }

    pub fn weak(&self) -> Weak<RefCell<Self>> {
        self.this.clone()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // no-op
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        // todo
    }
}

pub enum ApplicationExit {
    Success,
    Err(NonZeroU8),
}

impl Termination for ApplicationExit {
    fn report(self) -> ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::Err(code) => ExitCode::from(code.get()),
        }
    }
}
