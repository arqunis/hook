# Hook

Hook macro extracted from the `command_attr` crate.

This is a very simple macro for converting `async fn`s into `fn() -> Pin<Box<dyn
Future<...>>>`.

## License

`command_attr` is a crate belonging to the [Serenity project][serenity], which
is licensed under ISC. As such, this macro is also ISC licensed, with copyright
attributed to contributors of the Serenity project.

[serenity]: https://github.com/serenity-rs/serenity
