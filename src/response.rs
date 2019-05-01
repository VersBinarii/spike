#[derive(Debug, Serialize)]
pub struct SpikeResponse<T> {
    data: T,
}

impl<T> SpikeResponse<T>
where
    T: serde::ser::Serialize,
{
    pub fn from_data(data: T) -> Self {
        Self { data }
    }
}
