use leptos_reactive::Scope;

/// Represents the different possible values a single class on an element could have,
/// allowing you to do fine-grained updates to single items
/// in [`Element.classList`](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList).
///
/// This mostly exists for the [`view`](https://docs.rs/leptos_macro/latest/leptos_macro/macro.view.html)
/// macro’s use. You usually won't need to interact with it directly.
pub enum Class {
    /// Whether the class is present.
    Value(bool),
    /// A (presumably reactive) function, which will be run inside an effect to toggle the class.
    Fn(Box<dyn Fn() -> bool>),
}

/// Converts some type into a [Class].
pub trait IntoClass {
    /// Converts the object into a [Class].
    fn into_class(self, cx: Scope) -> Class;
}

impl IntoClass for bool {
    fn into_class(self, _cx: Scope) -> Class {
        Class::Value(self)
    }
}

impl<T> IntoClass for T
where
    T: Fn() -> bool + 'static,
{
    fn into_class(self, _cx: Scope) -> Class {
        let modified_fn = Box::new(self);
        Class::Fn(modified_fn)
    }
}

impl Class {
    /// Converts the class to its HTML value at that moment so it can be rendered on the server.
    pub fn as_value_string(&self, class_name: &'static str) -> &'static str {
        match self {
            Class::Value(value) => {
                if *value {
                    class_name
                } else {
                    ""
                }
            }
            Class::Fn(f) => {
                let value = f();
                if value {
                    class_name
                } else {
                    ""
                }
            }
        }
    }
}
