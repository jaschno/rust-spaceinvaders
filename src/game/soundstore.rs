use quad_snd::{AudioContext, PlaySoundParams, Sound};

pub struct SoundStore {
    audio_context: AudioContext,
    hit_sound: Sound,
    dead_sound: Sound,
    win_sound: Sound,
    background_music: Sound
} 

impl SoundStore {
    pub fn new() -> Self {
        let audio_context = AudioContext::new();

        Self {
            hit_sound: Sound::load(&audio_context, include_bytes!("../../assets/hit.wav")),
            dead_sound: Sound::load(&audio_context, include_bytes!("../../assets/dead.wav")),
            win_sound: Sound::load(&audio_context, include_bytes!("../../assets/win.wav")),
            background_music: Sound::load(&audio_context, include_bytes!("../../assets/background.wav")),
            audio_context,
        }
    }

    pub fn play_hit_sound(&self) {
        self.hit_sound.play(&self.audio_context, Default::default());
    }

    pub fn play_dead_sound(&self) {
        self.dead_sound.play(&self.audio_context, Default::default());
    }

    pub fn play_win_sound(&self) {
        self.win_sound.play(&self.audio_context, Default::default());
    }

    pub fn play_background_music(&self) {
        self.background_music.play(&self.audio_context, PlaySoundParams { looped: true, volume: 1. });
    }

    pub fn stop_background_music(&self) {
        self.background_music.stop(&self.audio_context);
    }
}
