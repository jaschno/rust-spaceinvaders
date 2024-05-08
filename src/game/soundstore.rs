use quad_snd::{AudioContext, PlaySoundParams, Sound};

pub struct SoundStore {
    audio_context: AudioContext,
    hit_sound: Option<Sound>,
    dead_sound: Option<Sound>,
    win_sound: Option<Sound>,
    background_music: Option<Sound>
} 

impl SoundStore {
    pub fn new() -> Self {
        let mut result = Self {
            audio_context: AudioContext::new(),
            hit_sound: None,
            dead_sound: None,
            win_sound: None,
            background_music: None  
        };
        result.win_sound = Some(Sound::load(&result.audio_context, include_bytes!("..\\..\\assets\\win.wav")));
        result.dead_sound = Some(Sound::load(&result.audio_context, include_bytes!("..\\..\\assets\\dead.wav")));
        result.hit_sound = Some(Sound::load(&result.audio_context, include_bytes!("..\\..\\assets\\hit.wav")));
        result.background_music = Some(Sound::load(&result.audio_context, include_bytes!("..\\..\\assets\\background.wav")));
        return result;
    }

    pub fn play_hit_sound(&self) {
        self.hit_sound.as_ref().unwrap().play(&self.audio_context, Default::default());
    }

    pub fn play_dead_sound(&self) {
        self.dead_sound.as_ref().unwrap().play(&self.audio_context, Default::default());
    }

    pub fn play_win_sound(&self) {
        self.win_sound.as_ref().unwrap().play(&self.audio_context, Default::default());
    }

    pub fn play_background_music(&self) {
        self.background_music.as_ref().unwrap().play(&self.audio_context, PlaySoundParams { looped: true, volume: 1. });
    }

    pub fn stop_background_music(&self) {
        self.background_music.as_ref().unwrap().stop(&self.audio_context);
    }
}
