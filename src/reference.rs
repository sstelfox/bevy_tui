/// Determines the method used by TUI to run an [`App`]'s [`Schedule`](bevy::ecs::schedule::Schedule).
///
/// TODO: I might want to refactor the event blocking into a separate setting...
///
/// It is used in the [`TuiScheduleRunnerSettings`].
#[derive(Copy, Clone, Debug)]
pub enum RunMode {
    /// Indicates that the [`App`]'s schedule should run only once.
    Once,

    //// Indicates that the [`App`]'s schedule should run only when it receives events from the
    //// terminal (such as resizes, keypresses, etc).
    //EventsOnly,
    /// Indicates that the [`App`]'s schedule should run repeatedly attempting to consistently run
    /// every `wait` interval. Updates will also occur when one or more terminal events are
    /// received.
    Loop {
        /// The maximum [`Duration`] to wait between [`Schedule`](bevy::ecs::schedule::Schedule)
        /// updates before repeating. The run time of the [`App`] update will reduce the duration
        /// of wait. If no wait duration is desired the [`RunMode::EventsOnly`] or
        /// [`RunMode::LoopNoEvents`] should be chosen instead.
        wait: Duration,
    },
    //// Indicates that the [`App`]'s schedule should run repeatedly attempting to consistently run
    //// every `wait` interval. Terminal events will not trigger updates to the app.
    //LoopNoEvents {
    //    /// The maximum [`Duration`] to wait between [`Schedule`](bevy::ecs::schedule::Schedule)
    //    /// updates before repeating. The run time of the [`App`] update will reduce the duration
    //    /// of wait. A value of [`None`] will not wait.
    //    wait: Option<Duration>,
    //},
}

/// The configuration information for the [`TuiScheduleRunnerPlugin`].
///
/// It gets added as a [`Resource`](bevy::ecs::system::Resource) inside of the [`TuiScheduleRunnerPlugin`].
#[derive(Copy, Clone, Default, Resource)]
pub struct TuiScheduleRunnerSettings {
    /// Determines how the [`Schedule`](bevy::ecs::schedule::Schedule) is triggered.
    pub run_mode: RunMode,
}

impl TuiScheduleRunnerSettings {
    //// See [`RunMode::EventsOnly`].
    //#[must_use]
    //pub fn run_evented() -> Self {
    //    Self {
    //        run_mode: RunMode::EventsOnly,
    //    }
    //}

    /// See [`RunMode::Loop`].
    #[must_use]
    pub fn run_loop(wait: Duration) -> Self {
        Self {
            run_mode: RunMode::Loop { wait },
        }
    }

    //// See [`RunMode::LoopNoEvents`].
    //#[must_use]
    //pub fn run_loop_unevented(wait: Duration) -> Self {
    //    Self {
    //        run_mode: RunMode::LoopNoEvents { wait: Some(wait) },
    //    }
    //}

    /// See [`RunMode::Once`].
    #[must_use]
    pub fn run_once() -> Self {
        Self {
            run_mode: RunMode::Once,
        }
    }
}

/// Plugin group that will add the minimum required to run Bevy with a TUI display.
///
/// * [`CorePlugin`](bevy::core::CorePlugin)
/// * [`TimePlugin`](bevy::time::TimePlugin)
/// * [`TuiScheduleRunnerPlugin`](TuiScheduleRunnerPlugin)
///
pub struct TuiPlugins;

impl PluginGroup for TuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
            .add(TimePlugin::default())
            .add(TuiScheduleRunnerPlugin::default())
    }
}

/// Configures an [`App`] to run its [`Schedule`](bevy::ecs::schedule::Schedule) according to a
/// given [`RunMode`] within a TUI.
#[derive(Default)]
pub struct TuiScheduleRunnerPlugin;

