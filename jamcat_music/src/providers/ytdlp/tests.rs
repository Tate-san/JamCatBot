#[cfg(test)]
mod tests {
    use super::super::Ytdl;

    #[tokio::test]
    async fn query_playlist() {
        let playlist_url =
            "https://www.youtube.com/watch?v=CHENRaquRHo&list=PLDisKgcnAC4SqX0mi1J5_gd7-hrIS5yJp";

        let ytdl = Ytdl::new();

        let _items = ytdl.query_playlist(playlist_url).await.unwrap();
    }

    #[tokio::test]
    async fn query() {
        let video_url = "https://www.youtube.com/watch?v=P4bKZT_Eg4A";

        let ytdl = Ytdl::new();

        let _query = ytdl.query(video_url).await.unwrap();

    }
}
