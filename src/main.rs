use std::fmt::Display;

use leptos::*;

fn main() {
    mount_to_body(BookList)
}

enum Language {
    EN,
    FR,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::EN => f.write_str("🇬🇧"),
            Language::FR => f.write_str("🇫🇷"),
        }
    }
}

struct Book {
    title: String,
    authors: Vec<String>,
    language: Language,
}

#[component]
fn BookList() -> impl IntoView {
    let books = vec![
        Book {
            title: "Software craft : TDD, Clean Code et autres pratiques essentielles".into(),
            authors: vec![
                "Cyrille Martraire".into(),
                "Arnaud Thiéfaine".into(),
                "Dorra Bartaguiz".into(),
                "Fabien Hiegel".into(),
                "Houssam Fakih".into(),
            ],
            language: Language::FR,
        },
        Book {
            title: "Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems".into(),
            authors: vec![
                "Martin Kleppmann".into(),
            ],
            language: Language::EN,
        },
        Book {
            title: "Accelerate: The Science of Lean Software and DevOps: Building and Scaling High Performing Technology".into(),
            authors: vec![
                "Nicole Forsgren".into(),
                "Jez Humble".into(),
                "Gene Kim".into(),
            ],
            language: Language::EN,
        }
    ];

    view! {
        <ul class="p-8 list-disc list-inside">
            {books.into_iter()
                .map(|book| view! { <li class="my-2"><BookCard book=book /></li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn BookCard(book: Book) -> impl IntoView {
    view! {
        <span>{book.language.to_string()} - {book.title}</span>
        <ul class="p-6 list-disc list-inside">
            {book.authors.into_iter()
                .map(|author| view! { <li class="my-2">{author}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}
