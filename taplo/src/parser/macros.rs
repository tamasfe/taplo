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

macro_rules! whitelisted {
    ($self:expr, $kind:ident, $($content:tt)*) => {
        {
            $self.whitelist_token($kind);
            let res = $($content)*;
            $self.blacklist_token($kind);
            res
        }
    };
}