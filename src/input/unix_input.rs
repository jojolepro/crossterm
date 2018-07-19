use input::ITerminalInput;
use std::io;

pub struct UnixInput;

impl UnixInput
{
    pub fn new() -> UnixInput
    {
        UnixInput {}
    }
}

impl ITerminalInput for UnixInput
{
    fn read_char(&self) -> io::Result<String>
    {
        let mut rv = String::new();
        Ok(rv)
    }

    fn read_key(&self) -> io::Result<()>
    {
        let mut rv = String::new();
        Ok(())
    }

    fn read_async(&self)
    {

    }

    fn read_until(&self, delimiter: u8)
    {

    }
}
