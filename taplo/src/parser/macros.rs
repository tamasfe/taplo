macro_rules! with_node {
    ($builder:expr, $kind:ident, $($content:tt)*) => {
        {
        $builder.start_node($kind.into());
        let res = $($content)*;
        $builder.finish_node();
        res
        }
    };
}
