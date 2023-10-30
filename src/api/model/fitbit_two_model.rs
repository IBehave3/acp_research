use serde::Deserialize;

#[derive(Deserialize)]
pub struct FitbitTwoQueryParameters {
    pub verify: String,
}
