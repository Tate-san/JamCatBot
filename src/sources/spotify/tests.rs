#[cfg(test)]
mod tests {
    use super::super::Spotify;

    #[tokio::test]
    async fn get_track() {
        dotenv::dotenv().unwrap();

        let track_id = "6tVahG14lCjexVQnYWKgwF";

        let mut spotify = Spotify::new();
        spotify.auth().await.unwrap();
        let track = spotify.get_track_keywords(track_id).await.unwrap();

        assert_eq!(
            "Bring Me The Horizon AURORA - liMOusIne (feat. AURORA)",
            track
        );
    }
}
