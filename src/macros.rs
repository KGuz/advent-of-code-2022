macro_rules! parse {
    ($s: ident as $t: ty) => {
        $s.parse::<$t>().expect("Parsing error")
    };
    ($s: expr) => {
        $s.parse().expect("Parsing error")
    };
    ($s: ident else $d: expr) => {
        $s.parse().unwrap_or($d)
    };
}
pub(crate) use parse;

macro_rules! re {
    ($($s: literal),*) => {
        regex::Regex::new(concat![$($s),*]).unwrap()
    };
}
pub(crate) use re;

macro_rules! captures {
    ($s: expr, $re: expr) => {{
        use itertools::Itertools;
        $re.captures($s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str())
            .collect_tuple()
            .unwrap()
    }};
}
pub(crate) use captures;

macro_rules! map {
    () => { std::collections::HashMap::new() };
    ($(($k: expr, $v: expr)),*) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}
pub(crate) use map;

macro_rules! set {
    () => { std::collections::HashSet::new() };
    ($($v: expr),*) => {{
        #[allow(unused_mut)]
        let mut queue = std::collections::HashSet::new();
        $(queue.insert($v);)*
        queue
    }};
}
pub(crate) use set;

macro_rules! queue {
    () => { std::collections::VecDeque::new() };
    ($($v: expr),*) => {{
        let mut queue = std::collections::VecDeque::new();
        $(queue.push_back($v);)*
        queue
    }};
}
pub(crate) use queue;

macro_rules! modules {
    ($($m: ident),*) => {$(
        pub mod $m;
    )*};
}
pub(crate) use modules;
