/// A point in 2D space with x and y coordinates
/// 
/// # Examples
/// 
/// ```
/// use p22::figures::Point;
/// let point = Point::new(3.0, 4.0);
/// assert_eq!(point.x, 3.0);
/// assert_eq!(point.y, 4.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new Point with the given x and y coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::Point;
    /// let point = Point::new(1.0, 2.0);
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, 2.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the distance between this point and another point
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::Point;
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.distance(&p2), 5.0);
    /// ```
    pub fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

/// A circle defined by a center point and radius
///
/// # Examples
///
/// ```
/// use p22::figures::{Circle, Point};
/// let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
/// assert_eq!(circle.area(), 78.53981633974483);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    /// Creates a new Circle with the given center point and radius
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Circle, Point};
    /// let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
    /// assert_eq!(circle.radius, 5.0);
    /// ```
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
    
    /// Calculates the perimeter (circumference) of the circle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Circle, Point};
    /// let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
    /// assert_eq!(circle.perimeter(), 31.41592653589793);
    /// ```
    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    /// Calculates the area of the circle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Circle, Point};
    /// let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
    /// assert_eq!(circle.area(), 78.53981633974483);
    /// ```
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

/// A triangle defined by three points in 2D space
///
/// # Examples
///
/// ```
/// use p22::figures::{Triangle, Point};
/// let triangle = Triangle::new(
///     Point::new(0.0, 0.0),
///     Point::new(3.0, 0.0),
///     Point::new(3.0, 4.0)
/// );
/// assert_eq!(triangle.area(), 6.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    /// Creates a new Triangle with the given three points
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Triangle, Point};
    /// let triangle = Triangle::new(
    ///     Point::new(0.0, 0.0),
    ///     Point::new(3.0, 0.0),
    ///     Point::new(3.0, 4.0)
    /// );
    /// ```
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
    
    /// Calculates the perimeter of the triangle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Triangle, Point};
    /// let triangle = Triangle::new(
    ///     Point::new(0.0, 0.0),
    ///     Point::new(3.0, 0.0),
    ///     Point::new(3.0, 4.0)
    /// );
    /// assert_eq!(triangle.perimeter(), 12.0);
    /// ```
    pub fn perimeter(&self) -> f64 {
        let ab = self.a.distance(&self.b);
        let bc = self.b.distance(&self.c);
        let ca = self.c.distance(&self.a);
        
        ab + bc + ca
    }
    
    /// Calculates the area of the triangle using Heron's formula
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Triangle, Point};
    /// let triangle = Triangle::new(
    ///     Point::new(0.0, 0.0),
    ///     Point::new(3.0, 0.0),
    ///     Point::new(3.0, 4.0)
    /// );
    /// assert_eq!(triangle.area(), 6.0);
    /// ```
    pub fn area(&self) -> f64 {
        let ab = self.a.distance(&self.b);
        let bc = self.b.distance(&self.c);
        let ca = self.c.distance(&self.a);
        
        let s = (ab + bc + ca) / 2.0;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }
}

/// A rectangle defined by top-left and bottom-right points
///
/// # Examples
///
/// ```
/// use p22::figures::{Rectangle, Point};
/// let rect = Rectangle::new(
///     Point::new(0.0, 5.0),
///     Point::new(5.0, 0.0)
/// );
/// assert_eq!(rect.area(), 25.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    /// Creates a new Rectangle with the given top-left and bottom-right points
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Rectangle, Point};
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// );
    /// ```
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self { top_left, bottom_right }
    }
    
    /// Returns the width of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Rectangle, Point};
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// );
    /// assert_eq!(rect.width(), 5.0);
    /// ```
    pub fn width(&self) -> f64 {
        self.bottom_right.x - self.top_left.x
    }
    
    /// Returns the height of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Rectangle, Point};
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// );
    /// assert_eq!(rect.height(), 5.0);
    /// ```
    pub fn height(&self) -> f64 {
        self.top_left.y - self.bottom_right.y
    }
    
    /// Calculates the perimeter of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Rectangle, Point};
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// );
    /// assert_eq!(rect.perimeter(), 20.0);
    /// ```
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width() + self.height())
    }
    
    /// Calculates the area of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Rectangle, Point};
    /// let rect = Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// );
    /// assert_eq!(rect.area(), 25.0);
    /// ```
    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }
}

/// An enum representing different shapes
///
/// # Examples
///
/// ```
/// use p22::figures::{Shape, Point, Circle};
/// let shape = Shape::Circle(Circle::new(Point::new(0.0, 0.0), 5.0));
/// assert_eq!(shape.area(), 78.53981633974483);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Shape {
    /// Calculates the perimeter of any shape
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Shape, Rectangle, Point};
    /// let shape = Shape::Rectangle(Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// ));
    /// assert_eq!(shape.perimeter(), 20.0);
    /// ```
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Point(_) => 0.0,
            Shape::Circle(circle) => circle.perimeter(),
            Shape::Triangle(triangle) => triangle.perimeter(),
            Shape::Rectangle(rectangle) => rectangle.perimeter(),
        }
    }

    /// Calculates the area of any shape
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::figures::{Shape, Rectangle, Point};
    /// let shape = Shape::Rectangle(Rectangle::new(
    ///     Point::new(0.0, 5.0),
    ///     Point::new(5.0, 0.0)
    /// ));
    /// assert_eq!(shape.area(), 25.0);
    /// ```
    pub fn area(&self) -> f64 {
        match self {
            Shape::Point(_) => 0.0,
            Shape::Circle(circle) => circle.area(),
            Shape::Triangle(triangle) => triangle.area(),
            Shape::Rectangle(rectangle) => rectangle.area(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_rectangle_methods() {
        let rect = Rectangle::new(
            Point::new(0.0, 5.0),
            Point::new(5.0, 0.0)
        );
        assert_eq!(rect.width(), 5.0);
        assert_eq!(rect.height(), 5.0);
        assert_eq!(rect.perimeter(), 20.0);
        assert_eq!(rect.area(), 25.0);
    }

    #[test]
    fn test_triangle_methods() {
        let triangle = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 4.0)
        );
        assert_eq!(triangle.perimeter(), 12.0);
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_circle_methods() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        assert_eq!(circle.perimeter(), 31.41592653589793);
        assert_eq!(circle.area(), 78.53981633974483);
    }
    
    #[test]
    fn test_shape_methods() {
        let point = Shape::Point(Point::new(1.0, 2.0));
        assert_eq!(point.perimeter(), 0.0);
        assert_eq!(point.area(), 0.0);
        
        let rect = Shape::Rectangle(Rectangle::new(
            Point::new(0.0, 5.0),
            Point::new(5.0, 0.0)
        ));
        assert_eq!(rect.perimeter(), 20.0);
        assert_eq!(rect.area(), 25.0);

        let circle = Shape::Circle(Circle::new(Point::new(0.0, 0.0), 5.0));
        assert_eq!(circle.perimeter(), 31.41592653589793);
        assert_eq!(circle.area(), 78.53981633974483);
        
        let triangle = Shape::Triangle(Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 4.0)
        ));
        assert_eq!(triangle.perimeter(), 12.0);
        assert_eq!(triangle.area(), 6.0);
    }
}