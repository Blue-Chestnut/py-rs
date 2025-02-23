ItemEnum {
 
    attrs: 
    [
        Attribute {
 
            pound_token: Pound, 
            style: AttrStyle::Outer, 
            bracket_token: Bracket, 
            meta: Meta::List {
 
                path: Path {
 
                    leading_colon: None, 
                    segments: [
                        PathSegment {
 
                            ident: Ident {
 
                                ident: "serde", 
                                span: "" 
                                }
, 
                            arguments: PathArguments::None 
                        }

                    ] 
                }
, 
            delimiter: MacroDelimiter::Paren(Paren), 
            tokens: TokenStream [
                Ident {
 
                    ident: "tag", 
                    span: "" 
                }
, 
                Punct {
 
                    ch: '=', 
                    spacing: Alone, 
                    span: "" 
                }
, 
                Literal {
 
                    kind: Str, 
                    symbol: "kind", 
                    suffix: None, 
                    span: "" 
                }
, 
                Punct {
 
                    ch: ',', 
                    spacing: Alone, 
                    span: "" 
                }
, 
                Ident {
 
                    ident: "content", 
                    span: "" 
                }
, 
                Punct {
 
                    ch: '=', 
                    spacing: Alone, 
                    span: "" 
                }
, 
                Literal {
 
                    kind: Str, 
                    symbol: "data", 
                    suffix: None, 
                    span: ""
                }

            ] 
        }

    }
, 
    Attribute {
 
        pound_token: Pound, 
        style: AttrStyle::Outer, 
        bracket_token: Bracket, 
        meta: Meta::List {
 
            path: Path {
 
                leading_colon: None, 
                segments: [PathSegment {
 
                    ident: Ident {
 ident: "py", span: "" }
, arguments: PathArguments::None }
] }
, delimiter: MacroDelimiter::Paren(Paren), tokens: TokenStream [Ident {
 ident: "export", span: "" }
] }
 }
], vis: Visibility::Inherited, enum_token: Enum, ident: Ident {
 ident: "ComplexEnum", span: "" }
, generics: Generics {
 lt_token: None, params: [], gt_token: None, where_clause: None }
, brace_token: Brace, variants: [Variant {
 attrs: [], ident: Ident {
 ident: "A", span: "" }
, fields: Fields::Unit, discriminant: None }
, Comma, Variant {
 attrs: [], ident: Ident {
 ident: "B", span: "" }
, fields: Fields::Named {
 brace_token: Brace, named: [Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: Some(Ident {
 ident: "foo", span: "" }
), colon_token: Some(Colon), ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "String", span: "" }
, arguments: PathArguments::None }
] }
 }
 }
, Comma, Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: Some(Ident {
 ident: "bar", span: "" }
), colon_token: Some(Colon), ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "f64", span: "" }
, arguments: PathArguments::None }
] }
 }
 }
] }
, discriminant: None }
, Comma, Variant {
 attrs: [], ident: Ident {
 ident: "W", span: "" }
, fields: Fields::Unnamed {
 paren_token: Paren, unnamed: [Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: None, colon_token: None, ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "SimpleEnum", span: "" }
, arguments: PathArguments::None }
] }
 }
 }
] }
, discriminant: None }
, Comma, Variant {
 attrs: [], ident: Ident {
 ident: "F", span: "" }
, fields: Fields::Named {
 brace_token: Brace, named: [Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: Some(Ident {
 ident: "nested", span: "" }
), colon_token: Some(Colon), ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "SimpleEnum", span: "" }
, arguments: PathArguments::None }
] }
 }
 }
] }
, discriminant: None }
, Comma, Variant {
 attrs: [], ident: Ident {
 ident: "V", span: "" }
, fields: Fields::Unnamed {
 paren_token: Paren, unnamed: [Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: None, colon_token: None, ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "Vec", span: "" }
, arguments: PathArguments::AngleBracketed {
 colon2_token: None, lt_token: Lt, args: [GenericArgument::Type(Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "Series", span: "" }
, arguments: PathArguments::None }
] }
 }
)], gt_token: Gt }
 }
] }
 }
 }
] }
, discriminant: None }
, Comma, Variant {
 attrs: [], ident: Ident {
 ident: "U", span: "" }
, fields: Fields::Unnamed {
 paren_token: Paren, unnamed: [Field {
 attrs: [], vis: Visibility::Inherited, mutability: FieldMutability::None, ident: None, colon_token: None, ty: Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "Box", span: "" }
, arguments: PathArguments::AngleBracketed {
 colon2_token: None, lt_token: Lt, args: [GenericArgument::Type(Type::Path {
 qself: None, path: Path {
 leading_colon: None, segments: [PathSegment {
 ident: Ident {
 ident: "User", span: "" }
, arguments: PathArguments::None }
] }
 }
)], gt_token: Gt }
 }
] }
 }
 }
] }
, discriminant: None }
, Comma] }

