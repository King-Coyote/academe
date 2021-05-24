#[macro_export]
macro_rules! context_menu {
    (
        $({
            label: $label:expr,
            commands: $commands:expr,
            closing: $closing:ident
        }),*
    ) => {
        ContextMenu(vec![
            $(ContextMenuItem {
                label: $label.to_string(),
                commands: $commands,
                closing: $closing,
            },)*
        ])
    };
}

#[macro_export]
macro_rules! game_commands {
    (
        $({
            target: $target:expr,
            command: $command:expr,
            level: $level:expr,
        }),*
    ) => {
        Arc::new(GameCommandQueue(vec![
            $(GameCommand {
                target: $target,
                command: $command,
                level: $level,
            },)*
        ]))
    };
}