use druid::{
    widget::{Button, Controller, Flex, ViewSwitcher},
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Event, EventCtx, ExtEventSink, Lens,
    PlatformError, Selector, Target, Widget, WidgetExt, WindowDesc,
};
use druid_enums::Matcher;

const CLOSE: Selector = Selector::new("druid-enums.overlay.close");
const COUNT: Selector = Selector::new("druid-enums.overlay.open");

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        println!("delegate: {:?}", cmd);
        if let Some(_) = cmd.get(CLOSE) {
            data.overlay = OverlayState::None;
            true
        } else if let Some(_) = cmd.get(COUNT) {
            data.overlay = OverlayState::Count(CountState::from(data.clone()));
            true
        } else {
            true
        }
    }
}

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = App)] // defaults to AppStateMatcher
enum OverlayState {
    Count(CountState),
    None,
}

#[derive(Clone, Data, Lens)]
struct AppState {
    overlay: OverlayState,
}

impl AppState {
    fn new() -> Self {
        Self {
            overlay: OverlayState::None,
            //overlay: OverlayState::Count(CountState { count: 0 }),
        }
    }
}

#[derive(Clone, Data, Lens, Default)]
struct CountState {
    count: u32,
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui).title("Druid Enums");
    let state = AppState::new();
    let launcher = AppLauncher::with_window(window);

    //let delegate = Delegate { sink: sink.clone() };

    launcher
        .delegate(Delegate)
        .use_simple_logger()
        .launch(state)
}

fn ui() -> impl Widget<AppState> {
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.overlay.clone(),
        |overlay, _data, _env| match overlay {
            OverlayState::None => Box::new(page_ui()),
            _ => Box::new(overlay_ui().lens(AppState::overlay)),
        },
    );

    Flex::column().with_flex_child(view_switcher, 1.)
}

fn overlay_ui() -> impl Widget<OverlayState> {
    // AppState::matcher() or
    App::new().count(count_ui()).controller(OverlayController)
}

fn page_ui() -> impl Widget<AppState> {
    Flex::column().with_child(
        Button::new("Count").on_click(|ctx, _: &mut AppState, _| ctx.submit_command(COUNT, None)),
    )
}

fn count_ui() -> impl Widget<CountState> {
    Flex::column()
        .with_child(
            Button::dynamic(CountState::count_label)
                .on_click(|_, state: &mut CountState, _| state.count += 1),
        )
        .with_spacer(5.0)
        .with_child(Button::new("Close").on_click(|ctx, _, _| ctx.submit_command(CLOSE, None)))
        .center()
}

struct OverlayController;

impl Controller<OverlayState, App> for OverlayController {
    fn event(
        &mut self,
        child: &mut App,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut OverlayState,
        env: &Env,
    ) {
        //match event {
        //Event::Command(cmd) if cmd.is(CLOSE) => {
        //let main_state = cmd.get_unchecked(CLOSE).clone();
        //*data = AppState::Count(main_state);
        //}
        //_ => {}
        //}
        child.event(ctx, event, data, env)
    }
}

impl CountState {
    pub fn count_label(&self, _: &Env) -> String {
        format!("clicked {} times", self.count)
    }
}

impl From<AppState> for CountState {
    fn from(_app_state: AppState) -> Self {
        CountState { count: 0 }
    }
}
