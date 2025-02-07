use cfg_if::cfg_if;
use leptos::leptos_dom::IntoChild;
use leptos::*;
use leptos::typed_builder::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
use wasm_bindgen::JsCast;

use crate::{use_location, use_resolved_path, State};

/// Describes a value that is either a static or a reactive URL, i.e.,
/// a [String], a [&str], or a reactive `Fn() -> String`.
pub trait TextProp {
    /// Converts the (static or reactive) URL into a function that can be called to
    /// return the URL.
    fn to_value(&self) -> Box<dyn Fn() -> String + '_>;
}

impl TextProp for &str {
    fn to_value(&self) -> Box<dyn Fn() -> String> {
        let s = self.to_string();
        Box::new(move || s.clone())
    }
}

impl TextProp for String {
    fn to_value(&self) -> Box<dyn Fn() -> String> {
        let s = self.clone();
        Box::new(move || s.clone())
    }
}

impl<F> TextProp for F
where
    F: Fn() -> String + 'static,
{
    fn to_value(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(self)
    }
}

/// Properties that can be passed to the [A] component, which is an HTML
/// [`a`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
/// progressively enhanced to use client-side routing.
#[derive(TypedBuilder)]
pub struct AProps<C, H>
where
    C: IntoChild,
    H: TextProp + 'static,
{
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    pub href: H,
    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[builder(default)]
    pub exact: bool,
    /// An object of any type that will be pushed to router state
    #[builder(default, setter(strip_option, into))]
    pub state: Option<State>,
    /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    /// will skip this page.)
    #[builder(default)]
    pub replace: bool,
    /// Sets the `class` attribute on the underlying `<a>` tag, making it easier to style.
    #[builder(default, setter(strip_option, into))]
    pub class: Option<MaybeSignal<String>>,
    /// The nodes or elements to be shown inside the link.
    pub children: Box<dyn Fn() -> Vec<C>>,
}

/// An HTML [`a`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
/// progressively enhanced to use client-side routing.
///
/// Client-side routing also works with ordinary HTML `<a>` tags, but `<A>` does two additional things:
/// 1) Correctly resolves relative nested routes. Relative routing with ordinary `<a>` tags can be tricky.
///    For example, if you have a route like `/post/:id`, `<A href="1">` will generate the correct relative
///    route, but `<a href="1">` likely will not (depending on where it appears in your view.)
/// 2) Sets the `aria-current` attribute if this link is the active link (i.e., it’s a link to the page you’re on).
///    This is helpful for accessibility and for styling. For example, maybe you want to set the link a 
///    different color if it’s a link to the page you’re currently on.
#[allow(non_snake_case)]
pub fn A<C, H>(cx: Scope, props: AProps<C, H>) -> Element
where
    C: IntoChild,
    H: TextProp + 'static,
{
    let location = use_location(cx);
    let href = use_resolved_path(cx, move || props.href.to_value()());
    let is_active = create_memo(cx, move |_| match href.get() {
        None => false,

        Some(to) => {
            let path = to
                .split(['?', '#'])
                .next()
                .unwrap_or_default()
                .to_lowercase();
            let loc = location.pathname.get().to_lowercase();
            if props.exact {
                loc == path
            } else {
                loc.starts_with(&path)
            }
        }
    });

    let mut children = (props.children)();
    if children.len() != 1 {
        debug_warn!("[Link] Pass exactly one child to <A/>. If you want to pass more than one child, nest them within an element.");
    }
    let child = children.remove(0);
    let class = props.class;

    cfg_if! {
        if #[cfg(any(feature = "csr", feature = "hydrate"))] {
            view! { cx,
                <a
                    href=move || href.get().unwrap_or_default()
                    prop:state={props.state.map(|s| s.to_js_value())}
                    prop:replace={props.replace}
                    aria-current=move || if is_active.get() { Some("page") } else { None }
                    class=move || class.as_ref().map(|class| class.get())
                >
                    {child}
                </a>
            }
        } else {
            view! { cx,
                <a
                    href=move || href().unwrap_or_default()
                    aria-current=move || if is_active() { Some("page") } else { None }
                    class=move || class.as_ref().map(|class| class.get())
                >
                    {child}
                </a>
            }
        }
    }
}
