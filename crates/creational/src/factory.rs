// #Problem:
// create a windows with multiples widget, have different look and feel by
// scrollbar and windows.
//                                  ┌─────────────────┐
//                                  │   WidgetFactory │                                   ┌──────────┐
//                                  ├─────────────────┘◄──────────────────────────────────┤   Client │
//                                  │  createScrollBar()                                  └─────────┬┘
//                                  │createWindow()   │                                             │
//                                  └─────────────────┘                                             │
//                                                                                                  │
//                                                                        ┌───────────┐             │
//                                                                        │ ProductA  ◄─────────────┤
//                                                                        └─────▲─────┘             │
//                                                                              │                   │
//             ┌───────────────────┐         ┌───────────────┬────────┐         │                   │
//             │ConcreteFactoryB   │         │ConcreteFactoryA        │         │                   │
//             ├───────────────────┤         ├───────────────┘   ┌────▼─────┬───┴─────┬───────────┐ │
//             │createProductA()   │         │createProductA()   │ProductA1 │         │ProductA2  ┼────────────┐
//             │createProductB()   │         │createProductB()   └──────────┘         └───────────┘ │          │
//             └────────▲──────▲───┘         └─────────┬─────┘                                      │          │
//                      │      │                       │                                            │          │
//                      │      │                       │                                            │          │
//                      │      │                       │                                            │          │
//                      │      │                       │                                            │          │
//                      │      │                       │                   ┌──────────┐             │          │
//                      │      │                       │                   │ ProductB │◄────────────┘          │
//                      │      │                       │                   └──────────┘                        │
//                      │      │                       │                       |      |                        │
//                      │      │                       │                       |      |                        │
//                      │      │                       │                       |      |                        │
//                      │      │                       │           ┌────────────┐    ┌──────────────┐          │
//                      │      │                       └───────────►ProductB1   │    │ProductB2     │          │
//                      │      │                                   └────────────┘    └─────┬────────┘          │
//                      │      │                                                           │                   │
//                      │      │                                                           │                   │
//                      │      │                                                           │                   │
//                      │      │                                                           │                   │
//                      │      │                                                           │                   │
//                       ──────┼───────────────────────────────────────────────────────────┘                   │
//                             │                                                                               │
//                             └───────────────────────────────────────────────────────────────────────────────
//

#[derive(Debug, Clone)]
pub struct Window {
    pub background: String,
    pub text: String,
}
#[derive(Debug, Clone)]
pub struct ScrollBar {
    pub width: i64,
    pub height: i64,
}
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    pub name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Themes {
    PMWindow,
    Motif,
}

pub trait WidgetFactory {
    fn create_window(&self) -> Window;
    fn create_scroll_bar(&self) -> ScrollBar;
    fn get_theme_info(&self) -> ThemeInfo;
}

struct PMWindowWidget {}
impl WidgetFactory for PMWindowWidget {
    fn create_window(&self) -> Window {
        Window {
            background: "white".into(),
            text: "green".into(),
        }
    }

    fn create_scroll_bar(&self) -> ScrollBar {
        ScrollBar {
            width: 300,
            height: 300,
        }
    }

    fn get_theme_info(&self) -> ThemeInfo {
        ThemeInfo {
            name: "PMWindow Widget".into(),
        }
    }
}

struct MotifWidget {}
impl WidgetFactory for MotifWidget {
    fn create_window(&self) -> Window {
        Window {
            background: "green".into(),
            text: "white".into(),
        }
    }

    fn create_scroll_bar(&self) -> ScrollBar {
        ScrollBar {
            width: 200,
            height: 200,
        }
    }
    fn get_theme_info(&self) -> ThemeInfo {
        ThemeInfo {
            name: "Motif Widget".into(),
        }
    }
}
// https://doc.rust-lang.org/rust-by-example/trait/dyn.htm
pub fn get_widget_factory(theme: Themes) -> Box<dyn WidgetFactory> {
    match theme {
        Themes::PMWindow => Box::new(PMWindowWidget {}),
        Themes::Motif => Box::new(MotifWidget {}),
    }
}
