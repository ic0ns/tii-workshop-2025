use p22::figures::{Point, Circle, Triangle, Rectangle, Shape};

#[test]
fn test_point_functionality() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    assert_eq!(p1.distance(&p2), 5.0);
    
    // Test accessors
    let p3 = Point::new(2.5, 3.5);
    assert_eq!(p3.x, 2.5);
    assert_eq!(p3.y, 3.5);
}

#[test]
fn test_rectangle_functionality() {
    let rect = Rectangle::new(
        Point::new(0.0, 5.0),
        Point::new(5.0, 0.0)
    );
    
    assert_eq!(rect.width(), 5.0);
    assert_eq!(rect.height(), 5.0);
    assert_eq!(rect.perimeter(), 20.0);
    assert_eq!(rect.area(), 25.0);
    
    // Test accessors
    assert_eq!(rect.top_left.x, 0.0);
    assert_eq!(rect.top_left.y, 5.0);
    assert_eq!(rect.bottom_right.x, 5.0);
    assert_eq!(rect.bottom_right.y, 0.0);
}

#[test]
fn test_triangle_functionality() {
    let triangle = Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(3.0, 4.0)
    );
    
    assert_eq!(triangle.perimeter(), 12.0);
    assert_eq!(triangle.area(), 6.0);
    
    // Test accessors
    assert_eq!(triangle.a.x, 0.0);
    assert_eq!(triangle.a.y, 0.0);
    assert_eq!(triangle.b.x, 3.0);
    assert_eq!(triangle.b.y, 0.0);
    assert_eq!(triangle.c.x, 3.0);
    assert_eq!(triangle.c.y, 4.0);
}

#[test]
fn test_circle_functionality() {
    let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
    
    assert_eq!(circle.perimeter(), 31.41592653589793);
    assert_eq!(circle.area(), 78.53981633974483);
    
    // Test accessors
    assert_eq!(circle.center.x, 0.0);
    assert_eq!(circle.center.y, 0.0);
    assert_eq!(circle.radius, 5.0);
}

#[test]
fn test_shape_functionality() {
    // Test Point
    let point = Shape::Point(Point::new(1.0, 2.0));
    assert_eq!(point.perimeter(), 0.0);
    assert_eq!(point.area(), 0.0);
    
    // Test Rectangle
    let rect = Shape::Rectangle(Rectangle::new(
        Point::new(0.0, 5.0),
        Point::new(5.0, 0.0)
    ));
    assert_eq!(rect.perimeter(), 20.0);
    assert_eq!(rect.area(), 25.0);

    // Test Circle
    let circle = Shape::Circle(Circle::new(Point::new(0.0, 0.0), 5.0));
    assert_eq!(circle.perimeter(), 31.41592653589793);
    assert_eq!(circle.area(), 78.53981633974483);
    
    // Test Triangle
    let triangle = Shape::Triangle(Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(3.0, 4.0)
    ));
    assert_eq!(triangle.perimeter(), 12.0);
    assert_eq!(triangle.area(), 6.0);
}