use std::fmt::Debug;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use leptos::{
    component, create_effect, event_target_value, prelude::*, spawn_local, view, Children, For,
    IntoView,
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;
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

    let ducky = create_stored_signal("ducky", Duck::One);
    let playback_rate = create_stored_signal("playback_rate", DEFAULT_PLAYBACK_RATE);
    let volume = create_stored_signal("volume", DEFAULT_VOLUME);

    view! {
        <div class="flex flex-col items-center w-screen h-screen">
            <div class="grow flex flex-col gap-3 items-center place-content-center">
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
            <Footer/>
        </div>
    }
}

#[component]
fn navbar(#[prop(into)] settings: SignalSetter<bool>) -> impl IntoView {
    let settings = move |_| settings.set(true);

    view! {
        <div class="flex gap-2 place-items-center">
            <div class="text-2xl">"🦆 Quack"</div>
            <button class="btn p-2" on:click=settings>"Settings"</button>
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
            <img
                class="my-8 rounded-xl max-w-[400px]"
                srcset={move || ducky.get().srcset()}
            />

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
            }
        };

        view! {
            <label>
                <input class="hidden peer" type="radio" name="duck"
                    checked={move || selection.get() == duck}
                    on:click=select
                />
                <img
                    class="settings-duck-image"
                    srcset=duck.srcset()
                />
            </label>
        }
    };

    view! {
        <Dialog show=show>
            <p class="settings-header">"Pick your duck!"</p>
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
            <div class="settings-ducks">
                <For
                    each=Duck::iter
                    key=|duck| *duck
                    view=duck_view
                />
            </div>
            <button class="btn p-2" on:click=close>"Close"</button>
        </Dialog>
    }
}

#[component]
fn dialog(children: Children, #[prop(into)] show: Signal<bool>) -> impl IntoView {
    view! {
        <div class="dialog" hidden={move || !show.get()}>
            <div class="dialog-backdrop"/>
            <div class="dialog-content">
                <div class="settings-dialog">
                    {children()}
                </div>
            </div>
        </div>
    }
}

const SOUNDS: &[&str] = &[
    "audio/duck1.mp3",
    "audio/duck2.mp3",
    "audio/duck3.mp3",
    "audio/duck4.mp3",
    "audio/duck5.mp3",
    "audio/duck6.mp3",
    "audio/duck7.mp3",
    "audio/duck8.mp3",
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
        <button class="p-3 text-3xl bg-green-600 rounded-full border-2 border-green-700 transition-all hover:bg-green-700 hover:border-green-600 max-w-[400px]" on:click=play>"🔊 Play Sound"</button>
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

    fn srcset(self) -> &'static str {
        match self {
            Self::One => "image/duck1.webp, image/duck1@2x.webp 2x, image/duck1@4x.webp 4x",
            Self::Two => "image/duck2.webp, image/duck2@2x.webp 2x, image/duck2@4x.webp 4x",
            Self::Three => "image/duck3.webp, image/duck3@2x.webp 2x, image/duck3@4x.webp 4x",
            Self::Four => "image/duck4.webp, image/duck4@2x.webp 2x, image/duck4@4x.webp 4x",
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
        <div class="slider">
            <span class="w-32">{label}</span>
            <input class="grow" type="range" min=min max=max step="any" value={move || value.get()} prop:value={move || value.get()} on:change=input/>
            <button class="btn py-0.5 px-1" on:click=reset>"Reset"</button>
        </div>
    }
}

#[component]
fn footer() -> impl IntoView {
    const PEXELS: &str = "https://www.pexels.com/search/rubber%20duck/";
    const VIDEVO: &str = "https://www.videvo.net/search/?q=animal+duck+cartoon&mode=sound-effects";

    view! {
        <div class="footer my-4 flex-initial">
            "Images from "
            <a class="link" href=PEXELS target="_blank">"Pexels"</a>

            " • "

            "Sounds from "
            <a class="link" href=VIDEVO target="_blank">"Videvo"</a>
        </div>
    }
}

fn create_stored_signal<T>(key: &'static str, default: T) -> RwSignal<T>
where
    T: Clone + Debug + Serialize,
    for<'de> T: Deserialize<'de>,
{
    let signal = create_rw_signal(match LocalStorage::get(key) {
        Ok(value) => value,
        Err(StorageError::KeyNotFound(_)) => default,
        Err(e) => {
            warn!("failed loading `{key}` from storage:\n{e:?}");
            default
        }
    });

    create_effect(move |_| {
        let value = signal.get();
        debug!("changed {key}: {value:.2?}");
        LocalStorage::set(key, value).unwrap_throw();
    });

    signal
}
