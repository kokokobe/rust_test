#[cfg(test)]
mod tests {
    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    use futures::executor::block_on;

    #[test]
    fn primer() {
        async fn hello_world() {
            println!("hello, world!");
        }
        // Nothing is printed
        let future = hello_world();
        // `future` is run and "hello, world!" is printed
        block_on(future);
    }

    #[test]
    fn primer2() {
        async fn learn_song() -> Song { /* ... */ }
        async fn sing_song(song: Song) { /* ... */ }
        async fn dance() { /* ... */ }
        async fn learn_and_sing() {
            // Wait until the song has been learned before singing it.
            // We use `.await` here rather than `block_on` to prevent blocking the
            // thread, which makes it possible to `dance` at the same time.
            let song = learn_song().await;
            sing_song(song).await;
        }
        async fn async_main() {
            let f1 = learn_and_sing();
            let f2 = dance();
            // `join!` is like `.await` but can wait for multiple futures concurrently.
            // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
            // future will take over the current thread. If `dance` becomes blocked,
            // `learn_and_sing` can take back over. If both futures are blocked, then
            // `async_main` is blocked and will yield to the executor.
            futures::join!(f1,f2);
        }
    }
}