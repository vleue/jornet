# Bevy Jornet

Bevy plugin for easy leaderboard integration with [Jornet](https://jornet.vleue.com). Works in WASM and native.

## Setup

Add this crate as a dependency, then add the plugin. You cna get an `id` and a `key` at https://jornet.vleue.com. The key must remain secret.

```rust
app.add_plugin(JornetPlugin::with_leaderboard(id, key));
```

```rust
fn leaderboard_setup(mut leaderboard: ResMut<Leaderboard>) {
    // `None` will create a new user with a random name
    leaderboard.create_player(None);
}
```
