use serde::de;
use std::{fmt, marker::PhantomData};

pub trait Keys: Sized + PartialEq + 'static {
    const NAMES: &'static [&'static str];

    fn from_str(s: &str) -> Option<Self>;
    fn as_str(&self) -> &'static str;
}

struct Visitor<K>(PhantomData<K>)
where
    K: Keys;

pub fn visitor_for<'de, K>() -> impl de::Visitor<'de, Value = K>
where
    K: Keys,
{
    Visitor::<K>(PhantomData)
}

impl<'de, K> de::Visitor<'de> for Visitor<K>
where
    K: Keys
{
    type Value = K;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "one of ")?;

        let mut first = true;
        for k in K::NAMES {
            if !first {
                write!(f, ", ")?;
                first = false;
            }

            write!(f, r#""{k}""#)?;
        }

        Ok(())
    }

    fn visit_str<E>(self, s: &str) -> Result<K, E>
    where
        E: de::Error,
    {
        K::from_str(s).ok_or_else(|| E::unknown_field(s, K::NAMES))
    }
}

#[macro_export]
macro_rules! keys {
    ($vis:vis $name:ident { $($k:ident ( $v:expr ) ,)+ }) => {
        #[derive(Clone, PartialEq, Eq, Debug, Hash)]
        $vis enum $name {
            $( $k, )*
        }

        impl $crate::keys::Keys for $name {
            const NAMES: &'static [&'static str] = &[
                $( $v, )*
            ];

            fn from_str(s: &str) -> Option<$name> {
                match s {
                    $( $v => Some($name::$k), )*
                    _ => None,
                }
            }

            fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$k => $v, )*
                }
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(d: D) -> Result<$name, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                d.deserialize_str($crate::keys::visitor_for::<$name>())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::Keys;

    keys!(pub Color {
        Red("red"),
        Green("green"),
        Blue("blue"),
    });

    #[test]
    fn from_str() {
        assert_eq!(Some(Color::Blue), Color::from_str("blue"));
        assert_eq!(None, Color::from_str("purple"));
    }

    #[test]
    fn to_str() {
        assert_eq!("blue", Color::Blue.as_str());
    }

    #[test]
    fn deserializes() {
        let json = serde_json::json!("blue");
        assert_eq!(Color::Blue, serde_json::from_value(json).unwrap());
    }

    #[test]
    fn deserializes_hashmap() {
        use std::collections::HashMap;

        let json = serde_json::json!({ "blue": 0, "red": 100, "green": 200, });
        let data: HashMap<Color, u8> = serde_json::from_value(json).unwrap();

        assert_eq!(Some(&0), data.get(&Color::Blue));
        assert_eq!(Some(&100), data.get(&Color::Red));
        assert_eq!(Some(&200), data.get(&Color::Green));
    }
}
