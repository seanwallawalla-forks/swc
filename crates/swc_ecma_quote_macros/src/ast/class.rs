use swc_ecma_ast::*;

impl_struct!(
    Class,
    [
        span,
        decorators,
        body,
        super_class,
        is_abstract,
        type_params,
        super_type_params,
        implements
    ]
);

impl_struct!(
    Constructor,
    [span, key, params, body, accessibility, is_optional]
);

impl_struct!(
    ClassMethod,
    [
        span,
        key,
        function,
        kind,
        is_static,
        accessibility,
        is_abstract,
        is_optional,
        is_override
    ]
);

impl_struct!(
    PrivateMethod,
    [
        span,
        key,
        function,
        kind,
        is_static,
        accessibility,
        is_abstract,
        is_optional,
        is_override
    ]
);

impl_struct!(
    ClassProp,
    [
        span,
        key,
        value,
        type_ann,
        is_static,
        decorators,
        accessibility,
        is_abstract,
        is_optional,
        is_override,
        readonly,
        declare,
        definite
    ]
);

impl_struct!(StaticBlock, [span, body]);
