//! This module contains the actual implementation of the turtle concept.
//!
//! The `Turtle` starts in the middle of the screen and can be given commands
//! such as `forward(100)` or `left(90)`. While moving across the screen, the
//! turtle draws its path on the canvas. Based on this primitive movements, you
//! can build more complex commands and draw nice patterns.
//!
//! A `Turtle` always has a `TurtleScreen` owned. This `TurtleScreen` must be
//! given to `Turtle::new()`
//!
//! # Example
//!
//! ```
//! use rurtle::graphic::TurtleScreen;
//! use rurtle::turtle::Turtle;
//! let screen = TurtleScreen::new((640, 480), "Turtle Demo");
//! let mut turtle = Turtle::new(screen);
//! for _ in (0..4) {
//!     turtle.forward(100.0);
//!     turtle.right(90.0);
//! }
//! ```
use super::graphic::TurtleScreen;
use super::graphic::color;

#[derive(Debug)]
enum PenState {
    PenUp,
    PenDown,
}

/// The `Turtle` struct is the thing that actually provides the methods to walk
/// on the screen
pub struct Turtle {
    screen: TurtleScreen,
    orientation: f32,
    position: (f32, f32),
    color: color::Color,
    pen: PenState,
}

impl Turtle {
    /// Construct a new Turtle. Moves the TurtleScreen.
    pub fn new(screen: TurtleScreen) -> Turtle {
        Turtle {
            screen: screen,
            orientation: 0.0,
            position: (0.0, 0.0),
            color: color::BLACK,
            pen: PenState::PenDown,
        }
    }

    /// Move the turtle to the given position. Depending on whether the pen is
    /// up or down, also draw the line. This function is used internally to
    /// implement everything else
    fn goto(&mut self, x: f32, y: f32) {
        let start_position = self.position;
        if let PenState::PenDown = self.pen {
            self.screen.add_line(start_position, (x, y), self.color);
        }
        self.position = (x, y);
        self.screen.turtle_position = self.position;
        self.screen.draw_and_update();
    }

    /// Return a reference to the underlaying `TurtleScreen` object
    pub fn get_screen(&mut self) -> &mut TurtleScreen {
        &mut self.screen
    }

    /// Turn the turtle by the given amount. Positive means counter-clockwise,
    /// negative means clockwise. The angle is given in degrees. This function
    /// is used internally.
    fn turn(&mut self, deg: f32) {
        let orientation = self.orientation;
        self.set_orientation(orientation + deg);
    }

    /// Take the length of a path and return the (delta_x, delta_y) attributes
    /// that you need to "walk" when heading in the current direction.
    fn length_to_vector(&self, length: f32) -> (f32, f32) {
        let orientation_rad = ::std::f32::consts::PI * self.orientation / 180.0;
        let delta_x = orientation_rad.sin() * length;
        let delta_y = orientation_rad.cos() * length;
        (-delta_x, delta_y)
    }

    /// Clear the screen. Note that this only removes the drawn lines, it does
    /// not change the turtle's position or orientation.
    pub fn clear(&mut self) {
        self.screen.clear();
    }

    /// Move the turtle forward by the given length
    pub fn forward(&mut self, length: f32) {
        let (x, y) = self.position;
        let (dx, dy) = self.length_to_vector(length);
        self.goto(x + dx, y + dy);
    }

    /// Move the turtle backward by the given length
    pub fn backward(&mut self, length: f32) {
        let (x, y) = self.position;
        let (dx, dy) = self.length_to_vector(length);
        self.goto(x - dx, y - dy);
    }

    /// Turn the turtle left
    pub fn left(&mut self, deg: f32) {
        self.turn(deg);
    }

    /// Turn the turtle right
    pub fn right(&mut self, deg: f32) {
        self.turn(-deg);
    }

    /// "Lifts" the pen so that no lines are drawn anymore
    pub fn pen_up(&mut self) {
        self.pen = PenState::PenUp;
    }

    /// Sinks the pen again so that lines are drawn
    pub fn pen_down(&mut self) {
        self.pen = PenState::PenDown;
    }

    /// Set the turtle's color. New lines will be drawn using that color but
    /// existing lines will remain in their color. `red`, `green` and `blue` are
    /// given as floats in the range [0; 1], where 0 means nothing and 1 full
    /// (like #FF in HTML).
    pub fn set_color(&mut self, red: f32, green: f32, blue: f32) {
        self.color = (red, green, blue, 1.0);
        self.screen.turtle_color = self.color;
        self.screen.draw_and_update();
    }

    /// Set the background color of the screen.
    pub fn set_background_color(&mut self, red: f32, green: f32, blue: f32) {
        self.screen.background_color = (red, green, blue, 1.);
        self.screen.draw_and_update();
    }

    /// Directly move the turtle to the given point without changing the
    /// direction. Draws a line if the pen is down. Note that the origin (0, 0)
    /// is in the center of the screen with positive coordinates being right/top
    /// and negative ones left/down.
    pub fn teleport(&mut self, x: f32, y: f32) {
        self.goto(x, y)
    }

    /// Set the turtle's orientation in degrees with 0 being faced north and
    /// positive degrees counting counter-clockwise.
    pub fn set_orientation(&mut self, deg: f32) {
        self.orientation = deg % 360.0;
        self.screen.turtle_orientation = self.orientation;
        self.screen.draw_and_update();
    }

    /// Move the turtle to the origin and set its orientation to 0
    pub fn home(&mut self) {
        self.teleport(0.0, 0.0);
        self.set_orientation(0.0);
    }

    /// Return the turtle's orientation
    pub fn get_orientation(&self) -> f32 { self.orientation }
    /// Return the turtle's position
    pub fn get_position(&self) -> (f32, f32) { self.position }

    /// Hide the turtle so it won't be drawn on the screen
    pub fn hide(&mut self) {
        self.screen.turtle_hidden = true;
        self.screen.draw_and_update();
    }

    /// Show the turtle again after it has been hidden
    pub fn show(&mut self) {
        self.screen.turtle_hidden = false;
        self.screen.draw_and_update();
    }

    /// Returns true if the turtle is currently hidden
    pub fn is_hidden(&self) -> bool {
        self.screen.turtle_hidden
    }

    /// Write the text on the screen. The lower-left corner of the Text starts
    /// where the turtle is.
    pub fn write(&mut self, text: &str) {
        self.screen.add_text(self.position, self.orientation, self.color, text);
    }

    /// Perform a floodfill at the current turtle position
    pub fn flood(&mut self) {
        self.screen.floodfill(self.position, self.color);
    }
}
