//INTENT:The basic idea of the Command pattern is to separate out actions
//into its own objects and pass them as parameters.
//using trait object

//REFERENCE: https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
//
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}
pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "execute create table"
    }

    fn rollback(&self) -> &str {
        "execute rollback"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "execute add field"
    }

    fn rollback(&self) -> &str {
        "roll back field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
}
