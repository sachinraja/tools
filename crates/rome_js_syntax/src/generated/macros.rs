//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](SyntaxNode::kind)"]
#[doc = r" of the provided [SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" The macro accepts an optional fallback branch wich defaults to"]
#[doc = r" `unreachable!()` as the only SyntaxKind variants not covered by"]
#[doc = r" this macro are token kinds that should not be used to construct"]
#[doc = r" a SyntaxNode."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r#" map_syntax_node!(syntax_node, node => Ok(node.format()), _ => Err("invalid node kind"))"#]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
rome_js_syntax :: map_syntax_node ! ($node , $pattern => $body , _ => unreachable ! ())
    };
    ($ node : expr , $ pattern : pat => $ body : expr , $ fallback : pat => $ default : expr) => {
        match $node {
            node => match rome_js_syntax::SyntaxNode::kind(&node) {
                rome_js_syntax::JsSyntaxKind::IMPORT_META => {
                    let $pattern = unsafe { rome_js_syntax::ImportMeta::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => {
                    let $pattern = unsafe { rome_js_syntax::JsArrayAssignmentPattern::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsArrayAssignmentPatternRestElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => {
                    let $pattern = unsafe { rome_js_syntax::JsArrayBindingPattern::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsArrayBindingPatternRestElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsArrayExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_HOLE => {
                    let $pattern = unsafe { rome_js_syntax::JsArrayHole::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsArrowFunctionExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsAssignmentExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ASSIGNMENT_WITH_DEFAULT => {
                    let $pattern = unsafe { rome_js_syntax::JsAssignmentWithDefault::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsAwaitExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BIG_INT_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsBigIntLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BINARY_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsBinaryExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BINDING_PATTERN_WITH_DEFAULT => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsBindingPatternWithDefault::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BLOCK_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsBlockStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_BREAK_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsBreakStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENTS => {
                    let $pattern = unsafe { rome_js_syntax::JsCallArguments::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CALL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsCallExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CASE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsCaseClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CATCH_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsCatchClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CATCH_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::JsCatchDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CLASS_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::JsClassDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsClassExportDefaultDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CLASS_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsClassExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsComputedMemberAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsComputedMemberExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsComputedMemberName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsConditionalExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsConstructorClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => {
                    let $pattern = unsafe { rome_js_syntax::JsConstructorParameters::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsContinueStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DEBUGGER_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsDebuggerStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsDefaultClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsDefaultImportSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DIRECTIVE => {
                    let $pattern = unsafe { rome_js_syntax::JsDirective::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsDoWhileStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ELSE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsElseClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsEmptyClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EMPTY_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsEmptyStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT => {
                    let $pattern = unsafe { rome_js_syntax::JsExport::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsExportAsClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportDefaultDeclarationClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportDefaultExpressionClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsExportFromClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsExportNamedClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsExportNamedFromClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportNamedFromSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportNamedShorthandSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsExportNamedSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPRESSION_SNIPPED => {
                    let $pattern = unsafe { rome_js_syntax::JsExpressionSnipped::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsExpressionStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsExtendsClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FINALLY_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsFinallyClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FOR_IN_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsForInStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsForOfStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FOR_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsForStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::JsForVariableDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FORMAL_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::JsFormalParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FUNCTION_BODY => {
                    let $pattern = unsafe { rome_js_syntax::JsFunctionBody::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::JsFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsFunctionExportDefaultDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsFunctionExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_GETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsGetterClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsGetterObjectMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsIdentifierAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { rome_js_syntax::JsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsIdentifierExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IF_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsIfStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT => {
                    let $pattern = unsafe { rome_js_syntax::JsImport::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_ASSERTION => {
                    let $pattern = unsafe { rome_js_syntax::JsImportAssertion::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
                    let $pattern = unsafe { rome_js_syntax::JsImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsImportBareClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsImportCallExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsImportDefaultClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsImportNamedClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsImportNamespaceClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IN_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsInExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::JsInitializerClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsInstanceofExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_LABELED_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsLabeledStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsLiteralExportName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsLiteralMemberName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsLogicalExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsMethodClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsMethodObjectMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_MODULE => {
                    let $pattern = unsafe { rome_js_syntax::JsModule::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_MODULE_SOURCE => {
                    let $pattern = unsafe { rome_js_syntax::JsModuleSource::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
                    let $pattern = unsafe { rome_js_syntax::JsNamedImportSpecifiers::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsNamespaceImportSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NEW_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsNewExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectAssignmentPattern::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectAssignmentPatternProperty::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectAssignmentPatternRest::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        rome_js_syntax::JsObjectAssignmentPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                    let $pattern = unsafe { rome_js_syntax::JsObjectBindingPattern::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectBindingPatternProperty::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectBindingPatternRest::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        rome_js_syntax::JsObjectBindingPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsObjectExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PARAMETERS => {
                    let $pattern = unsafe { rome_js_syntax::JsParameters::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsParenthesizedAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsPostUpdateExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsPrivateClassMemberName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PRIVATE_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsPrivateName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsPropertyClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsRegexLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_REST_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::JsRestParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_RETURN_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsReturnStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SCRIPT => {
                    let $pattern = unsafe { rome_js_syntax::JsScript::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsSequenceExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsSetterClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsSetterObjectMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsShorthandNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsShorthandPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SPREAD => {
                    let $pattern = unsafe { rome_js_syntax::JsSpread::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                    let $pattern = unsafe {
                        rome_js_syntax::JsStaticInitializationBlockClassMember::new_unchecked(node)
                    };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsStaticMemberAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsStaticMemberExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STATIC_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsStaticModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SUPER_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsSuperExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SWITCH_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsSwitchStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TEMPLATE => {
                    let $pattern = unsafe { rome_js_syntax::JsTemplate::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsTemplateElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_THIS_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsThisExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_THROW_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsThrowStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TRY_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsTryStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNARY_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsUnaryExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::JsVariableDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsVariableDeclarationClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                    let $pattern = unsafe { rome_js_syntax::JsVariableDeclarator::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_VARIABLE_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsVariableStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_WHILE_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsWhileStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_WITH_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsWithStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_YIELD_ARGUMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsYieldArgument::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_YIELD_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsYieldExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_ATTRIBUTE => {
                    let $pattern = unsafe { rome_js_syntax::JsxAttribute::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsxAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_CLOSING_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxClosingElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_CLOSING_FRAGMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxClosingFragment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsxExpressionAttributeValue::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                    let $pattern = unsafe { rome_js_syntax::JsxExpressionChild::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_FRAGMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxFragment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_MEMBER_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsxMemberName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsxName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_NAMESPACE_NAME => {
                    let $pattern = unsafe { rome_js_syntax::JsxNamespaceName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_OPENING_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxOpeningElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_OPENING_FRAGMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxOpeningFragment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                    let $pattern = unsafe { rome_js_syntax::JsxReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsxSelfClosingElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_SPREAD_ATTRIBUTE => {
                    let $pattern = unsafe { rome_js_syntax::JsxSpreadAttribute::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_SPREAD_CHILD => {
                    let $pattern = unsafe { rome_js_syntax::JsxSpreadChild::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_STRING => {
                    let $pattern = unsafe { rome_js_syntax::JsxString::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_TAG_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsxTagExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_TEXT => {
                    let $pattern = unsafe { rome_js_syntax::JsxText::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::NEW_TARGET => {
                    let $pattern = unsafe { rome_js_syntax::NewTarget::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ABSTRACT_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsAbstractModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsAccessibilityModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ANY_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsAnyType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ARRAY_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsArrayType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_AS_ASSIGNMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsAsAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_AS_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::TsAsExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ASSERTS_CONDITION => {
                    let $pattern = unsafe { rome_js_syntax::TsAssertsCondition::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ASSERTS_RETURN_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsAssertsReturnType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_BIG_INT_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsBigIntLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_BIGINT_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsBigintType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsBooleanLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_BOOLEAN_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsBooleanType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsCallSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsConditionalType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsConstructSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsConstructorSignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsConstructorType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsDeclareFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DECLARE_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsDeclareModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DECLARE_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsDeclareStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsDefaultTypeClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsDefinitePropertyAnnotation::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsDefiniteVariableAnnotation::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                    let $pattern = unsafe {
                        rome_js_syntax::TsEmptyExternalModuleDeclarationBody::new_unchecked(node)
                    };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ENUM_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::TsEnumDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ENUM_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::TsEnumMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsExportAsNamespaceClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsExportAssignmentClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsExportDeclareClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsExtendsClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsExternalModuleDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsExternalModuleReference::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_FUNCTION_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsFunctionType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsGetterSignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsGetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::TsGlobalDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { rome_js_syntax::TsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsImplementsClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsImportEqualsDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsImportType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsImportTypeQualifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsIndexSignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsIndexSignatureParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsIndexSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsIndexedAccessType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INFER_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsInferType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INTERFACE_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::TsInterfaceDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INTERSECTION_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsIntersectionType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsMappedType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsMappedTypeAsClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsMappedTypeOptionalModifierClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsMappedTypeReadonlyModifierClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsMethodSignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsMethodSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MODULE_BLOCK => {
                    let $pattern = unsafe { rome_js_syntax::TsModuleBlock::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_MODULE_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::TsModuleDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NAME_WITH_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { rome_js_syntax::TsNameWithTypeArguments::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsNamedTupleTypeElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NEVER_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsNeverType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsNonNullAssertionAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsNonNullAssertionExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsNonPrimitiveType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NULL_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsNullLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsNumberLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_NUMBER_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsNumberType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_OBJECT_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsObjectType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsOptionalPropertyAnnotation::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsOptionalTupleTypeElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_OVERRIDE_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsOverrideModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PARENTHESIZED_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsParenthesizedType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PREDICATE_RETURN_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsPredicateReturnType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::TsPropertyParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsPropertySignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsPropertySignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => {
                    let $pattern = unsafe { rome_js_syntax::TsQualifiedModuleName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_QUALIFIED_NAME => {
                    let $pattern = unsafe { rome_js_syntax::TsQualifiedName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_READONLY_MODIFIER => {
                    let $pattern = unsafe { rome_js_syntax::TsReadonlyModifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_REFERENCE_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsReferenceType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsRestTupleTypeElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => {
                    let $pattern = unsafe { rome_js_syntax::TsReturnTypeAnnotation::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsSetterSignatureClassMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsSetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_STRING_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsStringLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_STRING_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsStringType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_SYMBOL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsSymbolType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { rome_js_syntax::TsTemplateElement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsTemplateLiteralType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_THIS_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::TsThisParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_THIS_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsThisType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TUPLE_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsTupleType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeAliasDeclaration::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ANNOTATION => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeAnnotation::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeArguments::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsTypeAssertionAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsTypeAssertionExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeConstraintClause::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeOperatorType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeParameterName::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETERS => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeParameters::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPEOF_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeofType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_UNDEFINED_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsUndefinedType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_UNION_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsUnionType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_UNKNOWN_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsUnknownType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_VOID_TYPE => {
                    let $pattern = unsafe { rome_js_syntax::TsVoidType::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknown::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_ASSIGNMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownAssignment::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_BINDING => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownBinding::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_EXPRESSION => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownExpression::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsUnknownImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_MEMBER => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownMember::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsUnknownNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_PARAMETER => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownParameter::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_UNKNOWN_STATEMENT => {
                    let $pattern = unsafe { rome_js_syntax::JsUnknownStatement::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsArrayAssignmentPatternElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsArrayBindingPatternElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsArrayElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsCallArgumentList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CLASS_MEMBER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsClassMemberList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsConstructorModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsConstructorParameterList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_DIRECTIVE_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsDirectiveList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportNamedFromSpecifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsExportNamedSpecifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsImportAssertionEntryList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_METHOD_MODIFIER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsMethodModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_MODULE_ITEM_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsModuleItemList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsNamedImportSpecifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => {
                    let $pattern = unsafe {
                        rome_js_syntax::JsObjectAssignmentPatternPropertyList::new_unchecked(node)
                    };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::JsObjectBindingPatternPropertyList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_OBJECT_MEMBER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsObjectMemberList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PARAMETER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsParameterList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_PROPERTY_MODIFIER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsPropertyModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_STATEMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsStatementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_SWITCH_CASE_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsSwitchCaseList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsTemplateElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsVariableDeclaratorList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsxAttributeList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::JSX_CHILD_LIST => {
                    let $pattern = unsafe { rome_js_syntax::JsxChildList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_ENUM_MEMBER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsEnumMemberList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsIndexSignatureModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsIntersectionTypeElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_METHOD_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsMethodSignatureModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PROPERTY_PARAMETER_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsPropertyParameterModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_PROPERTY_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { rome_js_syntax::TsPropertySignatureModifierList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTemplateElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTupleTypeElementList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_ARGUMENT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeArgumentList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_MEMBER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeMemberList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETER_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsTypeParameterList::new_unchecked(node) };
                    $body
                }
                rome_js_syntax::JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                    let $pattern = unsafe { rome_js_syntax::TsUnionTypeVariantList::new_unchecked(node) };
                    $body
                }
                $fallback => $default,
            },
        }
    };
}
