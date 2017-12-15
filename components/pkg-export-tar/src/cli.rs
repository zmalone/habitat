use clap::{App};

#[derive(Clone)]
pub struct Cli<'a, 'b>
where
    'a: 'b,
{
    pub app: App<'a, 'b>,
}


