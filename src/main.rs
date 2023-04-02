trait Info {
    fn name(&self) -> &str;
}

trait InfoProvider {
    type Item<'a>: Info + 'a
    where
        Self: 'a;
    fn info<'a>(&'a self) -> Self::Item<'a>;
}

struct Thing {
    name: String,
}

struct ThingInfo<'a> {
    thing: &'a Thing,
}

impl Info for ThingInfo<'_> {
    fn name(&self) -> &str {
        &self.thing.name
    }
}

impl InfoProvider for Thing {
    type Item<'a> = ThingInfo<'a>;

    fn info<'a>(&'a self) -> Self::Item<'a> {
        ThingInfo { thing: self }
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;

impl InfoProvider for UnassociatedThing {
    type Item<'a> = UnassociatedThingInfo;

    fn info(&self) -> Self::Item<'_> {
        UnassociatedThingInfo {}
    }
}
impl Info for UnassociatedThingInfo {
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
        IP: InfoProvider,
    {
        let info = obj.info();
        info.name().into()
    }

    // fn get_info<'a, IP>(obj: &'a IP) -> impl Info + 'a
    // where
    //     IP: InfoProvider<'a>,
    // {
    //     obj.info()
    // }

    // #[test]
    // fn does_not_compile() {
    //     let info = {
    //         let obj = UnassociatedThing {  };
    //         get_info(&obj)
    //     };
    //     assert_eq!(info.name(),"John");
    // }

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
