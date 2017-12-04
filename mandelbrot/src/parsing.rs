/* Parsing Pair Command-Line Arguments
 * -----------------------------------
 * The program needs to take several command-line arguments controlling the resolu-
 * tion of the image we'll write, and the portion of the Mandelbrot set the image
 * shows.
 * Since these command-line arguments all follow a common form, here's a function to
 * parse them:
 */

use std::std::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0, 0.5"`.
/// 
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is the
/// character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None        => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                             _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}


/* The definition of "parse_pair" is a generic function:
 *
 *      fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
 *
 * You can read the clase <T: FromStr> alaoud as: "For any type T that implements the
 * FromStr trait...". This effectively lets us define an entire family of functions at
 * once:
 */

