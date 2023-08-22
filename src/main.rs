use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use leptos::{
    component, create_effect, event_target_value, prelude::*, spawn_local, view, For, IntoView,
};
use serde::{Deserialize, Serialize};
use log::info;
use wasm_bindgen::{throw_str, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, HtmlAudioElement};

fn main() {
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap_throw();

    leptos::mount_to_body(|| view! { <App/> });
}

const DEFAULT_PLAYBACK_RATE: f64 = 0.8;
const DEFAULT_VOLUME: f64 = 0.1;

#[component]
fn app() -> impl IntoView {
    let show_settings = create_rw_signal(false);

    let ducky = create_rw_signal(match LocalStorage::get("ducky") {
        Ok(value) => value,
        Err(StorageError::KeyNotFound(_)) => Duck::One,
        Err(err) => throw_str(&err.to_string()),
    });

    create_effect(move |_| {
        LocalStorage::set("ducky", ducky.get()).unwrap_throw();
    });

    let playback_rate = create_rw_signal(match LocalStorage::get("playback_rate") {
        Ok(value) => value,
        Err(StorageError::KeyNotFound(_)) => DEFAULT_PLAYBACK_RATE,
        Err(err) => throw_str(&err.to_string()),
    });

    create_effect(move |_| {
        LocalStorage::set("playback_rate", playback_rate.get()).unwrap_throw();
    });

    let volume = create_rw_signal(match LocalStorage::get("volume") {
        Ok(value) => value,
        Err(StorageError::KeyNotFound(_)) => DEFAULT_VOLUME,
        Err(err) => throw_str(&err.to_string()),
    });

    create_effect(move |_| {
        LocalStorage::set("volume", volume.get()).unwrap_throw();
    });

    view! {
        <div class="flex flex-col gap-3 items-center place-content-center w-screen h-screen">
            <Navbar
                settings=show_settings
            />
            <Content
                ducky=ducky
                playback_rate=playback_rate
                volume=volume
            />
            <Settings
                show=show_settings
                playback_rate=playback_rate
                volume=volume
                selection=ducky
            />
        </div>
    }
}

#[component]
fn navbar(#[prop(into)] settings: SignalSetter<bool>) -> impl IntoView {
    let settings = move |_| settings.set(true);

    view! {
        <div class="flex gap-2 place-items-center m-3">
            <div class="text-2xl">"🦆 Quack"</div>
            <button class="p-2 rounded border-2 transition-all bg-slate-600 border-slate-700 hover:bg-slate-500 hover:border-slate-600" on:click=settings>"Settings"</button>
        </div>
    }
}

#[component]
fn content(
    #[prop(into)] ducky: Signal<Duck>,
    #[prop(into)] playback_rate: Signal<f64>,
    #[prop(into)] volume: Signal<f64>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center text-center">
            <h1 class="text-xl italic">"Rubber Ducking as a service! Finally!"</h1>
            <img class="my-3 rounded-xl w-[400px]" src={move || ducky.get().source()} />

            <Sounds playback_rate=playback_rate volume=volume/>
        </div>
    }
}

