content =========  <p> hey |name| from component </p>
---------------------------------------- source = "user"
---------------------------------------- source = "User"
content =========  <p> this is a testing rust page |test| </p> <button onclick="(() => {
        let loc = window.location.pathname;
        conn.send(loc + `,incr,0`);
        })();"
> incr </button> <button onclick="(() => {
        let loc = window.location.pathname;
        conn.send(loc + `,decr,0`);
        })();"> decr </button> <user />
---------------------------------------- source = "testMacro"
---------------------------------------- source = "TestMacro"
found func incr
found func decr
---------------------------------------- source = "homepage"
---------------------------------------- source = "Homepage"
#![feature(prelude_import)]
#![feature(fn_traits)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod shared {
    pub mod user {
        use macros::*;
        use achernar::html::*;
        use achernar::page::Html;
        use once_cell::sync::Lazy;
        pub static HTML: Lazy<Html> = Lazy::new(|| Html {
            content: HtmlRoot::from_str(&"<p> hey |name| from component </p>"),
            id: "YdpPKd4HwiRHQdBmGMqwNa6c1G5q4s".to_string(),
        });
        use std::sync::Arc;
        use achernar::page::*;
        use futures_util::{SinkExt, stream::SplitSink};
        use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}};
        use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
        use std::collections::HashMap;
        use tracing::info;
        pub struct User {
            #[system]
            sender: Option<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
            #[updateAble]
            name: String,
        }
        use async_trait::async_trait;
        impl Component for User {
            fn eval(&self) -> String {
                use std::fs;
                let buff = {
                    let res = ::alloc::fmt::format(
                        format_args!("<div id={0}>{1}</div>", HTML.id, HTML.content),
                    );
                    res
                };
                let buff = buff
                    .replace(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!("|{0}|", "name"),
                            );
                            res
                        },
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "<span id={0}>{1}</span>",
                                    "RYYyefz6eWsIPyr4G063fLzLAfCQ5i",
                                    self.name,
                                ),
                            );
                            res
                        },
                    );
                buff
            }
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out.insert(
                                "set_name",
                                Box::new(|a, b| Box::pin(Self::set_name(a, b))),
                            );
                            out
                        };
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\shared\\user.rs:7",
                                        "webServer::shared::user",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\shared\\user.rs"),
                                        Some(7u32),
                                        Some("webServer::shared::user"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "method"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&format_args!("executing setter") as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&debug(&method) as &dyn Value),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\shared\\user.rs:7",
                                            "webServer::shared::user",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\shared\\user.rs"),
                                            Some(7u32),
                                            Some("webServer::shared::user"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("found system method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                        } else {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\shared\\user.rs:7",
                                            "webServer::shared::user",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\shared\\user.rs"),
                                            Some(7u32),
                                            Some("webServer::shared::user"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(
                                                            &format_args!("didnt find system method") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            __self.user_execute(method, arg).await;
                        }
                    };
                })
            }
        }
        impl User {
            async fn set_name(&mut self, new_val: &str) {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\shared\\user.rs:7",
                                "webServer::shared::user",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\shared\\user.rs"),
                                Some(7u32),
                                Some("webServer::shared::user"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("in setter") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let Ok(new_val) = new_val.parse::<String>() else {
                if let Some(s) = &self.sender {
                    let mut send = s.lock().await;
                    let _ = send
                        .send(
                            tokio_tungstenite::tungstenite::Message::Text({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "value \"{0}\" is not a valid type of \'{1}\'",
                                        new_val,
                                        "String",
                                    ),
                                );
                                res
                            }),
                        )
                        .await;
                    let _ = send.flush().await;
                }
                return;
            };
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\shared\\user.rs:7",
                                "webServer::shared::user",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\shared\\user.rs"),
                                Some(7u32),
                                Some("webServer::shared::user"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("parse succesfull") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                self.name = new_val.clone();
                if let Some(s) = &self.sender {
                    let mut send = s.lock().await;
                    let _ = send
                        .send(
                            tokio_tungstenite::tungstenite::Message::Text({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}:{1}",
                                        "RYYyefz6eWsIPyr4G063fLzLAfCQ5i",
                                        new_val,
                                    ),
                                );
                                res
                            }),
                        )
                        .await;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event webServer\\src\\shared\\user.rs:7",
                                    "webServer::shared::user",
                                    ::tracing::Level::INFO,
                                    Some("webServer\\src\\shared\\user.rs"),
                                    Some(7u32),
                                    Some("webServer::shared::user"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("flushing") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let _ = send.flush().await;
                }
            }
        }
        impl User {
            fn init(&mut self) {
                self.name = "namer".to_string();
            }
        }
        use achernar::page::*;
        impl UserFunctions for User {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn user_execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\shared\\user.rs:7",
                                        "webServer::shared::user",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\shared\\user.rs"),
                                        Some(7u32),
                                        Some("webServer::shared::user"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(
                                                        &format_args!("searching for user methods") as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\shared\\user.rs:7",
                                            "webServer::shared::user",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\shared\\user.rs"),
                                            Some(7u32),
                                            Some("webServer::shared::user"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("executing user method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                            return;
                        }
                    };
                })
            }
        }
        impl User {
            pub fn new() -> Self {
                let mut out = Self {
                    sender: None,
                    name: <String>::default(),
                };
                Self::init(&mut out);
                out
            }
        }
    }
}
mod pages {
    pub mod testMacro {
        use macros::*;
        pub static ROUTE: &str = "/testRoute";
        use achernar::html::*;
        use achernar::page::Html;
        use once_cell::sync::Lazy;
        pub static HTML: Lazy<Html> = Lazy::new(|| Html {
            content: HtmlRoot::from_str(
                &"<p> this is a testing rust page |test| </p> <button onclick=\"(() => {\n        let loc = window.location.pathname;\n        conn.send(loc + `,incr,0`);\n        })();\"\n> incr </button> <button onclick=\"(() => {\n        let loc = window.location.pathname;\n        conn.send(loc + `,decr,0`);\n        })();\"> decr </button> <user />",
            ),
            id: "OyBXSU2bSr7A0NcTdbRT36Kog5roJ9".to_string(),
        });
        use std::sync::Arc;
        use achernar::page::*;
        use futures_util::{SinkExt, stream::SplitSink};
        use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}};
        use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
        use std::collections::HashMap;
        use tracing::info;
        pub struct TestMacro {
            #[system]
            sender: Option<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
            #[updateAble]
            test: i8,
            #[updateAble]
            subitem: Box<i8>,
        }
        use async_trait::async_trait;
        impl Page for TestMacro {
            fn set_sender(
                &mut self,
                sender: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
            ) {
                self.sender = Some(sender);
            }
            fn eval(
                &self,
                components: MutexGuard<
                    '_,
                    HashMap<String, Box<dyn Component + Sync + Send>>,
                >,
            ) -> String {
                use std::fs;
                let mut x = HTML.content.clone();
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\testMacro.rs:17",
                                "webServer::pages::testMacro",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\testMacro.rs"),
                                Some(17u32),
                                Some("webServer::pages::testMacro"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("components = ") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                components
                    .iter()
                    .for_each(|(name, _)| {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event webServer\\src\\pages\\testMacro.rs:17",
                                    "webServer::pages::testMacro",
                                    ::tracing::Level::INFO,
                                    Some("webServer\\src\\pages\\testMacro.rs"),
                                    Some(17u32),
                                    Some("webServer::pages::testMacro"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("{0}", name) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    });
                components
                    .iter()
                    .for_each(|(name, comp)| {
                        x.replace_component(name, &comp.eval());
                    });
                let buff = {
                    let res = ::alloc::fmt::format(
                        format_args!("<div id={0}>{1}</div>", HTML.id, x),
                    );
                    res
                };
                let buff = buff
                    .replace(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!("|{0}|", "test"),
                            );
                            res
                        },
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "<span id={0}>{1}</span>",
                                    "slEeYrf7wzCLxWaMjV4gHLcVoqOTNn",
                                    self.test,
                                ),
                            );
                            res
                        },
                    );
                let buff = buff
                    .replace(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!("|{0}|", "subitem"),
                            );
                            res
                        },
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "<span id={0}>{1}</span>",
                                    "b0I7GaMsM0K1yo6at4SQb609bSpFtp",
                                    self.subitem,
                                ),
                            );
                            res
                        },
                    );
                buff
            }
            fn state_has_changed(&self) {
                {
                    ::std::io::_print(format_args!("should update now lol\n"));
                };
            }
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out.insert(
                                "set_test",
                                Box::new(|a, b| Box::pin(Self::set_test(a, b))),
                            );
                            out.insert(
                                "set_subitem",
                                Box::new(|a, b| Box::pin(Self::set_subitem(a, b))),
                            );
                            out
                        };
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\pages\\testMacro.rs:17",
                                        "webServer::pages::testMacro",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\pages\\testMacro.rs"),
                                        Some(17u32),
                                        Some("webServer::pages::testMacro"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "method"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&format_args!("executing setter") as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&debug(&method) as &dyn Value),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\testMacro.rs:17",
                                            "webServer::pages::testMacro",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\testMacro.rs"),
                                            Some(17u32),
                                            Some("webServer::pages::testMacro"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("found system method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                        } else {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\testMacro.rs:17",
                                            "webServer::pages::testMacro",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\testMacro.rs"),
                                            Some(17u32),
                                            Some("webServer::pages::testMacro"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(
                                                            &format_args!("didnt find system method") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            __self.user_execute(method, arg).await;
                        }
                    };
                })
            }
        }
        impl TestMacro {
            async fn set_test(&mut self, new_val: &i8) {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\testMacro.rs:17",
                                "webServer::pages::testMacro",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\testMacro.rs"),
                                Some(17u32),
                                Some("webServer::pages::testMacro"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("in setter") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\testMacro.rs:17",
                                "webServer::pages::testMacro",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\testMacro.rs"),
                                Some(17u32),
                                Some("webServer::pages::testMacro"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("parse succesfull") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                self.test = new_val.clone();
                if let Some(s) = &self.sender {
                    let mut send = s.lock().await;
                    let _ = send
                        .send(
                            tokio_tungstenite::tungstenite::Message::Text({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}:{1}",
                                        "slEeYrf7wzCLxWaMjV4gHLcVoqOTNn",
                                        new_val,
                                    ),
                                );
                                res
                            }),
                        )
                        .await;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event webServer\\src\\pages\\testMacro.rs:17",
                                    "webServer::pages::testMacro",
                                    ::tracing::Level::INFO,
                                    Some("webServer\\src\\pages\\testMacro.rs"),
                                    Some(17u32),
                                    Some("webServer::pages::testMacro"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("flushing") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let _ = send.flush().await;
                }
            }
            async fn set_subitem(&mut self, new_val: &Box<i8>) {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\testMacro.rs:17",
                                "webServer::pages::testMacro",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\testMacro.rs"),
                                Some(17u32),
                                Some("webServer::pages::testMacro"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("in setter") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\testMacro.rs:17",
                                "webServer::pages::testMacro",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\testMacro.rs"),
                                Some(17u32),
                                Some("webServer::pages::testMacro"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("parse succesfull") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                self.subitem = new_val.clone();
                if let Some(s) = &self.sender {
                    let mut send = s.lock().await;
                    let _ = send
                        .send(
                            tokio_tungstenite::tungstenite::Message::Text({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}:{1}",
                                        "b0I7GaMsM0K1yo6at4SQb609bSpFtp",
                                        new_val,
                                    ),
                                );
                                res
                            }),
                        )
                        .await;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event webServer\\src\\pages\\testMacro.rs:17",
                                    "webServer::pages::testMacro",
                                    ::tracing::Level::INFO,
                                    Some("webServer\\src\\pages\\testMacro.rs"),
                                    Some(17u32),
                                    Some("webServer::pages::testMacro"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("flushing") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let _ = send.flush().await;
                }
            }
        }
        impl TestMacro {
            async fn incr(&mut self, _val: &str) {
                self.set_test(&(self.test + 1)).await;
            }
            async fn decr(&mut self, _val: &str) {
                self.set_test(&(self.test - 1)).await;
            }
            fn init(&mut self) {
                self.test = 8;
            }
        }
        use achernar::page::*;
        impl UserFunctions for TestMacro {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn user_execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\pages\\testMacro.rs:17",
                                        "webServer::pages::testMacro",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\pages\\testMacro.rs"),
                                        Some(17u32),
                                        Some("webServer::pages::testMacro"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(
                                                        &format_args!("searching for user methods") as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out.insert(
                                "incr",
                                Box::new(|a, b| Box::pin(Self::incr(a, b))),
                            );
                            out.insert(
                                "decr",
                                Box::new(|a, b| Box::pin(Self::decr(a, b))),
                            );
                            out
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\testMacro.rs:17",
                                            "webServer::pages::testMacro",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\testMacro.rs"),
                                            Some(17u32),
                                            Some("webServer::pages::testMacro"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("executing user method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                            return;
                        }
                    };
                })
            }
        }
        impl TestMacro {
            pub fn new() -> Self {
                let mut out = Self {
                    sender: None,
                    test: <i8>::default(),
                    subitem: <Box<i8>>::default(),
                };
                Self::init(&mut out);
                out
            }
        }
    }
    pub mod homepage {
        use macros::*;
        pub static ROUTE: &str = "/";
        use achernar::html::HtmlRoot;
        use achernar::page::Html;
        use once_cell::sync::Lazy;
        pub static HTML: Lazy<Html> = Lazy::new(|| Html {
            content: HtmlRoot::from_str(
                &"\r\n\r\n<div>\r\n    <p> hey |name|</p>\r\n    <p> this is a testing homepage build to test routing </p>\r\n    <a href=\"testRoute\"> to testing page </a>\r\n\r\n</div>",
            ),
            id: "dWCszAuyacuAQmON6h4yNLLz2MnfPe".to_string(),
        });
        use std::sync::Arc;
        use achernar::page::*;
        use futures_util::{SinkExt, stream::SplitSink};
        use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}};
        use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
        use std::collections::HashMap;
        use tracing::info;
        pub struct Homepage {
            #[system]
            sender: Option<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
            name: String,
        }
        use async_trait::async_trait;
        impl Page for Homepage {
            fn set_sender(
                &mut self,
                sender: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
            ) {
                self.sender = Some(sender);
            }
            fn eval(
                &self,
                components: MutexGuard<
                    '_,
                    HashMap<String, Box<dyn Component + Sync + Send>>,
                >,
            ) -> String {
                use std::fs;
                let mut x = HTML.content.clone();
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event webServer\\src\\pages\\homepage.rs:6",
                                "webServer::pages::homepage",
                                ::tracing::Level::INFO,
                                Some("webServer\\src\\pages\\homepage.rs"),
                                Some(6u32),
                                Some("webServer::pages::homepage"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = CALLSITE.metadata().fields().iter();
                            CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&format_args!("components = ") as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                components
                    .iter()
                    .for_each(|(name, _)| {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event webServer\\src\\pages\\homepage.rs:6",
                                    "webServer::pages::homepage",
                                    ::tracing::Level::INFO,
                                    Some("webServer\\src\\pages\\homepage.rs"),
                                    Some(6u32),
                                    Some("webServer::pages::homepage"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("{0}", name) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    });
                components
                    .iter()
                    .for_each(|(name, comp)| {
                        x.replace_component(name, &comp.eval());
                    });
                let buff = {
                    let res = ::alloc::fmt::format(
                        format_args!("<div id={0}>{1}</div>", HTML.id, x),
                    );
                    res
                };
                let buff = buff
                    .replace(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!("|{0}|", "name"),
                            );
                            res
                        },
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "<span id={0}>{1}</span>",
                                    "8Vy4i6sD1QaGfSX9edaWtnYVKauhV8",
                                    self.name,
                                ),
                            );
                            res
                        },
                    );
                buff
            }
            fn state_has_changed(&self) {
                {
                    ::std::io::_print(format_args!("should update now lol\n"));
                };
            }
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out
                        };
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\pages\\homepage.rs:6",
                                        "webServer::pages::homepage",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\pages\\homepage.rs"),
                                        Some(6u32),
                                        Some("webServer::pages::homepage"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "method"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&format_args!("executing setter") as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&debug(&method) as &dyn Value),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\homepage.rs:6",
                                            "webServer::pages::homepage",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\homepage.rs"),
                                            Some(6u32),
                                            Some("webServer::pages::homepage"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("found system method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                        } else {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\homepage.rs:6",
                                            "webServer::pages::homepage",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\homepage.rs"),
                                            Some(6u32),
                                            Some("webServer::pages::homepage"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(
                                                            &format_args!("didnt find system method") as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            __self.user_execute(method, arg).await;
                        }
                    };
                })
            }
        }
        impl Homepage {}
        impl Homepage {
            fn init(&mut self) {
                self.name = "user".to_string();
            }
        }
        use achernar::page::*;
        impl UserFunctions for Homepage {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn user_execute<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 mut self,
                method: &'life1 str,
                arg: &'life2 str,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = (),
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let mut __self = self;
                    let () = {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event webServer\\src\\pages\\homepage.rs:6",
                                        "webServer::pages::homepage",
                                        ::tracing::Level::INFO,
                                        Some("webServer\\src\\pages\\homepage.rs"),
                                        Some(6u32),
                                        Some("webServer::pages::homepage"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(
                                                        &format_args!("searching for user methods") as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        let methods = {
                            let mut out: HashMap<&'static str, Handler<Self>> = HashMap::new();
                            out
                        };
                        if let Some(func) = methods.get(method) {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event webServer\\src\\pages\\homepage.rs:6",
                                            "webServer::pages::homepage",
                                            ::tracing::Level::INFO,
                                            Some("webServer\\src\\pages\\homepage.rs"),
                                            Some(6u32),
                                            Some("webServer::pages::homepage"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("executing user method") as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            (func)(__self, arg).await;
                            return;
                        }
                    };
                })
            }
        }
        impl Homepage {
            pub fn new() -> Self {
                let mut out = Self {
                    sender: None,
                    name: <String>::default(),
                };
                Self::init(&mut out);
                out
            }
        }
    }
}
mod http_return {
    pub struct HttpReturn {
        version: String,
        status_code: u32,
        reason: String,
        headers: Vec<String>,
        body: Option<String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for HttpReturn {
        #[inline]
        fn clone(&self) -> HttpReturn {
            HttpReturn {
                version: ::core::clone::Clone::clone(&self.version),
                status_code: ::core::clone::Clone::clone(&self.status_code),
                reason: ::core::clone::Clone::clone(&self.reason),
                headers: ::core::clone::Clone::clone(&self.headers),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl HttpReturn {
        pub fn new(version: &str) -> Self {
            Self {
                version: {
                    let res = ::alloc::fmt::format(format_args!("HTTP/{0}", version));
                    res
                },
                status_code: 500,
                reason: "Internal server Error".to_string(),
                headers: Vec::new(),
                body: None,
            }
        }
        pub fn to_string(self) -> String {
            let headers: String = self
                .headers
                .into_iter()
                .map(|val| {
                    let res = ::alloc::fmt::format(format_args!("{0}\r\n", val));
                    res
                })
                .collect();
            let mut out = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0} {1} {2}\r\n{3}\r\n",
                        self.version,
                        self.status_code,
                        self.reason,
                        headers,
                    ),
                );
                res
            };
            if headers.is_empty() {
                out += "\r\n";
            }
            if let Some(x) = self.body {
                out += &x;
            }
            {
                ::std::io::_print(format_args!("test = {0}\n", out));
            };
            out
        }
    }
    pub struct HttpReturnBuilder {
        build_target: HttpReturn,
    }
    impl HttpReturnBuilder {
        pub fn from_version(version: &str) -> Self {
            HttpReturnBuilder {
                build_target: HttpReturn::new(version),
            }
        }
        pub fn set_code(mut self, code: u32, reason: &str) -> Self {
            self.build_target.status_code = code;
            self.build_target.reason = reason.to_owned();
            self
        }
        pub fn add_to_headers(mut self, header: &str) -> Self {
            self.build_target.headers.push(header.to_string());
            self
        }
        pub fn add_content(mut self, content: &str) -> Self {
            self.build_target
                .headers
                .push({
                    let res = ::alloc::fmt::format(
                        format_args!("Content-Length: {0}", content.len()),
                    );
                    res
                });
            self.build_target.body = Some(content.to_owned());
            self
        }
        pub fn build(self) -> HttpReturn {
            self.build_target.clone()
        }
        pub fn internal_error() -> HttpReturn {
            Self::from_version("1.1").set_code(500, "Internal Server Error").build()
        }
        pub fn not_implemented() -> HttpReturn {
            Self::from_version("1.1").set_code(501, "not implemented").build()
        }
        pub fn moved(file: &str) -> HttpReturn {
            Self::from_version("1.1")
                .set_code(301, "moved")
                .add_to_headers(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("Location: /{0}", file),
                        );
                        res
                    },
                )
                .build()
        }
    }
}
use achernar;
use macros::load_components;
fn main() -> anyhow::Result<()> {
    let body = async {
        use macros::load_pages;
        use achernar::page::*;
        let set = achernar::settings::Settings {
            ip_address: "127.0.0.1".to_string(),
            port: 8081,
            shared_folder: {
                use std::collections::HashMap;
                use std::boxed;
                use std::sync::Arc;
                use tokio::sync::Mutex;
                let mut output: Arc<
                    Mutex<HashMap<String, Box<dyn Component + Sync + Send>>>,
                > = Arc::new(Mutex::new(HashMap::new()));
                {
                    let mut out = output.lock().await;
                    out.insert("user".to_string(), Box::new(shared::user::User::new()));
                }
                output
            },
            pages_folder: {
                use std::collections::HashMap;
                use std::boxed;
                use std::sync::Arc;
                use tokio::sync::Mutex;
                let mut output: Arc<
                    Mutex<HashMap<String, Box<dyn Page + Sync + Send>>>,
                > = Arc::new(Mutex::new(HashMap::new()));
                {
                    let mut out = output.lock().await;
                    out.insert(
                        pages::homepage::ROUTE.to_string(),
                        Box::new(pages::homepage::Homepage::new()),
                    );
                    out.insert(
                        pages::testMacro::ROUTE.to_string(),
                        Box::new(pages::testMacro::TestMacro::new()),
                    );
                }
                output
            },
        };
        achernar::init(set).await
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
