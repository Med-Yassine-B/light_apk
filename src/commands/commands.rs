pub struct Command{
    pub command:String,
    pub _usage:String,
    pub callback:fn(Vec<String>)
}
pub struct CmdRegistrer{
    commands:Vec<Command>
}
impl CmdRegistrer {
    pub fn new()->Self{
        Self{
            commands:Vec::new(),
        }
    }
    pub fn add_command(&mut self,command:Command){
       self.commands.push(command);
    } 
    pub fn check_command(&self ,cmd:String,args:Vec<String>){
       for command in &self.commands{
           if command.command==cmd {
               (command.callback)(args);
               return;
           }
       }
       println!("{cmd}: Not found! try `light_apk --help`")
    }
}

