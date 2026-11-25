use nice_plug::{
    formatters::{
        s2v_f32_gain_to_db, s2v_f32_hz_then_khz, v2s_f32_gain_to_db, v2s_f32_hz_then_khz,
    },
    midi::Key,
    prelude::*,
    util::{db_to_gain, db_to_gain_fast, midi_key_to_freq},
};
use std::{f32::consts, sync::Arc};

pub struct MyPlugin {
    params: Arc<MyPluginParams>,
    sample_rate: f32,
    phase: f32,
    midi_key_num: u8,
    midi_note_freq: f32,
    midi_note_gain: Smoother<f32>,
}

#[derive(Params)]
struct MyPluginParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "freq"]
    pub frequency: FloatParam,
    #[id = "usemid"]
    pub use_midi: BoolParam,
}
impl Default for MyPluginParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", db_to_gain(0.0), FloatRange::gain_range(-30.0, 30.0))
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(v2s_f32_gain_to_db(2))
                .with_string_to_value(s2v_f32_gain_to_db()),
            frequency: FloatParam::new(
                "Freq",
                420.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_value_to_string(v2s_f32_hz_then_khz(0))
            .with_string_to_value(s2v_f32_hz_then_khz()),
            use_midi: BoolParam::new("Use MIDI", false),
        }
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(MyPluginParams::default()),
            sample_rate: 1.0,
            phase: 0.0,
            midi_key_num: 0,
            midi_note_freq: 1.0,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.0)),
        }
    }
}

impl MyPlugin {
    fn calculate_sine(&mut self, frequency: f32) -> f32 {
        let phase_delta = frequency / self.sample_rate;
        let sine = (self.phase * consts::TAU).sin();
        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }
}

impl Plugin for MyPlugin {
    const NAME: &'static str = "Synthoza";
    const VENDOR: &'static str = "molor824";
    const URL: &'static str = "https://github.com/molor824/Synthoza";
    const EMAIL: &'static str = "info@example.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type Editor = ();
    type SysExMessage = ();
    type BackgroundTask = ();

    fn activate(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl ActivateContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn reset(&mut self) {
        self.phase = 0.0;
        self.midi_key_num = 0;
        self.midi_note_freq = 1.0;
        self.midi_note_gain.reset(0.0);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            let gain = self.params.gain.smoothed.next();
            let sine = if self.params.use_midi.value() {
                while let Some(event) = next_event {
                    if event.timing() > sample_id as u32 {
                        break;
                    }

                    match event {
                        NoteEvent::NoteOn { key, velocity, .. } => {
                            self.midi_key_num = key.number_or_middle_c();
                            self.midi_note_freq = midi_key_to_freq(self.midi_key_num);
                            self.midi_note_gain.set_target(self.sample_rate, velocity);
                        }
                        NoteEvent::NoteOff { key, .. } if key == Key::Number(self.midi_key_num) => {
                            self.midi_note_gain.set_target(self.sample_rate, 0.0);
                        }
                        NoteEvent::PolyPressure { key, pressure, .. }
                            if key == Key::Number(self.midi_key_num) =>
                        {
                            self.midi_note_gain.set_target(self.sample_rate, pressure);
                        }
                        _ => {}
                    }
                    next_event = context.next_event();
                }
                self.calculate_sine(self.midi_note_freq) * self.midi_note_gain.next()
            } else {
                let frequency = self.params.frequency.smoothed.next();
                self.calculate_sine(frequency)
            };

            for sample in channel_samples {
                *sample = sine * db_to_gain_fast(gain);
            }
        }

        ProcessStatus::KeepAlive
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for MyPlugin {
    const CLAP_ID: &'static str = "synthoza";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Make music via programming.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

nice_export_clap!(MyPlugin);
