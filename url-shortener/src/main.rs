use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        let count = create_signal(vec![1, 2]);
        view! {
            ul {
                Keyed(
                    iterable=count,
                    view=|x| view! {
                        li { (x) }
                    },
                )
            }
        }
    });
}