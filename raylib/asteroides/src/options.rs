pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ejemplo", about= "Un ejemplo de uso de StructOpt.")]
pub struct Opt{
    #[structopt(short = "w", long = "width", default_value="800")]
    pub width: i32,
    #[structopt(short = "h", long = "height", default_value="480")]
    pub height: i32,
    #[structopt(long = "fps", default_value="60")]
    pub fps: u32,
}

impl Opt {
    #[allow(dead_code)]
    pub fn new() -> Self{
	Opt::from_args()
    }

    pub fn open_window(&self, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread){
	let (mut rl, thread) = raylib::init()
	    .size(self.width, self.height)
	    .title(name)
	    .build();
	rl.set_target_fps(self.fps);
	(rl, thread)
    }
}
