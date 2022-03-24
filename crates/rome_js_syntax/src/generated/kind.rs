//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum JsSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file.May have trivia attached"]
    EOF,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    QUESTION,
    QUESTION2,
    QUESTIONDOT,
    AMP,
    PIPE,
    PLUS,
    PLUS2,
    STAR,
    STAR2,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    DOT3,
    COLON,
    EQ,
    EQ2,
    EQ3,
    FAT_ARROW,
    BANG,
    NEQ,
    NEQ2,
    MINUS,
    MINUS2,
    LTEQ,
    GTEQ,
    PLUSEQ,
    MINUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AMP2,
    PIPE2,
    SHL,
    SHR,
    USHR,
    SHLEQ,
    SHREQ,
    USHREQ,
    AMP2EQ,
    PIPE2EQ,
    STAR2EQ,
    QUESTION2EQ,
    AT,
    BACKTICK,
    BREAK_KW,
    CASE_KW,
    CATCH_KW,
    CLASS_KW,
    CONST_KW,
    CONTINUE_KW,
    DEBUGGER_KW,
    DEFAULT_KW,
    DELETE_KW,
    DO_KW,
    ELSE_KW,
    ENUM_KW,
    EXPORT_KW,
    EXTENDS_KW,
    FALSE_KW,
    FINALLY_KW,
    FOR_KW,
    FUNCTION_KW,
    IF_KW,
    IN_KW,
    INSTANCEOF_KW,
    IMPORT_KW,
    NEW_KW,
    NULL_KW,
    RETURN_KW,
    SUPER_KW,
    SWITCH_KW,
    THIS_KW,
    THROW_KW,
    TRY_KW,
    TRUE_KW,
    TYPEOF_KW,
    VAR_KW,
    VOID_KW,
    WHILE_KW,
    WITH_KW,
    IMPLEMENTS_KW,
    INTERFACE_KW,
    LET_KW,
    PACKAGE_KW,
    PRIVATE_KW,
    PROTECTED_KW,
    PUBLIC_KW,
    STATIC_KW,
    YIELD_KW,
    ABSTRACT_KW,
    AS_KW,
    ASSERTS_KW,
    ASSERT_KW,
    ANY_KW,
    ASYNC_KW,
    AWAIT_KW,
    BOOLEAN_KW,
    CONSTRUCTOR_KW,
    DECLARE_KW,
    GET_KW,
    INFER_KW,
    IS_KW,
    KEYOF_KW,
    MODULE_KW,
    NAMESPACE_KW,
    NEVER_KW,
    READONLY_KW,
    REQUIRE_KW,
    NUMBER_KW,
    OBJECT_KW,
    SET_KW,
    STRING_KW,
    SYMBOL_KW,
    TYPE_KW,
    UNDEFINED_KW,
    UNIQUE_KW,
    UNKNOWN_KW,
    FROM_KW,
    GLOBAL_KW,
    BIGINT_KW,
    OVERRIDE_KW,
    OF_KW,
    JS_NUMBER_LITERAL,
    JS_BIG_INT_LITERAL,
    JS_STRING_LITERAL,
    JS_REGEX_LITERAL,
    JSX_TEXT_LITERAL,
    JSX_STRING_LITERAL,
    TARGET,
    META,
    HASH,
    TEMPLATE_CHUNK,
    DOLLAR_CURLY,
    ERROR_TOKEN,
    IDENT,
    JSX_IDENT,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    JS_SHEBANG,
    JS_MODULE,
    JS_MODULE_ITEM_LIST,
    JS_SCRIPT,
    JS_EXPRESSION_SNIPPED,
    JS_DIRECTIVE,
    JS_DIRECTIVE_LIST,
    JS_STATEMENT_LIST,
    JS_BLOCK_STATEMENT,
    JS_FUNCTION_BODY,
    JS_VARIABLE_STATEMENT,
    JS_VARIABLE_DECLARATION,
    JS_VARIABLE_DECLARATOR_LIST,
    JS_VARIABLE_DECLARATOR,
    JS_VARIABLE_DECLARATION_CLAUSE,
    TS_DEFINITE_VARIABLE_ANNOTATION,
    JS_INITIALIZER_CLAUSE,
    JS_EMPTY_STATEMENT,
    JS_EXPRESSION_STATEMENT,
    JS_IF_STATEMENT,
    JS_ELSE_CLAUSE,
    JS_DO_WHILE_STATEMENT,
    JS_WHILE_STATEMENT,
    JS_FOR_STATEMENT,
    JS_FOR_IN_STATEMENT,
    JS_FOR_OF_STATEMENT,
    JS_FOR_VARIABLE_DECLARATION,
    JS_CONTINUE_STATEMENT,
    JS_BREAK_STATEMENT,
    JS_RETURN_STATEMENT,
    JS_WITH_STATEMENT,
    JS_SWITCH_STATEMENT,
    JS_SWITCH_CASE_LIST,
    JS_CASE_CLAUSE,
    JS_DEFAULT_CLAUSE,
    JS_LABELED_STATEMENT,
    JS_THROW_STATEMENT,
    JS_TRY_STATEMENT,
    JS_TRY_FINALLY_STATEMENT,
    JS_CATCH_CLAUSE,
    JS_CATCH_DECLARATION,
    JS_FINALLY_CLAUSE,
    JS_DEBUGGER_STATEMENT,
    JS_FUNCTION_DECLARATION,
    JS_PARAMETERS,
    JS_PARAMETER_LIST,
    JS_FORMAL_PARAMETER,
    JS_REST_PARAMETER,
    TS_THIS_PARAMETER,
    TS_PROPERTY_PARAMETER,
    TS_PROPERTY_PARAMETER_MODIFIER_LIST,
    TS_TYPE_ANNOTATION,
    TS_RETURN_TYPE_ANNOTATION,
    JS_IDENTIFIER_BINDING,
    JS_IDENTIFIER_EXPRESSION,
    JS_REFERENCE_IDENTIFIER,
    JS_NAME,
    JS_PRIVATE_NAME,
    JS_THIS_EXPRESSION,
    JS_ARRAY_EXPRESSION,
    JS_ARRAY_ELEMENT_LIST,
    JS_ARRAY_HOLE,
    JS_COMPUTED_MEMBER_NAME,
    JS_LITERAL_MEMBER_NAME,
    JS_OBJECT_EXPRESSION,
    JS_OBJECT_MEMBER_LIST,
    JS_PROPERTY_OBJECT_MEMBER,
    JS_GETTER_OBJECT_MEMBER,
    JS_SETTER_OBJECT_MEMBER,
    JS_METHOD_OBJECT_MEMBER,
    JS_SUPER_EXPRESSION,
    JS_PARENTHESIZED_EXPRESSION,
    JS_NEW_EXPRESSION,
    JS_FUNCTION_EXPRESSION,
    JS_STATIC_MEMBER_EXPRESSION,
    JS_COMPUTED_MEMBER_EXPRESSION,
    JS_CALL_EXPRESSION,
    JS_UNARY_EXPRESSION,
    JS_PRE_UPDATE_EXPRESSION,
    JS_POST_UPDATE_EXPRESSION,
    JS_BINARY_EXPRESSION,
    JS_INSTANCEOF_EXPRESSION,
    JS_IN_EXPRESSION,
    JS_LOGICAL_EXPRESSION,
    JS_CONDITIONAL_EXPRESSION,
    JS_ASSIGNMENT_EXPRESSION,
    JS_SEQUENCE_EXPRESSION,
    JS_CALL_ARGUMENTS,
    JS_CALL_ARGUMENT_LIST,
    JS_STRING_LITERAL_EXPRESSION,
    JS_NUMBER_LITERAL_EXPRESSION,
    JS_BIG_INT_LITERAL_EXPRESSION,
    JS_BOOLEAN_LITERAL_EXPRESSION,
    JS_NULL_LITERAL_EXPRESSION,
    JS_REGEX_LITERAL_EXPRESSION,
    JS_TEMPLATE,
    JS_TEMPLATE_ELEMENT,
    JS_TEMPLATE_CHUNK_ELEMENT,
    JS_TEMPLATE_ELEMENT_LIST,
    JS_IMPORT_CALL_EXPRESSION,
    NEW_TARGET,
    IMPORT_META,
    JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
    JS_SPREAD,
    JS_OBJECT_BINDING_PATTERN,
    JS_ARRAY_BINDING_PATTERN,
    JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST,
    JS_BINDING_PATTERN_WITH_DEFAULT,
    JS_ARRAY_BINDING_PATTERN_REST_ELEMENT,
    JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST,
    JS_OBJECT_BINDING_PATTERN_REST,
    JS_OBJECT_BINDING_PATTERN_PROPERTY,
    JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY,
    JS_ARROW_FUNCTION_EXPRESSION,
    JS_YIELD_EXPRESSION,
    JS_YIELD_ARGUMENT,
    JS_CLASS_DECLARATION,
    JS_CLASS_EXPRESSION,
    JS_CLASS_MEMBER_LIST,
    JS_STATIC_MODIFIER,
    TS_DECLARE_MODIFIER,
    TS_READONLY_MODIFIER,
    TS_ABSTRACT_MODIFIER,
    TS_OVERRIDE_MODIFIER,
    TS_ACCESSIBILITY_MODIFIER,
    JS_EXTENDS_CLAUSE,
    TS_IMPLEMENTS_CLAUSE,
    JS_PRIVATE_CLASS_MEMBER_NAME,
    JS_CONSTRUCTOR_CLASS_MEMBER,
    TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
    JS_CONSTRUCTOR_MODIFIER_LIST,
    JS_CONSTRUCTOR_PARAMETER_LIST,
    JS_CONSTRUCTOR_PARAMETERS,
    JS_CONSTRUCTOR_PARAMETER,
    JS_PROPERTY_CLASS_MEMBER,
    JS_PROPERTY_MODIFIER_LIST,
    TS_OPTIONAL_PROPERTY_ANNOTATION,
    TS_DEFINITE_PROPERTY_ANNOTATION,
    JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER,
    JS_METHOD_CLASS_MEMBER,
    JS_METHOD_MODIFIER_LIST,
    JS_GETTER_CLASS_MEMBER,
    JS_SETTER_CLASS_MEMBER,
    JS_EMPTY_CLASS_MEMBER,
    JS_ASSIGNMENT_WITH_DEFAULT,
    JS_PARENTHESIZED_ASSIGNMENT,
    JS_IDENTIFIER_ASSIGNMENT,
    JS_STATIC_MEMBER_ASSIGNMENT,
    JS_COMPUTED_MEMBER_ASSIGNMENT,
    TS_NON_NULL_ASSERTION_ASSIGNMENT,
    TS_AS_ASSIGNMENT,
    TS_TYPE_ASSERTION_ASSIGNMENT,
    JS_ARRAY_ASSIGNMENT_PATTERN,
    JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST,
    JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT,
    JS_OBJECT_ASSIGNMENT_PATTERN,
    JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST,
    JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY,
    JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY,
    JS_OBJECT_ASSIGNMENT_PATTERN_REST,
    JS_IMPORT,
    JS_IMPORT_BARE_CLAUSE,
    JS_IMPORT_DEFAULT_CLAUSE,
    JS_IMPORT_NAMESPACE_CLAUSE,
    JS_IMPORT_NAMED_CLAUSE,
    JS_NAMED_IMPORT_SPECIFIERS,
    JS_NAMED_IMPORT_SPECIFIER_LIST,
    JS_NAMESPACE_IMPORT_SPECIFIER,
    JS_DEFAULT_IMPORT_SPECIFIER,
    JS_NAMED_IMPORT_SPECIFIER,
    JS_SHORTHAND_NAMED_IMPORT_SPECIFIER,
    JS_IMPORT_ASSERTION,
    JS_IMPORT_ASSERTION_ENTRY_LIST,
    JS_IMPORT_ASSERTION_ENTRY,
    JS_MODULE_SOURCE,
    JS_EXPORT,
    JS_EXPORT_NAMED_CLAUSE,
    JS_EXPORT_NAMED_SPECIFIER_LIST,
    JS_EXPORT_NAMED_SHORTHAND_SPECIFIER,
    JS_EXPORT_NAMED_SPECIFIER,
    JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE,
    JS_EXPORT_DEFAULT_DECLARATION_CLAUSE,
    JS_CLASS_EXPORT_DEFAULT_DECLARATION,
    JS_FUNCTION_EXPORT_DEFAULT_DECLARATION,
    JS_EXPORT_FROM_CLAUSE,
    JS_EXPORT_NAMED_FROM_CLAUSE,
    JS_EXPORT_NAMED_FROM_SPECIFIER_LIST,
    JS_EXPORT_NAMED_FROM_SPECIFIER,
    JS_EXPORT_AS_CLAUSE,
    TS_EXPORT_AS_NAMESPACE_CLAUSE,
    TS_EXPORT_ASSIGNMENT_CLAUSE,
    TS_EXPORT_DECLARE_CLAUSE,
    JS_LITERAL_EXPORT_NAME,
    JS_AWAIT_EXPRESSION,
    TS_IDENTIFIER_BINDING,
    TS_ANY_TYPE,
    TS_UNKNOWN_TYPE,
    TS_NUMBER_TYPE,
    TS_NON_PRIMITIVE_TYPE,
    TS_BOOLEAN_TYPE,
    TS_BIGINT_TYPE,
    TS_STRING_TYPE,
    TS_SYMBOL_TYPE,
    TS_VOID_TYPE,
    TS_UNDEFINED_TYPE,
    TS_NEVER_TYPE,
    TS_THIS_TYPE,
    TS_TYPEOF_TYPE,
    TS_PARENTHESIZED_TYPE,
    TS_MAPPED_TYPE,
    TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE,
    TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE,
    TS_MAPPED_TYPE_AS_CLAUSE,
    TS_TYPE_ALIAS_DECLARATION,
    TS_MODULE_DECLARATION,
    TS_GLOBAL_DECLARATION,
    TS_QUALIFIED_MODULE_NAME,
    TS_MODULE_BLOCK,
    TS_EXTERNAL_MODULE_DECLARATION,
    TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY,
    TS_QUALIFIED_NAME,
    TS_REFERENCE_TYPE,
    TS_UNION_TYPE,
    TS_UNION_TYPE_VARIANT_LIST,
    TS_INTERSECTION_TYPE,
    TS_INTERSECTION_TYPE_ELEMENT_LIST,
    TS_OBJECT_TYPE,
    TS_TYPE_MEMBER_LIST,
    TS_INTERFACE_DECLARATION,
    TS_EXTENDS_CLAUSE,
    TS_PROPERTY_SIGNATURE_TYPE_MEMBER,
    TS_METHOD_SIGNATURE_TYPE_MEMBER,
    TS_CALL_SIGNATURE_TYPE_MEMBER,
    TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER,
    TS_GETTER_SIGNATURE_TYPE_MEMBER,
    TS_SETTER_SIGNATURE_TYPE_MEMBER,
    TS_INDEX_SIGNATURE_TYPE_MEMBER,
    TS_IMPORT_TYPE,
    TS_IMPORT_TYPE_QUALIFIER,
    TS_ARRAY_TYPE,
    TS_INDEXED_ACCESS_TYPE,
    TS_TUPLE_TYPE,
    TS_TUPLE_TYPE_ELEMENT_LIST,
    TS_REST_TUPLE_TYPE_ELEMENT,
    TS_OPTIONAL_TUPLE_TYPE_ELEMENT,
    TS_NAMED_TUPLE_TYPE_ELEMENT,
    TS_TYPE_OPERATOR_TYPE,
    TS_INFER_TYPE,
    TS_CONSTRUCTOR_TYPE,
    TS_FUNCTION_TYPE,
    TS_PREDICATE_RETURN_TYPE,
    TS_ASSERTS_RETURN_TYPE,
    TS_ASSERTS_CONDITION,
    TS_TYPE_PARAMETERS,
    TS_TYPE_PARAMETER_LIST,
    TS_TYPE_PARAMETER,
    TS_TYPE_PARAMETER_NAME,
    TS_TYPE_CONSTRAINT_CLAUSE,
    TS_DEFAULT_TYPE_CLAUSE,
    TS_STRING_LITERAL_TYPE,
    TS_NUMBER_LITERAL_TYPE,
    TS_BIG_INT_LITERAL_TYPE,
    TS_BOOLEAN_LITERAL_TYPE,
    TS_NULL_LITERAL_TYPE,
    TS_TEMPLATE_LITERAL_TYPE,
    TS_TEMPLATE_ELEMENT_LIST,
    TS_TEMPLATE_CHUNK_ELEMENT,
    TS_TEMPLATE_ELEMENT,
    TS_TYPE_ARGUMENTS,
    TS_TYPE_ARGUMENT_LIST,
    TS_TYPE_LIST,
    TS_EXTENDS,
    TS_CONDITIONAL_TYPE,
    TS_NON_NULL_ASSERTION_EXPRESSION,
    TS_TYPE_ASSERTION_EXPRESSION,
    TS_AS_EXPRESSION,
    TS_ENUM_DECLARATION,
    TS_ENUM_MEMBER_LIST,
    TS_ENUM_MEMBER,
    TS_IMPORT_EQUALS_DECLARATION,
    TS_EXTERNAL_MODULE_REFERENCE,
    TS_NAME_WITH_TYPE_ARGUMENTS,
    TS_DECLARE_FUNCTION_DECLARATION,
    TS_DECLARE_STATEMENT,
    TS_INDEX_SIGNATURE_PARAMETER,
    TS_PROPERTY_SIGNATURE_CLASS_MEMBER,
    TS_PROPERTY_SIGNATURE_MODIFIER_LIST,
    TS_METHOD_SIGNATURE_CLASS_MEMBER,
    TS_METHOD_SIGNATURE_MODIFIER_LIST,
    TS_GETTER_SIGNATURE_CLASS_MEMBER,
    TS_SETTER_SIGNATURE_CLASS_MEMBER,
    TS_INDEX_SIGNATURE_CLASS_MEMBER,
    TS_INDEX_SIGNATURE_MODIFIER_LIST,
    JSX_NAME,
    JSX_NAMESPACE_NAME,
    JSX_REFERENCE_IDENTIFIER,
    JSX_TAG_EXPRESSION,
    JSX_ELEMENT,
    JSX_FRAGMENT,
    JSX_SELF_CLOSING_ELEMENT,
    JSX_OPENING_ELEMENT,
    JSX_CLOSING_ELEMENT,
    JSX_MEMBER_NAME,
    JSX_TEXT,
    JSX_ATTRIBUTE_LIST,
    JSX_ATTRIBUTE,
    JSX_SPREAD_ATTRIBUTE,
    JSX_ATTRIBUTE_INITIALIZER_CLAUSE,
    JSX_EXPRESSION_ATTRIBUTE_VALUE,
    JSX_CHILD_LIST,
    JSX_EXPRESSION_CHILD,
    JSX_SPREAD_CHILD,
    JSX_STRING,
    JS_UNKNOWN,
    JS_UNKNOWN_EXPRESSION,
    JS_UNKNOWN_STATEMENT,
    JS_UNKNOWN_MEMBER,
    JS_UNKNOWN_BINDING,
    JS_UNKNOWN_PARAMETER,
    JS_UNKNOWN_IMPORT_ASSERTION_ENTRY,
    JS_UNKNOWN_NAMED_IMPORT_SPECIFIER,
    JS_UNKNOWN_ASSIGNMENT,
    #[doc(hidden)]
    __LAST,
}
use self::JsSyntaxKind::*;
impl JsSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | QUESTION | QUESTION2 | QUESTIONDOT | AMP | PIPE
            | PLUS | PLUS2 | STAR | STAR2 | SLASH | CARET | PERCENT | DOT | DOT3 | COLON | EQ
            | EQ2 | EQ3 | FAT_ARROW | BANG | NEQ | NEQ2 | MINUS | MINUS2 | LTEQ | GTEQ | PLUSEQ
            | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
            | SHL | SHR | USHR | SHLEQ | SHREQ | USHREQ | AMP2EQ | PIPE2EQ | STAR2EQ
            | QUESTION2EQ | AT | BACKTICK => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            JS_NUMBER_LITERAL | JS_BIG_INT_LITERAL | JS_STRING_LITERAL | JS_REGEX_LITERAL
            | JSX_TEXT_LITERAL | JSX_STRING_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            JS_MODULE_ITEM_LIST
            | JS_DIRECTIVE_LIST
            | JS_STATEMENT_LIST
            | JS_VARIABLE_DECLARATOR_LIST
            | JS_SWITCH_CASE_LIST
            | JS_PARAMETER_LIST
            | TS_PROPERTY_PARAMETER_MODIFIER_LIST
            | JS_ARRAY_ELEMENT_LIST
            | JS_OBJECT_MEMBER_LIST
            | JS_CALL_ARGUMENT_LIST
            | JS_TEMPLATE_ELEMENT_LIST
            | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
            | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
            | JS_CLASS_MEMBER_LIST
            | JS_CONSTRUCTOR_MODIFIER_LIST
            | JS_CONSTRUCTOR_PARAMETER_LIST
            | JS_PROPERTY_MODIFIER_LIST
            | JS_METHOD_MODIFIER_LIST
            | JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST
            | JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST
            | JS_NAMED_IMPORT_SPECIFIER_LIST
            | JS_IMPORT_ASSERTION_ENTRY_LIST
            | JS_EXPORT_NAMED_SPECIFIER_LIST
            | JS_EXPORT_NAMED_FROM_SPECIFIER_LIST
            | TS_UNION_TYPE_VARIANT_LIST
            | TS_INTERSECTION_TYPE_ELEMENT_LIST
            | TS_TYPE_MEMBER_LIST
            | TS_TUPLE_TYPE_ELEMENT_LIST
            | TS_TYPE_PARAMETER_LIST
            | TS_TEMPLATE_ELEMENT_LIST
            | TS_TYPE_ARGUMENT_LIST
            | TS_TYPE_LIST
            | TS_ENUM_MEMBER_LIST
            | TS_PROPERTY_SIGNATURE_MODIFIER_LIST
            | TS_METHOD_SIGNATURE_MODIFIER_LIST
            | TS_INDEX_SIGNATURE_MODIFIER_LIST
            | JSX_ATTRIBUTE_LIST
            | JSX_CHILD_LIST => true,
            _ => false,
        }
    }
    pub const fn is_before_expr(self) -> bool {
        match self {
            BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON | QUESTION | PLUS2
            | MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW | ELSE_KW | RETURN_KW | THROW_KW
            | NEW_KW | EXTENDS_KW | YIELD_KW | IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ
            | INSTANCEOF_KW | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ
            | AMP2 | PIPE2 | SHLEQ | SHREQ | USHREQ | EQ | EQ2 | EQ3 | NEQ | NEQ2 | FAT_ARROW
            | MINUS | PLUS | AWAIT_KW => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<JsSyntaxKind> {
        let kw = match ident {
            "break" => BREAK_KW,
            "case" => CASE_KW,
            "catch" => CATCH_KW,
            "class" => CLASS_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "debugger" => DEBUGGER_KW,
            "default" => DEFAULT_KW,
            "delete" => DELETE_KW,
            "do" => DO_KW,
            "else" => ELSE_KW,
            "enum" => ENUM_KW,
            "export" => EXPORT_KW,
            "extends" => EXTENDS_KW,
            "false" => FALSE_KW,
            "finally" => FINALLY_KW,
            "for" => FOR_KW,
            "function" => FUNCTION_KW,
            "if" => IF_KW,
            "in" => IN_KW,
            "instanceof" => INSTANCEOF_KW,
            "import" => IMPORT_KW,
            "new" => NEW_KW,
            "null" => NULL_KW,
            "return" => RETURN_KW,
            "super" => SUPER_KW,
            "switch" => SWITCH_KW,
            "this" => THIS_KW,
            "throw" => THROW_KW,
            "try" => TRY_KW,
            "true" => TRUE_KW,
            "typeof" => TYPEOF_KW,
            "var" => VAR_KW,
            "void" => VOID_KW,
            "while" => WHILE_KW,
            "with" => WITH_KW,
            "implements" => IMPLEMENTS_KW,
            "interface" => INTERFACE_KW,
            "let" => LET_KW,
            "package" => PACKAGE_KW,
            "private" => PRIVATE_KW,
            "protected" => PROTECTED_KW,
            "public" => PUBLIC_KW,
            "static" => STATIC_KW,
            "yield" => YIELD_KW,
            "abstract" => ABSTRACT_KW,
            "as" => AS_KW,
            "asserts" => ASSERTS_KW,
            "assert" => ASSERT_KW,
            "any" => ANY_KW,
            "async" => ASYNC_KW,
            "await" => AWAIT_KW,
            "boolean" => BOOLEAN_KW,
            "constructor" => CONSTRUCTOR_KW,
            "declare" => DECLARE_KW,
            "get" => GET_KW,
            "infer" => INFER_KW,
            "is" => IS_KW,
            "keyof" => KEYOF_KW,
            "module" => MODULE_KW,
            "namespace" => NAMESPACE_KW,
            "never" => NEVER_KW,
            "readonly" => READONLY_KW,
            "require" => REQUIRE_KW,
            "number" => NUMBER_KW,
            "object" => OBJECT_KW,
            "set" => SET_KW,
            "string" => STRING_KW,
            "symbol" => SYMBOL_KW,
            "type" => TYPE_KW,
            "undefined" => UNDEFINED_KW,
            "unique" => UNIQUE_KW,
            "unknown" => UNKNOWN_KW,
            "from" => FROM_KW,
            "global" => GLOBAL_KW,
            "bigint" => BIGINT_KW,
            "override" => OVERRIDE_KW,
            "of" => OF_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "'('",
            R_PAREN => "')'",
            L_CURLY => "'{'",
            R_CURLY => "'}'",
            L_BRACK => "'['",
            R_BRACK => "']'",
            L_ANGLE => "<",
            R_ANGLE => ">",
            TILDE => "~",
            QUESTION => "?",
            QUESTION2 => "??",
            QUESTIONDOT => "?.",
            AMP => "&",
            PIPE => "|",
            PLUS => "+",
            PLUS2 => "++",
            STAR => "*",
            STAR2 => "**",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            DOT => ".",
            DOT3 => "...",
            COLON => ":",
            EQ => "=",
            EQ2 => "==",
            EQ3 => "===",
            FAT_ARROW => "=>",
            BANG => "!",
            NEQ => "!=",
            NEQ2 => "!==",
            MINUS => "-",
            MINUS2 => "--",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            MINUSEQ => "-=",
            PIPEEQ => "|=",
            AMPEQ => "&=",
            CARETEQ => "^=",
            SLASHEQ => "/=",
            STAREQ => "*=",
            PERCENTEQ => "%=",
            AMP2 => "&&",
            PIPE2 => "||",
            SHL => "<<",
            SHR => ">>",
            USHR => ">>>",
            SHLEQ => "<<=",
            SHREQ => ">>=",
            USHREQ => ">>>=",
            AMP2EQ => "&&=",
            PIPE2EQ => "||=",
            STAR2EQ => "**=",
            QUESTION2EQ => "??=",
            AT => "@",
            BACKTICK => "'`'",
            BREAK_KW => "break",
            CASE_KW => "case",
            CATCH_KW => "catch",
            CLASS_KW => "class",
            CONST_KW => "const",
            CONTINUE_KW => "continue",
            DEBUGGER_KW => "debugger",
            DEFAULT_KW => "default",
            DELETE_KW => "delete",
            DO_KW => "do",
            ELSE_KW => "else",
            ENUM_KW => "enum",
            EXPORT_KW => "export",
            EXTENDS_KW => "extends",
            FALSE_KW => "false",
            FINALLY_KW => "finally",
            FOR_KW => "for",
            FUNCTION_KW => "function",
            IF_KW => "if",
            IN_KW => "in",
            INSTANCEOF_KW => "instanceof",
            IMPORT_KW => "import",
            NEW_KW => "new",
            NULL_KW => "null",
            RETURN_KW => "return",
            SUPER_KW => "super",
            SWITCH_KW => "switch",
            THIS_KW => "this",
            THROW_KW => "throw",
            TRY_KW => "try",
            TRUE_KW => "true",
            TYPEOF_KW => "typeof",
            VAR_KW => "var",
            VOID_KW => "void",
            WHILE_KW => "while",
            WITH_KW => "with",
            IMPLEMENTS_KW => "implements",
            INTERFACE_KW => "interface",
            LET_KW => "let",
            PACKAGE_KW => "package",
            PRIVATE_KW => "private",
            PROTECTED_KW => "protected",
            PUBLIC_KW => "public",
            STATIC_KW => "static",
            YIELD_KW => "yield",
            ABSTRACT_KW => "abstract",
            AS_KW => "as",
            ASSERTS_KW => "asserts",
            ASSERT_KW => "assert",
            ANY_KW => "any",
            ASYNC_KW => "async",
            AWAIT_KW => "await",
            BOOLEAN_KW => "boolean",
            CONSTRUCTOR_KW => "constructor",
            DECLARE_KW => "declare",
            GET_KW => "get",
            INFER_KW => "infer",
            IS_KW => "is",
            KEYOF_KW => "keyof",
            MODULE_KW => "module",
            NAMESPACE_KW => "namespace",
            NEVER_KW => "never",
            READONLY_KW => "readonly",
            REQUIRE_KW => "require",
            NUMBER_KW => "number",
            OBJECT_KW => "object",
            SET_KW => "set",
            STRING_KW => "string",
            SYMBOL_KW => "symbol",
            TYPE_KW => "type",
            UNDEFINED_KW => "undefined",
            UNIQUE_KW => "unique",
            UNKNOWN_KW => "unknown",
            FROM_KW => "from",
            GLOBAL_KW => "global",
            BIGINT_KW => "bigint",
            OVERRIDE_KW => "override",
            OF_KW => "of",
            JS_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: JsSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: JsSyntaxKind :: COMMA } ; ['('] => { $ crate :: JsSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: JsSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: JsSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: JsSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: JsSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: JsSyntaxKind :: R_BRACK } ; [<] => { $ crate :: JsSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: JsSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: JsSyntaxKind :: TILDE } ; [?] => { $ crate :: JsSyntaxKind :: QUESTION } ; [??] => { $ crate :: JsSyntaxKind :: QUESTION2 } ; [?.] => { $ crate :: JsSyntaxKind :: QUESTIONDOT } ; [&] => { $ crate :: JsSyntaxKind :: AMP } ; [|] => { $ crate :: JsSyntaxKind :: PIPE } ; [+] => { $ crate :: JsSyntaxKind :: PLUS } ; [++] => { $ crate :: JsSyntaxKind :: PLUS2 } ; [*] => { $ crate :: JsSyntaxKind :: STAR } ; [**] => { $ crate :: JsSyntaxKind :: STAR2 } ; [/] => { $ crate :: JsSyntaxKind :: SLASH } ; [^] => { $ crate :: JsSyntaxKind :: CARET } ; [%] => { $ crate :: JsSyntaxKind :: PERCENT } ; [.] => { $ crate :: JsSyntaxKind :: DOT } ; [...] => { $ crate :: JsSyntaxKind :: DOT3 } ; [:] => { $ crate :: JsSyntaxKind :: COLON } ; [=] => { $ crate :: JsSyntaxKind :: EQ } ; [==] => { $ crate :: JsSyntaxKind :: EQ2 } ; [===] => { $ crate :: JsSyntaxKind :: EQ3 } ; [=>] => { $ crate :: JsSyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: JsSyntaxKind :: BANG } ; [!=] => { $ crate :: JsSyntaxKind :: NEQ } ; [!==] => { $ crate :: JsSyntaxKind :: NEQ2 } ; [-] => { $ crate :: JsSyntaxKind :: MINUS } ; [--] => { $ crate :: JsSyntaxKind :: MINUS2 } ; [<=] => { $ crate :: JsSyntaxKind :: LTEQ } ; [>=] => { $ crate :: JsSyntaxKind :: GTEQ } ; [+=] => { $ crate :: JsSyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: JsSyntaxKind :: MINUSEQ } ; [|=] => { $ crate :: JsSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: JsSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: JsSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: JsSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: JsSyntaxKind :: STAREQ } ; [%=] => { $ crate :: JsSyntaxKind :: PERCENTEQ } ; [&&] => { $ crate :: JsSyntaxKind :: AMP2 } ; [||] => { $ crate :: JsSyntaxKind :: PIPE2 } ; [<<] => { $ crate :: JsSyntaxKind :: SHL } ; [>>] => { $ crate :: JsSyntaxKind :: SHR } ; [>>>] => { $ crate :: JsSyntaxKind :: USHR } ; [<<=] => { $ crate :: JsSyntaxKind :: SHLEQ } ; [>>=] => { $ crate :: JsSyntaxKind :: SHREQ } ; [>>>=] => { $ crate :: JsSyntaxKind :: USHREQ } ; [&&=] => { $ crate :: JsSyntaxKind :: AMP2EQ } ; [||=] => { $ crate :: JsSyntaxKind :: PIPE2EQ } ; [**=] => { $ crate :: JsSyntaxKind :: STAR2EQ } ; [??=] => { $ crate :: JsSyntaxKind :: QUESTION2EQ } ; [@] => { $ crate :: JsSyntaxKind :: AT } ; ['`'] => { $ crate :: JsSyntaxKind :: BACKTICK } ; [break] => { $ crate :: JsSyntaxKind :: BREAK_KW } ; [case] => { $ crate :: JsSyntaxKind :: CASE_KW } ; [catch] => { $ crate :: JsSyntaxKind :: CATCH_KW } ; [class] => { $ crate :: JsSyntaxKind :: CLASS_KW } ; [const] => { $ crate :: JsSyntaxKind :: CONST_KW } ; [continue] => { $ crate :: JsSyntaxKind :: CONTINUE_KW } ; [debugger] => { $ crate :: JsSyntaxKind :: DEBUGGER_KW } ; [default] => { $ crate :: JsSyntaxKind :: DEFAULT_KW } ; [delete] => { $ crate :: JsSyntaxKind :: DELETE_KW } ; [do] => { $ crate :: JsSyntaxKind :: DO_KW } ; [else] => { $ crate :: JsSyntaxKind :: ELSE_KW } ; [enum] => { $ crate :: JsSyntaxKind :: ENUM_KW } ; [export] => { $ crate :: JsSyntaxKind :: EXPORT_KW } ; [extends] => { $ crate :: JsSyntaxKind :: EXTENDS_KW } ; [false] => { $ crate :: JsSyntaxKind :: FALSE_KW } ; [finally] => { $ crate :: JsSyntaxKind :: FINALLY_KW } ; [for] => { $ crate :: JsSyntaxKind :: FOR_KW } ; [function] => { $ crate :: JsSyntaxKind :: FUNCTION_KW } ; [if] => { $ crate :: JsSyntaxKind :: IF_KW } ; [in] => { $ crate :: JsSyntaxKind :: IN_KW } ; [instanceof] => { $ crate :: JsSyntaxKind :: INSTANCEOF_KW } ; [import] => { $ crate :: JsSyntaxKind :: IMPORT_KW } ; [new] => { $ crate :: JsSyntaxKind :: NEW_KW } ; [null] => { $ crate :: JsSyntaxKind :: NULL_KW } ; [return] => { $ crate :: JsSyntaxKind :: RETURN_KW } ; [super] => { $ crate :: JsSyntaxKind :: SUPER_KW } ; [switch] => { $ crate :: JsSyntaxKind :: SWITCH_KW } ; [this] => { $ crate :: JsSyntaxKind :: THIS_KW } ; [throw] => { $ crate :: JsSyntaxKind :: THROW_KW } ; [try] => { $ crate :: JsSyntaxKind :: TRY_KW } ; [true] => { $ crate :: JsSyntaxKind :: TRUE_KW } ; [typeof] => { $ crate :: JsSyntaxKind :: TYPEOF_KW } ; [var] => { $ crate :: JsSyntaxKind :: VAR_KW } ; [void] => { $ crate :: JsSyntaxKind :: VOID_KW } ; [while] => { $ crate :: JsSyntaxKind :: WHILE_KW } ; [with] => { $ crate :: JsSyntaxKind :: WITH_KW } ; [implements] => { $ crate :: JsSyntaxKind :: IMPLEMENTS_KW } ; [interface] => { $ crate :: JsSyntaxKind :: INTERFACE_KW } ; [let] => { $ crate :: JsSyntaxKind :: LET_KW } ; [package] => { $ crate :: JsSyntaxKind :: PACKAGE_KW } ; [private] => { $ crate :: JsSyntaxKind :: PRIVATE_KW } ; [protected] => { $ crate :: JsSyntaxKind :: PROTECTED_KW } ; [public] => { $ crate :: JsSyntaxKind :: PUBLIC_KW } ; [static] => { $ crate :: JsSyntaxKind :: STATIC_KW } ; [yield] => { $ crate :: JsSyntaxKind :: YIELD_KW } ; [abstract] => { $ crate :: JsSyntaxKind :: ABSTRACT_KW } ; [as] => { $ crate :: JsSyntaxKind :: AS_KW } ; [asserts] => { $ crate :: JsSyntaxKind :: ASSERTS_KW } ; [assert] => { $ crate :: JsSyntaxKind :: ASSERT_KW } ; [any] => { $ crate :: JsSyntaxKind :: ANY_KW } ; [async] => { $ crate :: JsSyntaxKind :: ASYNC_KW } ; [await] => { $ crate :: JsSyntaxKind :: AWAIT_KW } ; [boolean] => { $ crate :: JsSyntaxKind :: BOOLEAN_KW } ; [constructor] => { $ crate :: JsSyntaxKind :: CONSTRUCTOR_KW } ; [declare] => { $ crate :: JsSyntaxKind :: DECLARE_KW } ; [get] => { $ crate :: JsSyntaxKind :: GET_KW } ; [infer] => { $ crate :: JsSyntaxKind :: INFER_KW } ; [is] => { $ crate :: JsSyntaxKind :: IS_KW } ; [keyof] => { $ crate :: JsSyntaxKind :: KEYOF_KW } ; [module] => { $ crate :: JsSyntaxKind :: MODULE_KW } ; [namespace] => { $ crate :: JsSyntaxKind :: NAMESPACE_KW } ; [never] => { $ crate :: JsSyntaxKind :: NEVER_KW } ; [readonly] => { $ crate :: JsSyntaxKind :: READONLY_KW } ; [require] => { $ crate :: JsSyntaxKind :: REQUIRE_KW } ; [number] => { $ crate :: JsSyntaxKind :: NUMBER_KW } ; [object] => { $ crate :: JsSyntaxKind :: OBJECT_KW } ; [set] => { $ crate :: JsSyntaxKind :: SET_KW } ; [string] => { $ crate :: JsSyntaxKind :: STRING_KW } ; [symbol] => { $ crate :: JsSyntaxKind :: SYMBOL_KW } ; [type] => { $ crate :: JsSyntaxKind :: TYPE_KW } ; [undefined] => { $ crate :: JsSyntaxKind :: UNDEFINED_KW } ; [unique] => { $ crate :: JsSyntaxKind :: UNIQUE_KW } ; [unknown] => { $ crate :: JsSyntaxKind :: UNKNOWN_KW } ; [from] => { $ crate :: JsSyntaxKind :: FROM_KW } ; [global] => { $ crate :: JsSyntaxKind :: GLOBAL_KW } ; [bigint] => { $ crate :: JsSyntaxKind :: BIGINT_KW } ; [override] => { $ crate :: JsSyntaxKind :: OVERRIDE_KW } ; [of] => { $ crate :: JsSyntaxKind :: OF_KW } ; [ident] => { $ crate :: JsSyntaxKind :: IDENT } ; [EOF] => { $ crate :: JsSyntaxKind :: EOF } ; [#] => { $ crate :: JsSyntaxKind :: HASH } ; }
