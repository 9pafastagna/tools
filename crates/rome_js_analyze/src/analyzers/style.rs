//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_comma_operator;
mod no_implicit_boolean;
mod no_inferrable_types;
mod no_namespace;
mod no_negation_else;
mod no_non_null_assertion;
mod no_parameter_properties;
mod no_unused_template_literal;
mod use_block_statements;
mod use_default_parameter_last;
mod use_enum_initializers;
mod use_exponentiation_operator;
mod use_numeric_literals;
mod use_self_closing_elements;
mod use_shorthand_array_type;
mod use_single_case_statement;
mod use_single_var_declarator;
mod use_template;
mod use_while;
declare_group! { pub (crate) Style { name : "style" , rules : [self :: no_comma_operator :: NoCommaOperator , self :: no_implicit_boolean :: NoImplicitBoolean , self :: no_inferrable_types :: NoInferrableTypes , self :: no_namespace :: NoNamespace , self :: no_negation_else :: NoNegationElse , self :: no_non_null_assertion :: NoNonNullAssertion , self :: no_parameter_properties :: NoParameterProperties , self :: no_unused_template_literal :: NoUnusedTemplateLiteral , self :: use_block_statements :: UseBlockStatements , self :: use_default_parameter_last :: UseDefaultParameterLast , self :: use_enum_initializers :: UseEnumInitializers , self :: use_exponentiation_operator :: UseExponentiationOperator , self :: use_numeric_literals :: UseNumericLiterals , self :: use_self_closing_elements :: UseSelfClosingElements , self :: use_shorthand_array_type :: UseShorthandArrayType , self :: use_single_case_statement :: UseSingleCaseStatement , self :: use_single_var_declarator :: UseSingleVarDeclarator , self :: use_template :: UseTemplate , self :: use_while :: UseWhile ,] } }
