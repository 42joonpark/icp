use serde::Deserialize;

pub fn jsonize_vec<'a, T>(text: &'a str) -> Result<Vec<T>, serde_json::Error>
where
    T: Deserialize<'a>,
{
    let camp: Vec<T> = serde_json::from_str(text)?;
    Ok(camp)
}

pub fn jsonize<'a, T>(text: &'a str) -> Result<T, serde_json::Error>
where
    T: Deserialize<'a>,
{
    let camp: T = serde_json::from_str(text)?;
    Ok(camp)
}
