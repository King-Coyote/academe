#[macro_export]
macro_rules! context_menu_handler {
    (
        $commands:ident,
        $mouse:ident,
        $({
            label: $label:expr,
            action: $action:block
        }),*
    ) => {
        Some(Box::new(move |$commands: &mut Commands, $mouse: &MouseState| {
            let world_pos = $mouse.world_pos;
            let ui_pos = $mouse.ui_pos;
            $commands.spawn().insert(ContextMenuSpawn {
                pos: ui_pos,
                items: vec![
                    $(ContextMenuItem {
                        label: $label.to_string(),
                        handlers: Some(ClickHandlers {
                            left: Some(Box::new(move |$commands: &mut Commands, $mouse: &MouseState| $action)),
                            ..Default::default()
                        }),
                    },)*
                ]
            });
        }))
    }
}

#[macro_export]
macro_rules! context_menu {
    (
        $commands:ident,
        $mouse:ident,
        $({
            label: $label:expr,
            action: $action:block
        }),*
    ) => {
        ContextMenuSpawn {
            pos: $mouse.ui_pos,
            items: vec![
                $(ContextMenuItem {
                    label: $label.to_string(),
                    handlers: Some(ClickHandlers {
                        left: Some(Box::new(move |$commands: &mut Commands, $mouse: &MouseState| $action)),
                        ..Default::default()
                    }),
                },)*
            ]
        }
    }
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
macro_rules! dynamic_struct {
    (
        $({$name:expr, $val:expr}),*
    ) => {
        {
            let mut ds = DynamicStruct::default();
            $(ds.insert($name, $val);)*
            Arc::new(ds)
        }
    };
}

// use for newtype structs
#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty) => {
        impl Deref for $type {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// use for newtype structs
#[macro_export]
macro_rules! impl_deref_mut {
    ($type:ty, $target:ty) => {
        impl Deref for $type {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
