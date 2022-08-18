# Bevy Jornet

![Jornet logo](https://jornet.vleue.com/logo-200.png)

[Bevy](https://bevyengine.org) plugin for easy leaderboard integration with [Jornet](https://jornet.vleue.com). Works in WASM and native.

## Setup

Add this crate as a dependency, then add the plugin. You cna get an `id` and a `key` at https://jornet.vleue.com. The key must remain secret.

```rust
app.add_plugin(JornetPlugin::with_leaderboard(id, key));
```

You can then create a new player to send scores, or retrieve the current leaderboard:

```rust
fn leaderboard_setup(mut leaderboard: ResMut<Leaderboard>) {
    // `None` will create a new user with a random name
    leaderboard.create_player(None);

    leaderboard.refresh_leaderboard();
}
```

See [the `whac-a-square` example](./examples/whac-a-square.rs) for a complete integration.

![leaderboard](https://raw.githubusercontent.com/vleue/jornet/main/bevy-jornet/leaderboard.png)
