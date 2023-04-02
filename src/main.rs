trait Info<'a> {
    fn name(&self) -> &str;
}

trait InfoProvider<'a> {
    type Item: Info<'a>;
    fn info(&'a self) -> Self::Item;
}

struct Thing {
    name: String,
}

struct ThingInfo<'a> {
    thing: &'a Thing,
}

impl Info<'_> for ThingInfo<'_> {
    fn name(&self) -> &str {
        &self.thing.name
    }
}

impl<'a> InfoProvider<'a> for Thing {
    type Item = ThingInfo<'a>;

    fn info(&'a self) -> Self::Item {
        ThingInfo { thing: self }
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;

impl InfoProvider<'_> for UnassociatedThing {
    type Item = UnassociatedThingInfo;

    fn info(&'_ self) -> Self::Item {
        UnassociatedThingInfo {}
    }
}
impl Info<'_> for UnassociatedThingInfo {
    fn name(&self) -> &str {
        "Hello World"
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let obj = Thing {
            name: "John".into(),
        };
        let info = obj.info();
        assert_eq!(info.name(), "John");
    }

    fn generic_use_of_info<'a, IP>(obj: &'a IP) -> String
    where
        IP: InfoProvider<'a>,
    {
        let info = obj.info();
        info.name().into()
    }

    #[test]
    fn test_generic_use_of_info() {
        let obj = Thing {
            name: "John".into(),
        };
        let ret = generic_use_of_info(&obj);
        assert_eq!(ret, "John");
    }

    #[test]
    fn test_unassociated() {
        let obj = UnassociatedThing {};
        let ret = generic_use_of_info(&obj);
        assert_eq!(ret, "Hello World");
    }
}
