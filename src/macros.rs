// #[macro_export]
// macro_rules! context_menu {
//     ($($item:block),*) => {
//         ContextMenu(vec![
//             $(ContextMenuItem 
//                 $item
//             ,)*
//         ])
//     };
// }
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

#[macro_export]
macro_rules! test {
    ($($x:block),*) => {
        $($x)*;
    };
}
/*
ContextMenu(vec![
            ContextMenuItem {
                label: "Spawn creature".to_string(),
                commands: Arc::new(GameCommandQueue(vec![
                    GameCommand{
                        target: Target::World(None),
                        command: GameCommandType::Create("Body".to_string()),
                        level: 5,
                    },
                ])),
                closing: true,
            }
        ])

context_menu!(
    {},
)
*/