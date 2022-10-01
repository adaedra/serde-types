pub trait Keys: Sized + Clone + PartialEq + 'static {
    const VALUES: &'static [(&'static str, Self)];

    fn from_str(s: &str) -> Option<Self> {
        for (k, v) in Self::VALUES {
            if *k == s {
                return Some(v.clone());
            }
        }

        None
    }

    fn as_str(&self) -> &'static str {
        for (k, v) in Self::VALUES {
            if v == self {
                return k;
            }
        }

        unreachable!()
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
            const VALUES: &'static [(&'static str, Self)] = &[
                $( ($v, $name::$k), )*
            ];
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
}
