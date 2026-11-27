#[derive(Debug, Clone)]
pub enum Message {
    Poll,
    UpdateGain(f32),
}