impl Plugin for TuiScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        let settings = app
            .world
            .get_resource_or_insert_with(TuiScheduleRunnerSettings::default)
            .to_owned();

        app.set_runner(move |mut app: App| {
            let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

            match settings.run_mode {
                RunMode::Once => {
                    app.update();
                }
                //RunMode::EventsOnly => {
                //    while let Ok(event) = read_term_event() {
                //        process_event(&mut app, event);
                //        process_all_events(&mut app);

                //        let (keep_going, _) = tick(&mut app);
                //        if !keep_going {
                //            return;
                //        }
                //    }
                //},
                RunMode::Loop { wait } => {
                    let mut tick = move |app: &mut App,
                                         wait: Duration|
                          -> Result<Option<Duration>, AppExit> {
                        let start_time = Instant::now();

                        process_all_events(app);

                        if let Some(app_exit_events) =
                            app.world.get_resource_mut::<BevyEvents<AppExit>>()
                        {
                            if let Some(exit) = app_exit_event_reader.iter(&app_exit_events).last()
                            {
                                println!("found app exit event 1");
                                return Err(exit.clone());
                            } else {
                                println!("no app exit event 1");
                            }
                        } else {
                            println!("no exit event reader 1");
                        }

                        app.update();

                        if let Some(app_exit_events) =
                            app.world.get_resource_mut::<BevyEvents<AppExit>>()
                        {
                            if let Some(exit) = app_exit_event_reader.iter(&app_exit_events).last()
                            {
                                println!("found app exit event 1");
                                return Err(exit.clone());
                            } else {
                                println!("no app exit event 1");
                            }
                        } else {
                            println!("no exit event reader 1");
                        }

                        let end_time = Instant::now();
                        let exe_time = end_time - start_time;

                        if exe_time < wait {
                            return Ok(Some(wait - exe_time));
                        }

                        Ok(None)
                    };

                    {
                        while let Ok(delay) = tick(&mut app, wait) {
                            if let Some(delay) = delay {
                                std::thread::sleep(delay);
                            }
                        }
                    }

                    //let mut delay = Duration::from_millis(0);

                    //while let Ok(event_ready) = poll_term_event(delay) {
                    //    if event_ready {
                    //        process_all_events(&mut app);
                    //    }

                    //    let (keep_going, execution_time) = tick(&mut app);
                    //    if !keep_going {
                    //        return;
                    //    }

                    //    delay = wait - execution_time.expect("a time measurement if we intended to keep going");
                    //}
                }
                //RunMode::LoopNoEvents { wait } => {
                //    loop {
                //        process_all_events(&mut app);

                //        let (keep_going, execution_time) = tick(&mut app);
                //        if !keep_going {
                //            return;
                //        }

                //        if let Some(time) = wait {
                //            let delay = time - execution_time.expect("a time measurement if we intended to keep going");
                //            std::thread::sleep(delay);
                //        }
                //    }
                //},
            }
        });
    }
}

fn process_event(app: &mut App, event: TerminalEvent) {
    match event {
        TerminalEvent::Key(key) => match key.code {
            TerminalKeyCode::Char('q') => {
                println!("detected exit command");
                app.world.send_event(AppExit);
            }
            TerminalKeyCode::Esc => {
                crossterm::terminal::disable_raw_mode().expect("disabling raw mode");

                if let Some(mut terminal) = app.world.get_resource_mut::<BevyTerminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>>() {
                        {
                            let term_stdout = terminal.0.backend_mut();
                            term_stdout.queue(crossterm::terminal::LeaveAlternateScreen).expect("returning to normal screen");
                            term_stdout.queue(crossterm::event::DisableMouseCapture).expect("release mouse capture");
                            term_stdout.flush().expect("flush terminal commands");
                        }

                        terminal.0.show_cursor().expect("restoring cursor");
                    }

                std::process::exit(23);
            }
            _ => {
                println!("received unbound terminal key: {key:?}");
            }
        },
        _ => {
            println!("received unknown terminal event: {event:?}");
        }
    }
}
