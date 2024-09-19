// Помещаем структуру в отдельный модуль для того чтобы инкапсулировать ее поля
mod graph {
    pub struct Point {
        // используем ключевое слово "pub" для того чтобы иметь возможность получать доступ к структуре вне модуля
        x: f32, // поля структуры автоматически являются приватными
        y: f32,
    }

    impl Point {
        // конструктор
        pub fn new(x: f32, y: f32) -> Point {
            Point { x, y }
        }
        // функция для получения расстояния между двумя точками
        pub fn distance(&self, Point { x, y }: Point) -> f32 {
            ((x - self.x).powf(2.0) + (y - self.y).powf(2.0)).sqrt()
        }
    }
}

fn main() {
    let point1 = graph::Point::new(3.0, 2.0);
    let point2 = graph::Point::new(0.0, 0.0); // создаем два экземпляра структуры
    println!("Distance: {}", point2.distance(point1)); // находим расстояние
}
