use crate::context::Context;
use async_trait::async_trait;
use twilight_gateway::{Event, EventTypeFlags};
use twilight_model::gateway::payload::incoming::Ready;

macro_rules! create_event_handlers {
    ($($event_name:ident ($arg_type:ty)),* $(,)?) => {
        paste::paste! {
            #[async_trait]
            pub trait EventHandler: Send + Sync {
                    $(
                        async fn [<$event_name:snake>](&self, context: Context, [<$event_name:snake>]: $arg_type);
                    )*

                    fn used_event_flags(&self) -> EventTypeFlags {
                        EventTypeFlags::empty() $(| EventTypeFlags::[<$event_name:snake:upper>])*
                    }
            }

            #[async_trait]
            impl EventExt for Event {
                async fn dispatch(self, context: Context, event_handler: &dyn EventHandler) {
                    match self {
                        $(
                            Event::$event_name(event) => event_handler.[<$event_name:snake>](context, event).await,
                        )*
                        _ => {}
                    }
                }
            }
        }
    };
}

create_event_handlers! {
    Ready(Box<Ready>)
}

#[async_trait]
pub trait EventExt {
    async fn dispatch(self, context: Context, event_handler: &dyn EventHandler);
}
