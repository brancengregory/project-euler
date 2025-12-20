use anyhow::Result;
use std::iter::Iterator;

fn round_decimal(v: f64, n: usize) -> f64 {
	let factor = 10.0_f64.powi(n as i32);
	(v * factor).round() / factor
}

#[derive(Clone, Copy, Debug)]
struct Point {
	x: f64,
	y: f64,
}

fn distance_sq(a: &Point, b: &Point) -> f64 {
	let dx = b.x as f64 - a.x as f64;
	let dy = b.y as f64 - a.y as f64;
	dx * dx + dy * dy
}

fn shortest_distance(points: &[Point]) -> f64 {
	let mut sorted_points = points.to_vec();
	sorted_points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

	shortest_distance_recursive(&sorted_points).sqrt()
}

fn shortest_distance_recursive(points: &[Point]) -> f64 {
	let n = points.len();

	if n <= 3 {
		return points.iter().enumerate()
			.flat_map(|(i, p1)| {
				points[i + 1..].iter().map(|p2| distance_sq(p1, p2))
			})
			.fold(f64::INFINITY, f64::min);
	}

	let mid = n / 2;
	let (left, right) = points.split_at(mid);
	let mid_x = points[mid].x;

	let d_left = shortest_distance_recursive(left);
	let d_right = shortest_distance_recursive(right);
	let delta_sq = d_left.min(d_right);
	let delta = delta_sq.sqrt();

	let mut strip: Vec<_> = points.iter()
		.filter(|p| (p.x - mid_x).abs() < delta)
		.collect();

	strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

	let strip_min = strip.iter().enumerate()
		.flat_map(|(i, p1)| {
			strip[i + 1..].iter()
				.take_while(|p2| p2.y - p1.y < delta)
				.map(|p2| distance_sq(p1, p2))
		})
		.fold(delta_sq, f64::min);

	strip_min
}

pub fn solve() -> Result<String> {
	let base: i64 = 290_797;
	let modulus: i64 = 50_515_093;
	let k = 2_000_000;

	let mut generator = std::iter::successors(Some(base), |&s| {
		Some((s * s) % modulus)
	});

	let points: Vec<Point> = (0..k).map(|_| {
		Point {
			x: generator.next().unwrap() as f64,
			y: generator.next().unwrap() as f64,
		}
	})
		.collect();

	let shortest_dist = shortest_distance(&points);
	let ans = round_decimal(shortest_dist, 9);

	Ok(ans.to_string())
}