#[component]
fn settings(
    show: RwSignal<bool>,
    #[prop(into)] playback_rate: RwSignal<f64>,
    #[prop(into)] volume: RwSignal<f64>,
    selection: RwSignal<Duck>,
) -> impl IntoView {
    let close = move |_| show.set(false);

    let duck_view = move |duck| {
        let select = move |_| {
            if selection.get() != duck {
                selection.set(duck);
                info!("selected Duck::{duck:?}");
            }
        };

        view! {
            <label class="m-2">
                <input type="radio" name="duck"
                    checked={move || selection.get() == duck}
                    on:click=select
                />
                <img class="w-64" src={duck.source()}/>
            </label>
        }
    };

    view! {
        <div class="relative z-10 aria-hidden:invisible" aria-hidden={move || if show.get(){"false"}else{"true"}}>
            <div class="fixed inset-0 bg-gray-500 opacity-75 transition-opacity" on:click=close/>
            <div class="overflow-y-auto fixed inset-0 z-10">
                <div class="flex justify-center items-center p-4 min-h-full text-center">
                    <div class="flex relative flex-col gap-1 p-4 rounded-lg shadow-lg transform shrink bg-slate-600">
                        <p class="mb-2 text-lg font-bold">"Pick your duck!"</p>
                        <Slider
                            label="Playback rate"
                            value=playback_rate
                            default=DEFAULT_PLAYBACK_RATE
                            min=0.15
                            max=2.0
                        />
                        <Slider
                            label="Volume"
                            value=volume
                            default=DEFAULT_VOLUME
                            min=0.01
                            max=1.0
                        />
                        <div class="grid grid-cols-2 shrink">
                            <For
                                each=Duck::iter
                                key=|duck| *duck
                                view=duck_view
                            />
                        </div>
                        <footer class="modal-card-foot">
                            <button class="p-2 rounded-md border-2 bg-slate-700" on:click=close>"Close"</button>
                        </footer>
                    </div>
                </div>
            </div>
        </div>
    }
}

const SOUNDS:&[&str] = &[
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_21.1_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_21.2_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_22.3_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_22.4_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_23.1_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_23.2_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_23.4_preview.mp3",
    "https://cdn.videvo.net/videvo_files/audio/premium/audio0024/watermarked/AnimalDuckCart%20CTE01_24.1_preview.mp3",
];

#[component]
fn sounds(playback_rate: Signal<f64>, volume: Signal<f64>) -> impl IntoView {
    let audio = HtmlAudioElement::new().unwrap_throw();

    create_effect({
        let audio = audio.clone();
        move |_| {
            audio.set_default_playback_rate(playback_rate.get());
            audio.set_volume(volume.get());
        }
    });

    let play = move |_| {
        let audio = audio.clone();
        spawn_local(async move {
            let sound = fastrand::choice(SOUNDS).unwrap();
            audio.set_src(sound);
            JsFuture::from(audio.play().unwrap_throw())
                .await
                .unwrap_throw();
        });
    };

    view! {
        <button class="p-3 text-3xl bg-green-600 rounded-full border-2 border-green-700 transition-all hover:bg-green-700 hover:border-green-600 w-[400px]" on:click=play>"🔊 Play Sound"</button>
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
enum Duck {
    One,
    Two,
    Three,
    Four,
}

impl Duck {
    fn iter() -> impl IntoIterator<Item = Self> {
        [Self::One, Self::Two, Self::Three, Self::Four]
    }

    fn source(self) -> &'static str {
        match self {
            Self::One => "https://images.pexels.com/photos/3759364/pexels-photo-3759364.jpeg",
            Self::Two => "https://images.pexels.com/photos/592677/pexels-photo-592677.jpeg",
            Self::Three => "https://images.pexels.com/photos/5337590/pexels-photo-5337590.jpeg",
            Self::Four => "https://images.pexels.com/photos/226601/pexels-photo-226601.jpeg",
        }
    }
}

#[component]
fn slider(
    label: &'static str,
    value: RwSignal<f64>,
    default: f64,
    min: f64,
    max: f64,
) -> impl IntoView {
    let input = move |event: Event| value.set(event_target_value(&event).parse().unwrap_throw());
    let reset = move |_| value.set(default);

    view! {
        <div class="flex gap-1 self-stretch place-items-stretch text-left">
            <span class="w-32">{label}</span>
            <input class="grow" type="range" min=min max=max step="any" value={move || value.get()} prop:value={move || value.get()} on:change=input/>
            <button class="py-0.5 px-1 rounded-md border-2 transition-all bg-slate-700 hover:bg-slate-600" on:click=reset>"Reset"</button>
        </div>
    }
}
