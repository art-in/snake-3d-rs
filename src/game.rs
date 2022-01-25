use std::{cell::RefCell, ops::Deref, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{
    actions::{control_actions, game_actions},
    drawers::scene_drawer,
    helpers::{
        canvas::resize_canvas,
        dom::{subscribe_to_event, unsubscribe_from_event},
        raf::start_request_animation_frame_loop,
    },
    models::{GameState, Size},
};

#[derive(Default)]
pub struct Game {
    state: GameState,

    // closures are saved so we can unsubscribe later and free up resources.
    // unsubscribing never really happens right now as game loops forever,
    // so we could simply .forget() them and do not save anything, but i leave
    // it anyway just as an example of more mature approach
    on_resize: Option<Closure<dyn FnMut(web_sys::Event)>>,
    on_keydown: Option<Closure<dyn FnMut(web_sys::Event)>>,
    on_mousedown: Option<Closure<dyn FnMut(web_sys::Event)>>,
    on_mouseup: Option<Closure<dyn FnMut(web_sys::Event)>>,
    on_mousemove: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

// 1. create wrapper-type, since game only used through pointer.
//    use custom type instead of type alias to be able to impl methods for it
// 2. allocate game on the heap because it should outlive main() and be
//    referenced from RAF loop and event handler closures
// 3. game is reference counted pointer (Rc) and not unique pointer (Box)
//    because we need to clone it to all the closures
pub struct GameRc(Rc<RefCell<Game>>);

// implement deref to avoid ".0" on every access to its only field
impl Deref for GameRc {
    type Target = Rc<RefCell<Game>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// override clone so returned typed is correct GameRc and not Rc<..>
impl Clone for GameRc {
    fn clone(&self) -> Self {
        GameRc(self.0.clone())
    }
}

impl GameRc {
    pub fn start() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .query_selector("canvas")
            .unwrap()
            .expect("failed to find canvas element")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let game = GameRc(Rc::new(RefCell::new(Game::default())));

        game_actions::init_game_state(&mut game.borrow_mut().state);
        scene_drawer::init_scene_drawer(&mut game.borrow_mut().state, canvas);

        game.on_resize();
        game.subscribe();

        // start game loop
        start_request_animation_frame_loop(Box::new(move || {
            game.loop_();
            true
        }));
    }

    pub fn loop_(&self) {
        game_actions::update_game_state_loop(&mut self.borrow_mut().state);
        scene_drawer::draw_scene_loop(&mut self.borrow_mut().state);
    }

    fn subscribe(&self) {
        let game = self.clone();
        subscribe_to_event(
            "resize",
            Box::new(move |_| game.on_resize()),
            &mut self.borrow_mut().on_resize,
        );
        let game = self.clone();
        subscribe_to_event(
            "keydown",
            Box::new(move |event| {
                let event = event.dyn_into::<web_sys::KeyboardEvent>().unwrap();
                game.on_keydown(event);
            }),
            &mut self.borrow_mut().on_keydown,
        );
        let game = self.clone();
        subscribe_to_event(
            "mousedown",
            Box::new(move |_| game.on_mousedown()),
            &mut self.borrow_mut().on_mousedown,
        );
        let scene = self.clone();
        subscribe_to_event(
            "mouseup",
            Box::new(move |_| scene.on_mouseup()),
            &mut self.borrow_mut().on_mouseup,
        );
        let scene = self.clone();
        subscribe_to_event(
            "mousemove",
            Box::new(move |event| {
                let event = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                scene.on_mousemove(event);
            }),
            &mut self.borrow_mut().on_mousemove,
        );
    }

    #[allow(dead_code)]
    fn unsubscribe(&self) {
        unsubscribe_from_event("resize", &mut self.borrow_mut().on_resize);
        unsubscribe_from_event("keydown", &mut self.borrow_mut().on_keydown);
        unsubscribe_from_event("mousedown", &mut self.borrow_mut().on_mousedown);
        unsubscribe_from_event("mouseup", &mut self.borrow_mut().on_mouseup);
        unsubscribe_from_event("mousemove", &mut self.borrow_mut().on_mousemove);
    }

    fn on_resize(&self) {
        let window = web_sys::window().unwrap();
        let body = window.document().unwrap().body().unwrap();

        let state = &self.borrow().state;
        let canvas = state.scene.canvas.as_ref().unwrap();

        let css_size = Size {
            width: body.client_width() as f64,
            height: body.client_height() as f64,
        };

        resize_canvas(canvas, css_size, window.device_pixel_ratio()).unwrap();
    }

    fn on_keydown(&self, event: web_sys::KeyboardEvent) {
        control_actions::on_keydown(&mut self.borrow_mut().state, &event.code());
    }

    fn on_mousedown(&self) {
        control_actions::on_mousedown(&mut self.borrow_mut().state);
    }

    fn on_mouseup(&self) {
        control_actions::on_mouseup(&mut self.borrow_mut().state);
    }

    fn on_mousemove(&self, event: web_sys::MouseEvent) {
        control_actions::on_mousemove(
            &mut self.borrow_mut().state,
            crate::models::Point2D {
                x: event.client_x() as f64,
                y: event.client_y() as f64,
            },
        );
    }
}
