use leptos::*;

fn main() {
    mount_to_body(BookList)
}

#[component]
fn BookList() -> impl IntoView {
    let books = vec![
        "Software craft : TDD, Clean Code et autres pratiques essentielles",
        "Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems",
        "Accelerate: The Science of Lean Software and DevOps: Building and Scaling High Performing Technology"
    ];

    view! {
        <ul class="p-8 list-disc list-inside">
            {books.into_iter()
                .map(|book| view! { <li class="my-2">{book}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}
