// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}
impl<'a> Default for ColorClassicStruct<'a> {
    fn default() -> Self { Self { name: "green", hex: "#00FF00" } }
}

struct ColorTupleStruct<'a>(&'a str, &'a str);
impl<'a> Default for ColorTupleStruct<'a> {
    fn default() -> Self { Self ( "green", "#00FF00" ) }
}

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct::default();

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // For more fun, use the field initialization shorthand.
        // let green =
        let green = ColorTupleStruct::default();

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // let unit_struct =
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
