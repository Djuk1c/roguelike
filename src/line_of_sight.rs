#[derive(Copy, Clone, Debug)]
pub struct Point2d {
    pub x: u32,
    pub y: u32
}
impl Point2d {
    pub fn new(xp: u32, yp: u32) -> Self {
        Self {
            x: xp,
            y: yp
        }
    }
}

pub fn get_line(a: Point2d, b: Point2d) -> Vec<Point2d> {
    let mut points = Vec::<Point2d>::new();
    let mut x1 = a.x as i32;
    let mut y1 = a.y as i32;
    let mut x2 = b.x as i32;
    let mut y2 = b.y as i32;
    let is_steep = (y2-y1).abs() > (x2-x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);   
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2+1) {
        if is_steep {
            points.push(Point2d{x:y as u32, y:x as u32});
        } else {
            points.push(Point2d{x:x as u32, y:y as u32});
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len()/2) {
            let end = points.len()-1;
            points.swap(i, end-i);
        }
    }
    points
}