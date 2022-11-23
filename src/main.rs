use bracket_lib::prelude::*;

mod map;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Crawler of Dungeons")
        .build()?;
    
    main_loop(context, State{})
}

struct State{}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}