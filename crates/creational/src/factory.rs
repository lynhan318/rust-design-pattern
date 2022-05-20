// # Abstract Factory:
// Intent:
// Provide an interface for creating families of related or dependent objects
// without specifying their concreate classes.

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

#[derive(Debug, Clone, Default)]
pub struct Window {
    pub background: String,
    pub text: String,
    pub widget_name: String,
}
#[derive(Debug, Clone, Default)]
pub struct ScrollBar {
    pub width: i64,
    pub height: i64,
    pub widget_name: String,
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

mod abstract_factory {
    use super::*;
    pub trait WidgetFactory {
        fn create_window(&self) -> Window;
        fn create_scroll_bar(&self) -> ScrollBar;
        fn get_theme_info(&self) -> ThemeInfo;
    }

    pub struct PMWindowWidget {}
    impl WidgetFactory for PMWindowWidget {
        fn create_window(&self) -> Window {
            Window {
                background: "white".into(),
                text: "green".into(),
                widget_name: "PMWindow".to_string(),
            }
        }

        fn create_scroll_bar(&self) -> ScrollBar {
            ScrollBar {
                width: 300,
                height: 300,
                widget_name: "ScrollBar".to_string(),
            }
        }

        fn get_theme_info(&self) -> ThemeInfo {
            ThemeInfo {
                name: "PMWindow Widget".into(),
            }
        }
    }

    pub struct MotifWidget {}
    impl WidgetFactory for MotifWidget {
        fn create_window(&self) -> Window {
            Window {
                background: "green".into(),
                text: "white".into(),
                widget_name: "MotifScollbar".to_string(),
            }
        }

        fn create_scroll_bar(&self) -> ScrollBar {
            ScrollBar {
                width: 200,
                height: 200,
                widget_name: "MotifScollbar".to_string(),
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
}

mod factory_method {
    use super::*;
    trait WidgetFactoryMethod {
        type Object;
        fn create() -> Box<Self::Object>
        where
            Self::Object: Default,
        {
            Box::new(Default::default())
        }
    }

    trait WidgetFactory {
        fn create_scroll_bar(&self) -> Box<ScrollBar>;
        fn create_window(&self) -> Box<Window>;
    }

    impl WidgetFactoryMethod for Window {
        type Object = Window;
        fn create() -> Box<Self::Object> {
            let widget = Window {
                widget_name: "Window".into(),
                ..Window::default()
            };
            Box::new(widget)
        }
    }

    impl WidgetFactoryMethod for ScrollBar {
        type Object = ScrollBar;
        fn create() -> Box<Self::Object> {
            let widget = ScrollBar {
                widget_name: "ScrollBar".into(),
                ..ScrollBar::default()
            };
            Box::new(widget)
        }
    }

    pub struct PMWidgetFactory {}
    impl WidgetFactory for PMWidgetFactory {
        fn create_scroll_bar(&self) -> Box<ScrollBar> {
            ScrollBar::create()
        }

        fn create_window(&self) -> Box<Window> {
            Window::create()
        }
    }
    pub struct MotifWidgetFactory {}
    impl WidgetFactory for MotifWidgetFactory {
        fn create_scroll_bar(&self) -> Box<ScrollBar> {
            ScrollBar::create()
        }

        fn create_window(&self) -> Box<Window> {
            Window::create()
        }
    }
}
