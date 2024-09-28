#[cfg(test)]
mod tests {
    use crate::music::types::QueryType;

    use super::super::Spotify;

    #[tokio::test]
    async fn get_track() {
        dotenv::dotenv().unwrap();

        let track_id = "6tVahG14lCjexVQnYWKgwF";

        let mut spotify = Spotify::new();
        spotify.auth().await.unwrap();
        let track = spotify.get_track_keywords(track_id).await.unwrap();

        assert_eq!(
            "Bring Me The Horizon AURORA - liMOusIne (feat. AURORA) \\\"topic\\\"",
            track
        );
    }

    #[tokio::test]
    async fn get_album() {
        dotenv::dotenv().unwrap();

        let mut spotify = Spotify::new();
        spotify.auth().await.unwrap();

        let res = spotify
            //.extract("https://open.spotify.com/playlist/07jWIh2Zo8RyOieEYUMtWV?si=df706fc1359f4f66")
            .extract("https://open.spotify.com/playlist/1zOSRkS0kFrExnZplQdbo9?si=37904fc5c27e47c8")
            .await
            .unwrap();

        println!("{:#?}", res);

        if let QueryType::KeywordsList(list) = res {
            println!("{}", list.len());
        }
    }
}
