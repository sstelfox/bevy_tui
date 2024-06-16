use std::time::{Duration, Instant};

use bevy::app::{App, AppExit};
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::ecs::system::Resource;
use crossterm::event::{poll as poll_term, read as read_term};

use crate::input::event_handler;
use crate::BevyTuiError;

/// By default the loop will target 4 FPS
const DEFAULT_LOOP_DELAY: Duration = Duration::from_millis(250);

#[derive(Resource)]
pub(crate) struct TuiPersistentState {
    first_run: bool,
    last_update: Instant,
    timeout_reached: bool,
}

impl TuiPersistentState {
    fn is_first_run(&self) -> bool {
        self.first_run
    }

    fn mark_completed_tick(&mut self) {
        self.first_run = false;
        self.last_update = Instant::now();
    }
}

impl Default for TuiPersistentState {
    fn default() -> Self {
        Self {
            first_run: true,
            last_update: Instant::now(),
            timeout_reached: false,
        }
    }
}

fn tick(
    app: &mut App,
    app_exit_event_reader: &mut ManualEventReader<AppExit>,
) -> Result<Option<AppExit>, BevyTuiError> {
    // The app needs to tick once to allow the startup system to setup the terminal. We delay any
    // event processing until this is available otherwise this would become a blocking call until
    // an event is received.
    let first_run = app
        .world_mut()
        .resource::<TuiPersistentState>()
        .is_first_run();

    if !first_run {
        // todo: need to adjust this delay based on how long the last loop took
        let events_available = poll_term(DEFAULT_LOOP_DELAY)?;

        if events_available {
            // Read all of the available events all at once
            while poll_term(Duration::from_secs(0))? {
                event_handler(app, read_term()?);
            }
        }

        // Indicate that this tick was triggered by the timeout and not by an event
        app.world_mut()
            .resource_mut::<TuiPersistentState>()
            .timeout_reached = !events_available;
    }

    app.update();
    app.world_mut()
        .resource_mut::<TuiPersistentState>()
        .mark_completed_tick();

    let Some(event_queue) = app.world_mut().get_resource::<Events<AppExit>>() else {
        return Err(BevyTuiError::MissingExitEventQueue);
    };

    // todo(sstelfox): Need to return the first failure exit code here, if at least one was
    // encountered but no error codes were returned we should return the success code.
    if let Some(exit) = app_exit_event_reader.read(event_queue).last() {
        return Ok(Some(exit.clone()));
    }

    Ok(None)
}

pub(crate) fn tui_schedule_runner(mut app: App) -> AppExit {
    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    loop {
        if let Ok(Some(exit)) = tick(&mut app, &mut app_exit_event_reader) {
            return exit;
        }
    }
}
