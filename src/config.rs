pub const ARENA_WIDTH: f32 = 480.0;
pub const ARENA_HEIGHT: f32 = 640.0;

pub const BACKGROUND_SHADER: &str = "shaders/background.wgsl";

pub const FONT_FIRA_MONO: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_FIRA_SANS: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_ARCADE: &str = "fonts/Arcade.ttf";

pub const PLAYER_SPRITE: &str = "sprites/player.png";
pub const ENEMY_SPRITE: &str = "sprites/enemy.png";
pub const BALL_SPRITE: &str = "sprites/ball.png";
pub const HINT_SPRITE: &str = "sprites/hint.png";
pub const DEATH_SPRITE: &str = "sprites/death.png";

pub const HIT_AUDIO: &str = "audios/hit.flac";
pub const MISS_AUDIO: &str = "audios/miss.flac";
pub const EXPLOSION_AUDIO: &str = "audios/explosion.flac";
pub const LOSE_AUDIO: &str = "audios/lose.flac";

pub const IMPACT_AUDIOS: [&str; 4] = [
    "audios/impacts/impact-d.wav",
    "audios/impacts/impact-e.wav",
    "audios/impacts/impact-f.wav",
    "audios/impacts/impact-g.wav",
];
pub const MAX_IMPACT_AUDIO_CHANNELS: u32 = 10;

pub const PREDICT_SIZE: usize = 100;
pub const PREDICT_TIME_STEP: f64 = 0.01;
pub const AI_TIME_STEP: f64 = 0.1;

pub const PHYSICS_REST_SPEED: f32 = 100.0;
pub const PHYSICS_TIME_STEP: f64 = 1.0 / 120.0;

pub const PADDLE_WIDTH: f32 = 96.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const BALL_SIZE: f32 = 16.0;

pub const PLAYER_MAX_SPEED: f32 = 2000.0;
pub const PLAYER_ASSIST_SPEED: f32 = 200.0;
pub const PLAYER_ASSIST_SPEED_THRESHOLD: f32 = -1000.0;

pub const ENEMY_MIN_SPEED: f32 = 500.0;
pub const ENEMY_MAX_SPEED: f32 = 3000.0;
pub const ENEMY_NORMAL_SPEED: f32 = 1500.0;
pub const ENEMY_HIT_SPEED_THRESHOLD: f32 = -0.0;
pub const ENEMY_DAMP: f32 = 20.0;

pub const BALL_MAX_SPEED: f32 = 3000.0;
pub const MAX_DAMAGE: f32 = 2000.0;

pub const MIN_BOUNCE_AUDIO_SPEED: f32 = 500.0;
pub const MAX_BOUNCE_AUDIO_SPEED: f32 = 2500.0;

pub const DEATH_EFFECT_SPEED: f32 = 50.0;
