//Intent:
//Bridge is a structural design pattern that lets you split a large class
//or a set of closely related classes into two separate hierarchies—abstraction
//and implementation—which can be developed independently of each other.
//Resource:
//- Design Patterns- Elements of Reusable Object-Oriented Software
//- https://refactoring.guru/design-patterns/bridge
//         ┌──────────────────┐
//         │                  │
//         │   Client         │
//         │                  │
//         └───────┬──────────┘
//                 │
//                 │
//                 │
//                 │
//                 │
//                 │
//                 │                     ┌─────────────────────────────┐                     ┌──────────────────────────┐
//                 │                     │ Abstraction                 │                     │Implementation            │
//                 └────────────────────►│+impl: Implenetation         │  composite          │                          │
//                                       │*operation2()                ├─────────────────────►*method1() *method3()     │
//┌──────────────────────┐               │*operation1()                │                     │*method2()                │
//│ operation1(){        ├ ─ ─ ─ ─ ─ ─ ─ ┴───────────▲─────────────────┘                     └────────────▲─────────────┘
//│   impl->method1()    │                           │                                                    │
//│ }                    │                           │                                                    │
//│                      │                           │                                                    │
//└──────────────────────┘              ┌────────────┴──────────┐                                         │
//                                      │                       │                            ┌────────────┴────────────────────┐
//                                      │ RefinedAbstraction    │                            │                                 │
//                                      │                       │                            │                                 │
//                                      │                       │                            │                                 │
//                                      └───────────────────────┘                            │                                 │
//                                                                                ┌──────────┴───────────┐           ┌─────────┴──────────┐
//                                                                                │ ConcreteImplA        │           │ ConcreteImplB      │
//                                                                                │                      │           │                    │
//                                                                                │                      │           │                    │
//                                                                                └──────────────────────┘           └────────────────────┘

// Example:
// imaging that you're building an application that reads/writes the document,
//  your app supports multi-platform Windows/macOS/Linux.
//  Each OS will have a different mechanism to store files
//  so that we will extract all methods read/write in abstraction,
//  and each os will be the implementor of the FileSystem interface.

type Paths = Vec<String>;

trait FileSystem {
    fn eof(&self) -> String;
    fn resolve(&self, paths: Paths) -> String;
    fn write(&self, path: String, content: String) -> bool;
    fn read(&self, path: String) -> String;
    fn os(&self) -> String;
}

trait FileManager {
    fn copy_file(&self, source: Paths) -> bool;
    fn rename_file(&self, source: Paths, new_name: String) -> bool;
    fn remove_file(&self, source: Paths) -> bool;
    fn new_file(&self, source: Paths, content: String) -> bool;
}

struct ThunarFileManager {
    file_system: Box<dyn FileSystem>,
}

struct Window {}
impl FileSystem for Window {
    fn eof(&self) -> String {
        String::from("\n")
    }

    fn resolve(&self, paths: Paths) -> String {
        paths.join("\\")
    }

    fn write(&self, path: String, content: String) -> bool {
        println!("Window:: Write file at {}", path);
        true
    }

    fn read(&self, path: String) -> String {
        println!("Window:: Read file at {}", path);
        "Window".into()
    }

    fn os(&self) -> String {
        "Window".into()
    }
}

struct MacOS {}
impl FileSystem for MacOS {
    fn eof(&self) -> String {
        "\nn".into()
    }

    fn resolve(&self, paths: Paths) -> String {
        paths.join("/")
    }

    fn write(&self, path: String, content: String) -> bool {
        println!("MacOS:: Write file at {}", path);
        true
    }

    fn read(&self, path: String) -> String {
        println!("MacOS:: Write file at {}", path);
        "MacOS".into()
    }

    fn os(&self) -> String {
        "MacOS".into()
    }
}

struct Linux {}
impl FileSystem for Linux {
    fn eof(&self) -> String {
        "\n".into()
    }

    fn resolve(&self, paths: Paths) -> String {
        paths.join("/")
    }

    fn write(&self, path: String, content: String) -> bool {
        println!("Linux::write at {}", path);
        true
    }

    fn read(&self, path: String) -> String {
        println!("Linux::read at {}", path);
        "Linux".into()
    }

    fn os(&self) -> String {
        todo!()
    }
}

impl FileManager for ThunarFileManager {
    fn copy_file(&self, paths: Paths) -> bool {
        let absolute_path = self.file_system.resolve(paths);
        println!("Copy::{}::{}", self.file_system.os(), absolute_path);
        true
    }

    fn rename_file(&self, paths: Paths, new_name: String) -> bool {
        let absolute_path = self.file_system.resolve(paths);
        println!(
            "Rename::{}::{} -> {}",
            self.file_system.os(),
            absolute_path,
            new_name
        );
        true
    }
    fn remove_file(&self, paths: Paths) -> bool {
        let absolute_path = self.file_system.resolve(paths);
        println!("Remove::{}::{}", self.file_system.os(), &absolute_path);
        true
    }

    fn new_file(&self, paths: Paths, content: String) -> bool {
        let absolute_path = self.file_system.resolve(paths);
        self.file_system.write(absolute_path, content)
    }
}

pub enum OS {
    Window,
    MacOS,
    Linux,
}

fn get_os_file_system(os: OS) -> Box<dyn FileManager> {
    match os {
        OS::Window => Box::new(ThunarFileManager {
            file_system: Box::new(Window {}),
        }),
        OS::MacOS => Box::new(ThunarFileManager {
            file_system: Box::new(MacOS {}),
        }),
        OS::Linux => Box::new(ThunarFileManager {
            file_system: Box::new(Linux {}),
        }),
    }
}

pub fn demo_bridge() {
    let thunar_window = get_os_file_system(OS::Window);
    thunar_window.new_file(vec!["home".into(), "tiny".into()], "this is content".into());
    let thunar_macos = get_os_file_system(OS::MacOS);
    thunar_macos.new_file(vec!["home".into(), "tiny".into()], "this is content".into());
    let thunar_linux = get_os_file_system(OS::Linux);
    thunar_linux.new_file(vec!["home".into(), "tiny".into()], "this is content".into());
}
