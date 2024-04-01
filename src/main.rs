use leptos::{
    component, create_signal, event_target_value, view, For, IntoView, ReadSignal, SignalGet,
    SignalSet, SignalUpdate, WriteSignal,
};
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
struct TodoItem {
    id: u32,
    content: String,
}

fn new_todo_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>) = create_signal(vec![]);
    view! {
        <div class="w-full h-full bg-red-400 flex justify-center items-start">
            <div class="flex flex-col gap-y-2 justify-center mt-2 min-w-[799px] max-w-[1299px]">
                <h1 class="text-white font-bold text-3xl text-center">Todo App</h1>
                <TodoInput initial_todos=todos/>
                <TodoList todos=todos/>
            </div>
        </div>
    }
}

#[component]
fn TodoInput(
    initial_todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>),
) -> impl IntoView {
    let (_, set_new_todo) = initial_todos;
    let (default_value, set_default_value) = create_signal("");

    view! {
        <input
            type="text"
            class="p-4 rounded-md drop-shadow-md"
            autofocus=true
            placeholder="Add todo"
            on:keydown=move |event| {
                if event.key() == "Enter" && !event_target_value(&event).is_empty() {
                    let input_value = event_target_value(&event);
                    let new_todo_item = TodoItem {
                        id: new_todo_id(),
                        content: input_value.clone(),
                    };
                    set_new_todo.update(|todo| todo.push(new_todo_item));
                    set_default_value.set("");
                }
            }

            prop:value=default_value
        />
    }
}

#[component]
fn TodoList(todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>)) -> impl IntoView {
    let (todo_list_state, set_todo_list_state) = todos;
    let my_todos = move || {
        todo_list_state
            .get()
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };
    view! {
        <ul class="bg-white w-full">
            <For
                each=my_todos
                key=|todo_key| todo_key.0
                children=move |item| {
                    view! {
                        <li class="text-black text-lg p-2 border-b-2 border-neutral-200 w-full flex justify-between items-center">
                            <p class="flex-1 p-4">{item.1.content}</p>
                            <button
                                class="w-[100px] h-[45px] bg-neutral-600 text-white rounded-md"
                                on:click=move |_| {
                                    set_todo_list_state
                                        .update(|todos| {
                                            todos.retain(|todo| todo.id != item.1.id)
                                        });
                                }
                            >

                                delete
                            </button>
                        </li>
                    }
                }
            />

        </ul>
    }
}
