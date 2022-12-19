use tcod::colors::*;
use tcod::console::*;


const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 70;
//60 fps max
const LIMIT_FPS: i32 = 60;

const MAP_WIDTH: i32 = 90;
const MAP_HEIGHT: i32 = 65;
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150, 
};

struct Tcod {
    root: Root,
    con: Offscreen,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color:Color) -> Self {
        Object { x, y, char, color }
    }
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self. y += dy;
    }
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn main(){
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("lucida12x12_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rogalike Game")
        .init();

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    
    let mut objects = [player, npc];

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    while !tcod.root.window_closed(){
        tcod.con.clear();

        tcod.root.flush();

     //   tcod.root.wait_for_keypress(true);
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0,0),
            1.0,
            1.0,
    
        );
        if exit {
            break;
        }
        for object in &objects {
            object.draw(&mut tcod.con);
        }
    }



}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { 
            code: Enter,
            alt: true,
            ..
        } => {    
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,

        //movement
        Key { code: Up, .. } => player.move_by (0, -1),
        Key { code: Down, .. } => player.move_by (0, 1),
        Key { code: Left, .. } => player.move_by (-1, 0),
        Key { code: Right, .. } => player.move_by (1, 0),

        _ => {}
    }
    false

}