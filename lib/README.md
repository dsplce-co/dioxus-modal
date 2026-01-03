> We're dsplce.co, check out our work on [github.com/dsplce-co](https://github.com/dsplce-co) 🖤

# dioxus-modal

 **Modal composable for Dioxus** — A minimal and type-safe framework for modals in [Dioxus](https://dioxuslabs.com/) applications.

⸻

## 🖤 Features

✅ Type-safe modal system with generics<br>
✅ Pass additional context to your modals<br>
✅ Close your modals from anywhere with a global fn<br>
✅ ARIA-compliant<br>
✅ Esc key handling<br>
✅ Portal-style rendering with proper z-index<br>
✅ Proc macro for modal component creation<br>
✅ Zero external CSS dependencies<br>

⸻

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-modal = "0.2"
```

This crate requires Rust 2024 edition and is compatible with Dioxus 0.6.

### Fullstack applications

You need to enable the crate's SSR feature on the server for fullstack apps. In your `Cargo.toml`:

```toml
[features]
server = ["dioxus/server", "dioxus-modal/ssr"]
```

This will tell `dioxus-modal` and its dependencies not to perform DOM-related operations at the stage of server side rendering.

⸻

## 🧪 Usage

### 1. Set up modal collector

Add the `ModalCollector` component to your app to enable rendering modals.

```rust
use dioxus::prelude::*;
use dioxus_modal::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
        ModalCollector {}
    }
}
```

### 2. Create modal component

Imagine in your application there is a user list view, and you want to add the functionality to delete a user to it. You decide a confirmation dialog would come in handy.

In `dioxus-modal`, your modal component needs to adhere to the following signature:

```rust
#[modal]
pub fn ConfirmationModal(input: Input, ctx: Context, close: fn()) -> Element;
```

Where:
- `Input` is dynamic data typically not known until the modal's opening is triggered (in our example it would be the user to delete). Should satisfy `'static`
- `Context` is something constant, passed to the modal on registration and thus not changeable (eg. a function responsible for user deletion). Should satisfy `'static + Clone`

Let's implement the confirmation dialog:

```rust
use dioxus_modal::prelude::*;

#[modal]
pub fn ConfirmationModal(user: User, delete_callback: fn(String), close: fn()) -> Element {
    rsx! {
        div {
            class: "confirmation-modal",
            h2 { "Confirm Action" }
            p { "Are you sure you want to delete {user.name}?" }

            div {
                class: "confirmation-modal__actions",
                button {
                    onclick: move |_| close(),
                    "Cancel"
                }
                button {
                    onclick: move |_| {
                        delete_callback(user.id.clone());
                        close();
                    },
                    "Confirm"
                }
            }
        }
    }
}
```

### 3. Use the modal

Now that you've defined the confirmation modal, let's call it using the `use_modal!` macro:

```rust
use dioxus::{logger::tracing, prelude::*};
use dioxus_modal::prelude::*;

#[derive(Clone, PartialEq)]
struct User {
    id: String,
    name: String,
}

#[component]
fn UsersView(users: Vec<User>) -> Element {
    let delete_user = move |id: String| {
        tracing::info!("Deleting user with id {id}");
    };

    // Registers the modal
    let modal = use_modal!(ConfirmationModal, delete_user);

    let on_delete = EventHandler::new(move |user: User| {
        modal.open(user.clone());
    });

    rsx! {
        // ❗ Notice the `ConfirmationModal` is not mounted directly
        // anywhere — it is the `ModalCollector`'s job to render modals
        ul {
            for user in users {
                li {
                    key: "{user.id}",
                    "{user.name}"
                    button {
                        onclick: move |_| on_delete.call(user.clone()),
                        "Delete"
                    }
                }
            }
        }
    }
}
```

⸻

## 📐 API Reference

### `ModalCollector`

Singleton component that manages modal state and rendering.

### `#[modal]`

Proc macro that helps the `ModalCollector` render your modals.

### `use_modal!`

Creates a typed modal controller:

```rust
// Without context
let modal = use_modal!(ModalComponent);

// With context
let modal = use_modal!(ModalComponent, context);
```

Returns a modal struct with the methods:
- `open(args)` - Opens the modal with provided arguments
- `close()` - Closes the modal

### `close`

Close the modal from anywhere in your application:

```rust
dioxus_modal::close();
```

### Defining a modal

#### With both context and input:

```rust
#[modal]
pub fn ModalComponent(input: Input, ctx: Context, close: fn()) -> Element {
    rsx! {
        // ...
    }
}

let modal = use_modal!(ModalComponent, context);
modal.open(input)
```

#### Skipping context:

```rust
#[modal]
pub fn ModalComponent(input: Input, ctx: (), close: fn()) -> Element {
    rsx! {
        // ...
    }
}

let modal = use_modal!(ModalComponent);
modal.open(input)
```

#### Skipping input:

```rust
#[modal]
pub fn ModalComponent(input: (), ctx: Context, close: fn()) -> Element {
    rsx! {
        // ...
    }
}

let modal = use_modal!(ModalComponent, context);
modal.open(())
```

### Modal Features

- **Accessibility**: Proper ARIA attributes (`role="dialog"`, `aria-modal`)
- **Keyboard Navigation**: Esc key closes modal out of the box
- **Global State**: Modals have a single place to render, and there is always one modal visible at a time (why would you want to show more than one modal at a time? 🤨)
- **Overlay**: Semi-transparent backdrop with proper positioning
- **Responsive**: Full viewport coverage with centered content
- **Transitions**: Built-in smooth enter/leave modal transitions

⸻

## 📁 Repo & Contributions

📦 Crate: [crates.io/crates/dioxus-modal](https://crates.io/crates/dioxus-modal)<br/>
🛠️ Repo: [github.com/dsplce-co/dioxus-modal](https://github.com/dsplce-co/dioxus-modal)<br/>

Contributions, issues, ideas? Hit us up 🖤

⸻

## 🔒 License

MIT or Apache-2.0, at your option.

⸻
