use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> Element {
    view! { cx,
        <header class="header">
            <nav class="inner">
                <A href="/">
                    <strong>"HN"</strong>
                </A>
                <A href="/new">
                    <strong>"New"</strong>
                </A>
                <A href="/show">
                    <strong>"Show"</strong>
                </A>
                <A href="/ask">
                    <strong>"Ask"</strong>
                </A>
                <A href="/job">
                    <strong>"Jobs"</strong>
                </A>
                <a class="github" href="http://github.com/gbj/leptos" target="_blank" rel="noreferrer">
                    "Built with Leptos"
                </a>
            </nav>
        </header>
    }
}
