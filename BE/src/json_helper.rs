use serde::{Serialize, Serializer};

pub fn serialize_f64_2dp<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded = (value * 100.0).round() / 100.0;
    serializer.serialize_f64(rounded)
}

pub fn serialize_vec_f64_2dp<S>(values: &Vec<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded: Vec<f64> = values
        .iter()
        .map(|v| (v * 100.0).round() / 100.0)
        .collect();
    rounded.serialize(serializer)
}