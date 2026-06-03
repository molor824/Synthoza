#[cfg(test)]
mod tests {
    use core::f32;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use mlua::prelude::*;
    use rtrb::RingBuffer;
    use std::{thread, time::Duration};

    #[test]
    fn test_cpal() {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate() as f32;
        let channels = config.channels() as usize;

        let mut phase = 0.0 as f32;
        let freq = 440.0;
        let amplitude = 0.3;

        let stream = device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| {
                    for frame in data.chunks_mut(channels) {
                        let sample = (phase * f32::consts::TAU).sin() * amplitude;

                        phase = (phase + freq / sample_rate) % 1.0;

                        for channel in frame.iter_mut() {
                            *channel = sample;
                        }
                    }
                },
                |err| eprintln!("Stream error: {err}"),
                None,
            )
            .unwrap();

        stream.play().unwrap();
        thread::sleep(Duration::from_secs(3));
    }

    #[test]
    fn test_mlua() {
        const FOURIER_ITER: usize = 3000;
        const WAVE: &str = r#"
        local phase = 0.0
        
        function process(freq, buffer, len)
            for i = 1, len, CHANNELS do
                local audio = 0.0
                local radian = phase * (math.pi * 2.0)

                for j = 1, FOURIER_ITER do
                    audio = audio + math.sin(radian * j) / j
                end

                buffer[i] = audio
                buffer[i + 1] = audio
            
                phase = (phase + freq / SAMPLE_RATE) % 1.0
            end
        end
        "#;

        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate() as usize;
        let channels = config.channels() as usize;

        let lua = Lua::new();
        let globals = lua.globals();

        globals.set("FOURIER_ITER", FOURIER_ITER).unwrap();
        globals.set("SAMPLE_RATE", sample_rate).unwrap();
        globals.set("CHANNELS", channels).unwrap();

        lua.load(WAVE).exec().unwrap();

        let lua_process = globals.get::<LuaFunction>("process").unwrap();

        let (mut stream_producer, mut stream_consumer) =
            RingBuffer::<f32>::new(channels * sample_rate);

        let stream = device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| {
                    let chunk = match stream_consumer.read_chunk(data.len()) {
                        Ok(ch) => ch,
                        Err(err) => {
                            data.fill(0.0);
                            eprintln!("{err:?}");
                            return;
                        }
                    };
                    let (first, second) = chunk.as_slices();
                    let mid = first.len();
                    data[..mid].copy_from_slice(first);
                    data[mid..].copy_from_slice(second);
                    chunk.commit_all();
                },
                |err| eprintln!("{err}"),
                None,
            )
            .unwrap();

        stream.play().unwrap();

        let freq_start = 200.0;
        let freq_end = 500.0;

        let duration = 10.0;
        let mut time = 0.0;
        let dt = 1.0 / sample_rate as f32;
        let amplitude = 0.2;

        let lua_buffer = lua
            .create_table_with_capacity(sample_rate * channels, 0)
            .unwrap();

        while time <= duration {
            let freq = freq_start + (freq_end - freq_start) * (time / duration);
            let buffer_size = (1.0 / freq * sample_rate as f32).round() as usize;

            for i in 1..=buffer_size {
                lua_buffer.raw_set(i, 0.0).unwrap();
            }
            lua_process
                .call::<()>((freq, &lua_buffer, buffer_size))
                .unwrap();
            let mut buffer: Vec<f32> = Vec::with_capacity(buffer_size);
            for i in 1..=buffer_size {
                buffer.push(lua_buffer.raw_get(i).unwrap());
            }

            for data in &mut buffer {
                *data *= amplitude;
            }

            loop {
                match stream_producer.push_entire_slice(&buffer) {
                    Ok(_) => break,
                    Err(_) => thread::sleep(Duration::from_millis(1)),
                }
            }

            time += buffer_size as f32 * dt;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
