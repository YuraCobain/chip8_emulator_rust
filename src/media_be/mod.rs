struct Point<T> {
    x: T,
    y: T,
}

trait Gfx {
    fn display_sprite(p: point, s: &[u8]) -> Option<u8>;
}

struct SfmlBackEnd {
}

