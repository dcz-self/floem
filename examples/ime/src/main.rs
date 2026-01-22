pub mod inputs;
pub mod texteditor;

use floem::{
    action::{set_theme, set_window_menu, toggle_global_theme, toggle_window_theme},
    event::{Event, EventListener},
    kurbo::Size,
    menu::*,
    muda::{AboutMetadataBuilder, PredefinedMenuItem},
    new_window,
    prelude::*,
    style::{Background, CursorStyle, Transition},
    theme::StyleThemeExt,
    ui_events::keyboard::{Key, KeyState, KeyboardEvent, Modifiers, NamedKey},
    window::{Theme, WindowConfig, WindowId},
};

fn app_view(window_id: WindowId) -> impl IntoView {
    let tabs: Vec<&'static str> = vec![
        "Input",
        "Text Editor",
    ];

    let create_view = |it: &str| {
        match it {
            "Input" => inputs::text_input_view().into_any(),
            "Text Editor" => texteditor::editor_view().into_any(),
            _ => Label::derived(|| "Not implemented".to_owned()).into_any(),
        }
        .debug_name(it.to_string())
    };

    let tabs = RwSignal::new(tabs);

    let side_bar_list = tabs
        .get()
        .into_iter()
        .map(move |item| {
            item.debug_name(item).style(move |s| {
                s.flex_row()
                    .font_size(18.)
                    .height(36.0)
                    .transition(Background, Transition::ease_in_out(100.millis()))
                    .active(|s| {
                        s.with_theme(|s, t| {
                            s.background(t.primary())
                                .hover(|s| s.background(t.primary_muted()))
                                .border_radius(t.border_radius())
                        })
                    })
                    .hover(|s| s.cursor(CursorStyle::Pointer))
            })
        })
        .list()
        .style(|s| s.flex_col().width(140.0).flex_grow(1.));

    let active_tab = side_bar_list.selection();

    let side_tab_bar = side_bar_list;

    let left_side_bar = side_tab_bar;

    let tab = tab(
        move || Some(active_tab.get().unwrap_or(0)),
        move || tabs.get(),
        |it| *it,
        create_view,
    )
    .debug_name("Active Tab")
    .style(|s| s.flex_col().flex_grow(1.).items_start());

    let tab = tab.scroll().style(|s| s.size_full());

    let view = Stack::horizontal((left_side_bar, tab))
        .style(|s| s.padding(5.0).width_full().height_full().col_gap(5.0))
        .window_title(|| "Widget Gallery".to_owned());

    let file_submenu = |m: SubMenu| {
        m.item("New Window", |i| {
            i.action(move || {
                new_window(app_view, None);
            })
        })
        .separator()
        .item("Close Window", |i| {
            i.action(move || {
                floem::close_window(window_id);
            })
        })
        .item("Quit Widget Gallery", |i| {
            i.action(|| {
                floem::quit_app();
            })
        })
    };

    let widget_submenu = |m: SubMenu| {
        tabs.with(|tabs| {
            tabs.iter().enumerate().fold(m, |menu, (idx, &tab)| {
                menu.item(tab, move |i| i.action(move || active_tab.set(Some(idx))))
            })
        })
    };

    let view_submenu = |m: SubMenu| {
        m.item("Inspector", |i| {
            i.action(|| {
                floem::action::inspect();
            })
        })
        .separator()
        .submenu("Navigate to Widget", widget_submenu)
        .separator()
        .item("Next Tab", |i| {
            i.action(move || {
                let current = active_tab.get().unwrap_or(0);
                let tab_count = tabs.get().len();
                active_tab.set(Some((current + 1) % tab_count));
            })
        })
        .item("Previous Tab", |i| {
            i.action(move || {
                let current = active_tab.get().unwrap_or(0);
                let tab_count = tabs.get().len();
                active_tab.set(if current == 0 {
                    Some(tab_count - 1)
                } else {
                    Some(current - 1)
                });
            })
        })
    };

    let theme_submenu = |m: SubMenu| {
        m.item("Toggle Global Theme", |i| i.action(toggle_global_theme))
            .item("Toggle Window Theme", |i| i.action(toggle_window_theme))
            .separator()
            .item("Set Light Theme", |i| {
                i.action(|| set_theme(Some(Theme::Light)))
            })
            .item("Set Dark Theme", |i| {
                i.action(|| set_theme(Some(Theme::Dark)))
            })
            .item("Follow OS Theme", |i| i.action(|| set_theme(None)))
    };

    let window_submenu = |m: SubMenu| {
        m.item("Open Current Tab in New Window", |i| {
            i.action(move || {
                let name = tabs.with(|tabs| tabs.get(active_tab.get().unwrap_or(0)).copied());
                new_window(
                    move |_| {
                        create_view(name.unwrap_or_default())
                            .scroll()
                            .style(|s| s.size_full())
                    },
                    Some(
                        WindowConfig::default()
                            .size(Size::new(500.0, 300.0))
                            .title(name.unwrap_or_default()),
                    ),
                );
            })
        })
        .separator()
        .item("Show Side Panel", |i| {
            i.checked(true).action(|| {
                println!("Toggle sidebar");
            })
        })
    };

    let help_submenu = |m: SubMenu| {
        m.item("About Widget Gallery", |i| {
            i.action(|| {
                println!("Floem Widget Gallery - A showcase of UI components built with Floem");
            })
        })
        .separator()
        .item("Floem Documentation", |i| {
            i.action(|| {
                println!("Opening Floem documentation...");
            })
        })
        .item("GitHub Repository", |i| {
            i.action(|| {
                println!("Opening GitHub repository...");
            })
        })
    };
    set_window_menu(
        Menu::new()
            .submenu("File", file_submenu)
            .submenu("View", view_submenu)
            .submenu("Window", window_submenu)
            .submenu("Theme", theme_submenu)
            .submenu("Help", help_submenu)
            .submenu("About", |s| {
                s.predefined(&PredefinedMenuItem::about(
                    Some("widget-gallery"),
                    Some(
                        AboutMetadataBuilder::new()
                            .name(Some("widget-gallery"))
                            .license(Some("MIT"))
                            .version(Some("0.1.0"))
                            .copyright(Some("Floem Authors"))
                            .build(),
                    ),
                ))
            }),
    );

    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::Key(KeyboardEvent {
            state: KeyState::Up,
            key,
            modifiers,
            ..
        }) = e
        {
            if *key == Key::Named(NamedKey::F11) {
                floem::action::inspect();
            } else if *key == Key::Character("q".into()) && modifiers.contains(Modifiers::META) {
                floem::quit_app();
            } else if *key == Key::Character("w".into()) && modifiers.contains(Modifiers::META) {
                floem::close_window(window_id);
            }
        }
    })
}

fn main() {
    
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }
    
    floem::Application::new()
        .window(app_view, Some(WindowConfig::default().size((500., 300.))))
        .on_event(|ae| match ae {
            floem::AppEvent::WillTerminate => {
                println!("terminating");
            }
            floem::AppEvent::Reopen {
                has_visible_windows,
            } => {
                if !has_visible_windows {
                    new_window(app_view, None);
                }
            }
        })
        .run();
}
