#[macro_use]
extern crate shrinkwraprs;

mod sorting_array;
mod tools;

use nannou::draw::Draw;
use nannou::prelude::*;
use nannou_audio::Buffer;

use crate::sorting_array::{SleepTimes, DisplayMode, QuickSortType, SortArray, SortInstruction, audio::Audio};

use std::f32::consts::PI;
use std::f64::consts::PI as PIf64;
use std::io::{self, Read};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

const CONFIG_FILE_LOCATION: &str = "./config.yaml";

pub const TWO_PI: f32 = 2.0 * PI;
pub const DEFAULT_DATA_LEN: usize = 200;
const SOUND_DURATION: Duration = Duration::from_millis(100);


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    arrays: Vec<SortArray>,
    current_display_mode: DisplayMode,
    window_dims: (f32, f32),
    audio_stream: nannou_audio::Stream<Audio>,
    audio_time_started: Option<Instant>,

    array_len: usize,
    multi_array_len: usize,
    sleep_times: Arc<SleepTimes>,
    radix_base: usize,
    quicksort_partition_type: QuickSortType,
}

impl Model {
    fn new() -> io::Result<Self> {
        use yaml_rust::YamlLoader;
        use std::fs;

        // Load config file.
        let mut conf_file_string = String::new();
        fs::File::open(CONFIG_FILE_LOCATION)?
            .read_to_string(&mut conf_file_string)?;

        let confs = YamlLoader::load_from_str(&conf_file_string).unwrap();
        if confs.len() == 0 { panic!("Error: Config file is empty.") }
        let conf = &confs[0];

        let len = conf["array_length"].as_i64()
            .expect("Could not parse array_length from config file.") as usize;
        let multi_len = conf["multi_array_length"].as_i64()
            .expect("Could not parse multi_array_length from config file.") as usize;
        let maximum_pitch = conf["maximum_pitch"].as_f64()
            .expect("Could not parse maximum_pitch field in config as a float.");
        let minimum_pitch = conf["minimum_pitch"].as_f64()
            .expect("Could not parse minimum_pitch field in config as a float.");
        let sleep_times = Arc::new(SleepTimes::from(conf));
        let radix_base = conf["radix_lsd_base"].as_i64()
            .expect("Could not parse radix_lsd_base as an integer.") as usize;
        let quicksort_partition_type = QuickSortType::from_str(
            conf["quicksort_partitioning"].as_str().expect("Could not parse quicksort_partitioning field in config as a string.")
        ).unwrap();

        println!("Config file loaded.");

        // Load audio.
        let audio_host = nannou_audio::Host::new();

        let audio_obj = Audio::new(minimum_pitch, maximum_pitch);
        let stream = audio_host
            .new_output_stream(audio_obj)
            .render(audio)
            .build()
            .unwrap();

        stream.pause().unwrap();

        Ok(Self {
            arrays: vec![SortArray::new(
                len,                
                Arc::clone(&sleep_times),
            )],
            current_display_mode: DisplayMode::Bars,
            window_dims: (0.0, 0.0),
            audio_stream: stream,
            audio_time_started: None,
            array_len: len,
            multi_array_len: multi_len,
            sleep_times,
            radix_base,
            quicksort_partition_type,
        })
    }

    // Sends instruction to all arrays
    fn instruction(&mut self, instruction: SortInstruction) {
        for arr in self.arrays.iter_mut() {
            arr.instruction(instruction);
        }
    }

    fn display(&self, draw: &Draw, transform: (f32, f32)) {
        for (i, arr) in self.arrays.iter().enumerate() {
            arr.display(
                draw,
                i,
                self.arrays.len(),
                arr.len(),
                self.current_display_mode,
                self.window_dims,
                transform,
            );
        }
    }

    fn set_to_single_array(&mut self) {
        self.arrays.clear();
        self.array_len = DEFAULT_DATA_LEN;
        self.arrays.push(SortArray::new(
            self.array_len,
            self.sleep_times.clone(),
        ));
    }

