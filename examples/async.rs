use std::{collections::HashMap, future::Future, sync::Mutex};

fn main() {
    spawn(async {
        let mut scores = UserScore(Mutex::new(HashMap::new()));
        scores.increment("Ryan").await;
    })
}

struct UserScore(Mutex<HashMap<String, u8>>);

impl UserScore {
    async fn increment(&mut self, name: &str) {
        let mut scores = self.0.lock().unwrap();
        let score = scores.entry(name.to_owned()).or_default();
        store(name, *score).await;
        *score += 1;
    }
}

pub async fn store(name: &str, score: u8) {
    todo!("Store {name} in the database with {score}")
}

pub fn spawn<T>(_future: T)
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    todo!("This is (almost) the same signature as tokio's spawn function")
}
