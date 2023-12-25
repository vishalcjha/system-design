use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use leptos::*;
use wasm_bindgen::closure::Closure;
use web_sys::js_sys::Math::random;

use crate::graphics::{
    canvas, canvas_context, circle_tag::draw_wrapped_number, clear_canvas, draw_grid_lines,
    request_animation_frame_custom,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
enum CountedDirection {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
}

impl CountedDirection {
    fn new() -> CountedDirection {
        let random = (random() * 100.) as u64;
        match random {
            num @ _ if num < 25 => CountedDirection::Up(0),
            num @ _ if num < 50 => CountedDirection::Down(0),
            num @ _ if num < 75 => CountedDirection::Right(0),
            _ => CountedDirection::Left(0),
        }
    }

    fn offset(&self) -> (f64, f64) {
        match self {
            CountedDirection::Up(_) => (0., -1.),
            CountedDirection::Down(_) => (0., 1.),
            CountedDirection::Right(_) => (1., 0.),
            CountedDirection::Left(_) => (-1., 0.),
        }
    }

    fn increment(self) -> CountedDirection {
        use CountedDirection::*;
        match self {
            CountedDirection::Up(x) => Up(x + 1),
            CountedDirection::Down(x) => Down(x + 1),
            CountedDirection::Right(x) => Right(x + 1),
            CountedDirection::Left(x) => Left(x + 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Car {
    position: Cell<(f64, f64)>,
    direction: Cell<CountedDirection>,
}

impl Car {
    fn new() -> Car {
        let pos = get_random_x_y();

        Car {
            position: Cell::new(pos),
            direction: Cell::new(CountedDirection::new()),
        }
    }

    fn is_with_in_grid(&self) -> bool {
        let (max_x, max_y) = get_canvas_x_y();
        let (pos_x, pos_y) = self.position.get();
        pos_x > 0. && pos_y > 0. && pos_x < max_x && pos_y < max_y
    }
}

fn get_random_x_y() -> (f64, f64) {
    let canvas = canvas();
    let width = canvas.width() as f64 / 2.;
    let height = canvas.height() as f64;
    let grid_y = height * random();
    let grid_x = width * random();
    (grid_x, grid_y)
}

fn get_canvas_x_y() -> (f64, f64) {
    let canvas = canvas();
    let width = canvas.width() as f64 / 2.;
    let height = canvas.height() as f64;
    (width, height)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Person {
    position: (f64, f64),
}

impl Person {
    pub fn new() -> Person {
        Person {
            position: get_random_x_y(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    persons: Vec<Person>,
    cars: Vec<Car>,
}

impl Grid {
    fn new() -> Grid {
        let persons = (0..3)
            .into_iter()
            .map(|_| Person::new())
            .collect::<Vec<_>>();

        Grid {
            persons,
            cars: (0..4).into_iter().map(|_| Car::new()).collect(),
        }
    }
}

#[component]
pub(super) fn GridComponent() -> impl IntoView {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    create_effect(move |_| {
        let mut grid = Grid::new();
        let f = g.clone();
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let context = canvas_context();
            clear_canvas();
            draw_grid_lines(16);
            for (index, person) in grid.persons.iter().enumerate() {
                draw_wrapped_number(
                    &context,
                    person.position.0,
                    person.position.1,
                    15.,
                    format!("P{}", index + 1),
                );
            }

            for (index, car) in grid.cars.iter().enumerate() {
                let (x, y) = car.position.get();
                draw_wrapped_number(&context, x, y, 15., format!("C{}", index + 1));
                let direction = car.direction.get();
                let (off_x, off_y) = direction.offset();
                car.position.set((x + off_x, y + off_y));
                if match direction {
                    CountedDirection::Up(x) => x,
                    CountedDirection::Down(x) => x,
                    CountedDirection::Right(x) => x,
                    CountedDirection::Left(x) => x,
                } > 60
                {
                    car.direction.set(CountedDirection::new());
                } else {
                    car.direction.set(direction.increment());
                }
            }

            grid.cars.retain(|car| car.is_with_in_grid());
            while grid.cars.len() < 4 {
                grid.cars.push(Car::new());
            }

            if let Some(x) = f.borrow().as_ref() {
                request_animation_frame_custom(x);
            }
        }));

        request_animation_frame_custom(g.borrow().as_ref().unwrap());
    });

    // this is not safe. But without this not able to stop animation.
    on_cleanup(move || {
        leptos::logging::log!("In cleanup");
        *f.borrow_mut() = None;
    });

    view! {}
}