    fn set_to_multi_array(&mut self, len: usize) {
        self.arrays.clear();
        for _ in 0..len {
            self.arrays.push(SortArray::new(
                self.multi_array_len,
                self.sleep_times.clone(),
            ));
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model::new().expect("Could not make model.")
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window_rect = app.window_rect();
    model.window_dims = (window_rect.w(), window_rect.h());

    if model.audio_stream.is_playing() {
        if let Some(time_playing) = model.audio_time_started {
            if time_playing.elapsed() >= SOUND_DURATION {
                model.audio_stream.pause().unwrap();
                model.audio_time_started = None;
            }
        }
    }

    if model.arrays.len() == 1 {
        let mut write = model.arrays[0].data.write().unwrap();
        if write.should_play_sound {
            if let Some(index) = write.active {
                let ratio = write[index] as f64/write.max_val as f64;
                model.audio_stream.send(move |audio| {
                    audio.hz = audio.min_hz + (audio.max_hz - audio.min_hz) * ratio;    // Interpolate
                }).unwrap();

                model.audio_stream.play().unwrap();
                model.audio_time_started = Some(Instant::now());

                write.should_play_sound = false;
            }
        }
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        // Keyboard events
        KeyPressed(key) => {
            match key {
                Key::S => model.instruction(SortInstruction::Shuffle(3)),
                Key::R => model.instruction(SortInstruction::Reset),
                Key::I => model.instruction(SortInstruction::Reverse),

                Key::C | Key::B | Key::D => {
                    if model.arrays.len() > 1 {
                        model.set_to_single_array();
                    }

                    match key {
                        Key::C => model.current_display_mode = DisplayMode::Circle,
                        Key::B => model.current_display_mode = DisplayMode::Bars,
                        Key::D => model.current_display_mode = DisplayMode::Dots,
                        // Key::L => model.current_display_mode = DisplayMode::Line,
                        _ => (),
                    }
                }
                Key::P => {
                    // Pixel display mode (multi-array)
                    model.array_len = model.multi_array_len;
                    // Make it so that each pixel is square.
                    let pixel_size = model.window_dims.0 / model.array_len as f32;
                    let array_num = (model.window_dims.1 / pixel_size).floor() as usize;

                    model.set_to_multi_array(array_num);
                    model.current_display_mode = DisplayMode::Pixels;
                }
                Key::Q => model.instruction(SortInstruction::Stop),

                Key::Key1 => model.instruction(SortInstruction::BubbleSort),
                Key::Key2 => model.instruction(SortInstruction::CocktailShakerSort),
                Key::Key3 => model.instruction(SortInstruction::InsertionSort),
                Key::Key4 => model.instruction(SortInstruction::SelectionSort),
                Key::Key5 => model.instruction(SortInstruction::ShellSort),
                Key::Key6 => model.instruction(SortInstruction::CombSort),
                Key::Key7 => model.instruction(SortInstruction::QuickSort(model.quicksort_partition_type)),
                Key::Key8 => model.instruction(SortInstruction::MergeSort),
                Key::Key9 => model.instruction(SortInstruction::RadixSort(model.radix_base)),
                _ => (),
            }
        }
        KeyReleased(_key) => {}

        // Mouse events
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseWheel(_amount, _phase) => {}
        MouseEntered => {}
        MouseExited => {}

        // Touch events
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}

        // Window events
        Moved(_pos) => {}
        Resized(_size) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let transformation = (-model.window_dims.0 / 2.0, -model.window_dims.1 / 2.0); // Axis starts bottom left corner

    let draw = app.draw();
    draw.background().color(BLACK);

    model.display(&draw, transformation);

    draw.to_frame(app, &frame).unwrap();
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    //println!("Rendering audio.");
    let sample_rate = buffer.sample_rate() as f64;

    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PIf64 * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        if sine_amp >= 0.0 {    // hsin waveform
            for channel in frame {
                *channel = sine_amp * audio.volume;
            }
        }
    }
}