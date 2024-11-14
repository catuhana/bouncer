use async_trait::async_trait;
use twilight_gateway::Event;
use twilight_model::gateway::payload::incoming::Ready;

macro_rules! create_event_handlers {
    ($($event_name:ident ($arg:ident: $arg_type:ty)),* $(,)?) => {
        paste::paste! {
            #[async_trait]
            pub trait EventHandler: Send + Sync {
                    $(
                        async fn [<$event_name:snake>](&self, $arg: $arg_type) {
                            drop($arg);
                        }
                    )*
            }

            #[async_trait]
            impl EventExt for Event {
                async fn dispatch(self, event_handler: &dyn EventHandler) {
                    match self {
                        $(
                            Event::$event_name(ready) => event_handler.[<$event_name:snake>](ready).await,
                        )*
                        _ => {}
                    }
                }
            }
        }
    };
}

// TODO: Make these have `context` to reference the client.
create_event_handlers! {
    Ready(ready: Box<Ready>)
}

#[async_trait]
pub trait EventExt {
    async fn dispatch(self, event_handler: &dyn EventHandler);
}
