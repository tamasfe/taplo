macro_rules! dom_node_from {
    ($($inner:ty => $name:ident),*) => {
        $(
            impl From<$inner> for Node {
                fn from(inner: $inner) -> Self {
                    Node::$name(inner)
                }
            }
        )*
    };
}

macro_rules! dom_node {
    (
    $(#[$attrs:meta])*
    pub struct $name:ident {
        $(
            $(#[$field_attrs:meta])*
            $field_name:ident: $field_ty:ty,
        )*
    }) => {
    $(#[$attrs])*
    pub struct $name {
            syntax: SyntaxElement,
            $(
                $(#[$field_attrs])*
                $field_name: $field_ty,
            )*
        }

        impl NodeSyntax for $name {
            fn syntax(&self) -> SyntaxElement {
                self.syntax.clone()
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.syntax.fmt(f)
            }
        }
    }
}

macro_rules! dom_node_no_display {
    (
    $(#[$attrs:meta])*
    pub struct $name:ident {
        $(
            $(#[$field_attrs:meta])*
            $field_name:ident: $field_ty:ty,
        )*
    }) => {
    $(#[$attrs])*
    pub struct $name {
            syntax: SyntaxElement,
            $(
                $(#[$field_attrs])*
                $field_name: $field_ty,
            )*
        }

        impl NodeSyntax for $name {
            fn syntax(&self) -> SyntaxElement {
                self.syntax.clone()
            }
        }
    }
}

macro_rules! dom_primitives {
    ($($($kind:ident)|* => $ast:ident),*) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct $ast(SyntaxElement);
            impl Cast for $ast {
                #[allow(unused)]
                fn cast(elem: SyntaxElement) -> Option<Self> {
                    match &elem {
                        rowan::NodeOrToken::Token(t) => {
                            match t.kind() {
                                $($kind)|* => {
                                    Some(Self(elem))
                                }
                                _ => {
                                    None
                                }
                            }
                        },
                        _ => {
                            None
                        }
                    }
                }
            }

            impl NodeSyntax for $ast {
                fn syntax(&self) -> SyntaxElement {
                    self.0.clone()
                }
            }
        )*
    };
}

macro_rules! dom_sealed {
    ($($id:ty,)*) => {
        $(impl Sealed for $id {})*
    };
}

macro_rules! rewrite_node_from {
    ($($inner:ty => $name:ident),*) => {
        $(
            impl From<$inner> for RewriteNode {
                fn from(inner: $inner) -> Self {
                    RewriteNode::New(Node::from(inner))
                }
            }

            impl From<$inner> for Node {
                fn from(inner: $inner) -> Self {
                    Node::$name(inner.into())
                }
            }
        )*
    };
}

macro_rules! rewrite_impl {
    ($(dom::$id:ident,)*) => {
        $(
            impl Rewrite for dom::$id {
                type Builder = builders::$id;

                fn rewrite() -> Self::Builder {
                    Self::Builder::new()
                }
            }
        )*
    };
}

macro_rules! rewrite_value_node_from {
    ($($v:ident => $id:ident,)*) => {
        $(
            impl From<OldOrNew<dom::$id>> for ValueNode {
                fn from(v: OldOrNew<dom::$id>) -> Self {
                    Self::$v(v)
                }
            }

            impl From<$id> for ValueNode {
                fn from(v: $id) -> Self {
                    ValueNode::$v(OldOrNew::New(v))
                }
            }
        )*

    };
}
