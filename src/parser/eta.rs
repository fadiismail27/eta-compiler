// auto-generated: "lalrpop 0.23.0"
// sha3: da82348b71a0ae3635f533b0d1310b76c5e6cc8c7f3fa10a201c4809d0c39aa0
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Interface {

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(i64),
        Variant3(Option<Token>),
        Variant4((Token, Token)),
        Variant5(alloc::vec::Vec<(Token, Token)>),
        Variant6(Expr),
        Variant7(alloc::vec::Vec<Expr>),
        Variant8(AssignTarget),
        Variant9(alloc::vec::Vec<AssignTarget>),
        Variant10(Param),
        Variant11(alloc::vec::Vec<Param>),
        Variant12(Type),
        Variant13(alloc::vec::Vec<Type>),
        Variant14((Stmt, Option<Token>)),
        Variant15(alloc::vec::Vec<(Stmt, Option<Token>)>),
        Variant16(Option<Expr>),
        Variant17(alloc::vec::Vec<Option<Expr>>),
        Variant18(Vec<Option<Expr>>),
        Variant19(Stmt),
        Variant20(Block),
        Variant21(Vec<Expr>),
        Variant22(Vec<AssignTarget>),
        Variant23(Vec<Type>),
        Variant24(Vec<Param>),
        Variant25(DeclSuffix),
        Variant26(FuncDef),
        Variant27(GlobalDecl),
        Variant28(IdentStmtRest),
        Variant29(Interface),
        Variant30(InterfaceDecl),
        Variant31(alloc::vec::Vec<InterfaceDecl>),
        Variant32(Option<Param>),
        Variant33(Program),
        Variant34(Option<Vec<Expr>>),
        Variant35(Option<Vec<Type>>),
        Variant36(TopLevelItem),
        Variant37(alloc::vec::Vec<TopLevelItem>),
        Variant38(alloc::vec::Vec<String>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 24, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 24, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 24, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -124, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -167, -167, 0, 0, 0, 0, -167, -167, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -167, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -137, 28, 0, 0, 0, 0, 0, -137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, 0, 0, 0, 0, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, -59, 0, 0, 0, 0, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -119, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -152, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, -66, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, -67, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -168, -168, 0, 0, 0, 0, -168, -168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -168, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -118,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -122,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        -182,
        // State 9
        -123,
        // State 10
        0,
        // State 11
        -124,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -121,
        // State 19
        -120,
        // State 20
        -167,
        // State 21
        0,
        // State 22
        -60,
        // State 23
        -59,
        // State 24
        -119,
        // State 25
        -152,
        // State 26
        -66,
        // State 27
        0,
        // State 28
        -67,
        // State 29
        0,
        // State 30
        -168,
        // State 31
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            14 => 3,
            17 => 7,
            27 => 20,
            31 => 25,
            33 => 12,
            47 => 8,
            48 => match state {
                1 => 11,
                _ => 9,
            },
            49 => 1,
            53 => match state {
                3 => 15,
                _ => 13,
            },
            59 => 18,
            66 => match state {
                6 => 26,
                7 => 28,
                _ => 21,
            },
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###""use""###,
        r###""if""###,
        r###""else""###,
        r###""while""###,
        r###""return""###,
        r###""length""###,
        r###""int""###,
        r###""bool""###,
        r###""true""###,
        r###""false""###,
        r###""(""###,
        r###"")""###,
        r###""[""###,
        r###""]""###,
        r###""{""###,
        r###""}""###,
        r###"":""###,
        r###"";""###,
        r###"",""###,
        r###""=""###,
        r###""_""###,
        r###""+""###,
        r###""-""###,
        r###""*""###,
        r###""*>>""###,
        r###""/""###,
        r###""%""###,
        r###""!""###,
        r###""<""###,
        r###""<=""###,
        r###"">""###,
        r###"">=""###,
        r###""==""###,
        r###""!=""###,
        r###""&""###,
        r###""|""###,
        r###"IDENT"###,
        r###"INT_LIT"###,
        r###"CHAR_LIT"###,
        r###"STRING_LIT"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i16],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = String;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Interface;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 40 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token::Use if true => Some(0),
            Token::If if true => Some(1),
            Token::Else if true => Some(2),
            Token::While if true => Some(3),
            Token::Return if true => Some(4),
            Token::Length if true => Some(5),
            Token::IntType if true => Some(6),
            Token::BoolType if true => Some(7),
            Token::True if true => Some(8),
            Token::False if true => Some(9),
            Token::LParen if true => Some(10),
            Token::RParen if true => Some(11),
            Token::LBracket if true => Some(12),
            Token::RBracket if true => Some(13),
            Token::LBrace if true => Some(14),
            Token::RBrace if true => Some(15),
            Token::Colon if true => Some(16),
            Token::Semicolon if true => Some(17),
            Token::Comma if true => Some(18),
            Token::Assign if true => Some(19),
            Token::Underscore if true => Some(20),
            Token::Plus if true => Some(21),
            Token::Minus if true => Some(22),
            Token::Mul if true => Some(23),
            Token::HighMul if true => Some(24),
            Token::Div if true => Some(25),
            Token::Mod if true => Some(26),
            Token::Not if true => Some(27),
            Token::Lt if true => Some(28),
            Token::Le if true => Some(29),
            Token::Gt if true => Some(30),
            Token::Ge if true => Some(31),
            Token::Eq if true => Some(32),
            Token::Ne if true => Some(33),
            Token::And if true => Some(34),
            Token::Or if true => Some(35),
            Token::Identifier(_) if true => Some(36),
            Token::IntLiteral(_) if true => Some(37),
            Token::CharLiteral(_) if true => Some(38),
            Token::StringLiteral(_) if true => Some(39),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 => __Symbol::Variant0(__token),
            36 | 39 => match __token {
                Token::Identifier(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            37 | 38 => match __token {
                Token::IntLiteral(__tok0) | Token::CharLiteral(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 24,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 28,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 31,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 33,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 34,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 34,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 34,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 35,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 37,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 37,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 39,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 39,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 41,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 41,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 42,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 43,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 44,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 44,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 45,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 45,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 45,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 46,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 46,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 46,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 46,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 46,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 48,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 48,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 48,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 48,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 49,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 51,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 52,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 52,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 53,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 54,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            143 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 56,
                }
            }
            144 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            145 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            146 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 56,
                }
            }
            147 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 57,
                }
            }
            148 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 57,
                }
            }
            149 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            150 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 58,
                }
            }
            151 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 59,
                }
            }
            152 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            153 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 60,
                }
            }
            154 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            155 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            156 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            157 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 62,
                }
            }
            158 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 62,
                }
            }
            159 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            160 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            161 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            162 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 64,
                }
            }
            163 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            164 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            165 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 65,
                }
            }
            166 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            167 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 66,
                }
            }
            168 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            169 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 67,
                }
            }
            170 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            171 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 68,
                }
            }
            172 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 68,
                }
            }
            173 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 69,
                }
            }
            174 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            175 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            176 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 70,
                }
            }
            177 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 71,
                }
            }
            178 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            179 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            180 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 71,
                }
            }
            181 => __state_machine::SimulatedReduce::Accept,
            182 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 73,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct InterfaceParser {
        _priv: (),
    }

    impl Default for InterfaceParser { fn default() -> Self { Self::new() } }
    impl InterfaceParser {
        pub fn new() -> InterfaceParser {
            InterfaceParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Interface, __lalrpop_util::ParseError<usize, Token, String>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
    >(
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Interface,__lalrpop_util::ParseError<usize, Token, String>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                __reduce94(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            95 => {
                __reduce95(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            96 => {
                __reduce96(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            97 => {
                __reduce97(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            98 => {
                __reduce98(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            99 => {
                __reduce99(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            100 => {
                __reduce100(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            101 => {
                __reduce101(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            102 => {
                __reduce102(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            103 => {
                __reduce103(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            104 => {
                __reduce104(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            105 => {
                __reduce105(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            106 => {
                __reduce106(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            107 => {
                __reduce107(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            108 => {
                __reduce108(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            109 => {
                __reduce109(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            110 => {
                __reduce110(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            111 => {
                __reduce111(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            112 => {
                __reduce112(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            113 => {
                __reduce113(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            114 => {
                __reduce114(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            115 => {
                __reduce115(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            116 => {
                __reduce116(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            117 => {
                __reduce117(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            118 => {
                __reduce118(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            119 => {
                __reduce119(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            120 => {
                __reduce120(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            121 => {
                __reduce121(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            122 => {
                __reduce122(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            123 => {
                __reduce123(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            124 => {
                __reduce124(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            125 => {
                __reduce125(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            126 => {
                __reduce126(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            127 => {
                __reduce127(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            128 => {
                __reduce128(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            129 => {
                __reduce129(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            130 => {
                __reduce130(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            131 => {
                __reduce131(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            132 => {
                __reduce132(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            133 => {
                __reduce133(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            134 => {
                __reduce134(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            135 => {
                __reduce135(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            136 => {
                __reduce136(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            137 => {
                __reduce137(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            138 => {
                __reduce138(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            139 => {
                __reduce139(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            140 => {
                __reduce140(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            141 => {
                __reduce141(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            142 => {
                __reduce142(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            143 => {
                __reduce143(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            144 => {
                __reduce144(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            145 => {
                __reduce145(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            146 => {
                __reduce146(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            147 => {
                __reduce147(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            148 => {
                __reduce148(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            149 => {
                __reduce149(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            150 => {
                __reduce150(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            151 => {
                __reduce151(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            152 => {
                __reduce152(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            153 => {
                __reduce153(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            154 => {
                __reduce154(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            155 => {
                __reduce155(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            156 => {
                __reduce156(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            157 => {
                __reduce157(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            158 => {
                __reduce158(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            159 => {
                __reduce159(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            160 => {
                __reduce160(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            161 => {
                __reduce161(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            162 => {
                __reduce162(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            163 => {
                __reduce163(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            164 => {
                __reduce164(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            165 => {
                __reduce165(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            166 => {
                __reduce166(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            167 => {
                __reduce167(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            168 => {
                __reduce168(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            169 => {
                __reduce169(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            170 => {
                __reduce170(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            171 => {
                __reduce171(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            172 => {
                __reduce172(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            173 => {
                __reduce173(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            174 => {
                __reduce174(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            175 => {
                __reduce175(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            176 => {
                __reduce176(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            177 => {
                __reduce177(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            178 => {
                __reduce178(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            179 => {
                __reduce179(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            180 => {
                __reduce180(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            181 => {
                // __Interface = Interface => ActionFn(1);
                let __sym0 = __pop_Variant29(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            182 => {
                __reduce182(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {__action}")
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Stmt, Option<Token>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Token, Token), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AssignTarget, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Block, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DeclSuffix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, FuncDef, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, GlobalDecl, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, IdentStmtRest, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Interface, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, InterfaceDecl, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant30(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant32<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant32(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant34<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Vec<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant34(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant35<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Vec<Type>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant35(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Param, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant33<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant33(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant36<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TopLevelItem, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant36(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AssignTarget>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Option<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(Token, Token)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<AssignTarget>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<InterfaceDecl>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant31(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Option<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant38<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant38(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant37<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<TopLevelItem>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant37(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? = ";" => ActionFn(114);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action114::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? =  => ActionFn(115);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action115::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]") = "[", "]" => ActionFn(96);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action96::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")* =  => ActionFn(94);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action94::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")* = ("[" "]")+ => ActionFn(95);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")+ = "[", "]" => ActionFn(166);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action166::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")+ = ("[" "]")+, "[", "]" => ActionFn(167);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action167::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]") = "[", Expr, "]" => ActionFn(101);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action101::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 4)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]")+ = "[", Expr, "]" => ActionFn(172);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action172::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 5)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]")+ = ("[" <Expr> "]")+, "[", Expr, "]" => ActionFn(173);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action173::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 5)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action141::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* =  => ActionFn(139);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action139::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(140);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(174);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action174::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 8)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(175);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action175::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 8)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",") = LhsItem, "," => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")* =  => ActionFn(132);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action132::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")* = (<LhsItem> ",")+ => ActionFn(133);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action133::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 10)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")+ = LhsItem, "," => ActionFn(180);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action180::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 11)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")+ = (<LhsItem> ",")+, LhsItem, "," => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 11)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action126::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(127);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(184);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action184::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 14)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(185);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action185::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 14)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",") = Type, "," => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 15)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")* =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action129::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")* = (<Type> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")+ = Type, "," => ActionFn(188);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action188::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 17)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")+ = (<Type> ",")+, Type, "," => ActionFn(189);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action189::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 17)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?) = Stmt, ";" => ActionFn(154);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action154::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 18)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?) = Stmt => ActionFn(155);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action155::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)* =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action105::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)* = (Stmt ";"?)+ => ActionFn(106);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = Stmt, ";" => ActionFn(192);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action192::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = Stmt => ActionFn(193);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action193::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = (Stmt ";"?)+, Stmt, ";" => ActionFn(194);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action194::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 20)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = (Stmt ";"?)+, Stmt => ActionFn(195);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action195::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim = "[", "]" => ActionFn(38);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 21)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim = "[", Expr, "]" => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 21)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim* =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action97::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 22)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim* = ArrayDim+ => ActionFn(98);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 22)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim+ = ArrayDim => ActionFn(142);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action142::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim+ = ArrayDim+, ArrayDim => ActionFn(143);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action143::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 23)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDimList =  => ActionFn(198);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action198::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 24)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDimList = ArrayDim+ => ActionFn(199);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action199::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = INT_LIT => ActionFn(77);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = CHAR_LIT => ActionFn(78);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = STRING_LIT => ActionFn(79);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "true" => ActionFn(80);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action80::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "false" => ActionFn(81);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = IDENT => ActionFn(82);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action82::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "(", Expr, ")" => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 25)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "{", Comma<Expr>, "}" => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 25)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = Block => ActionFn(23);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 26)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = "_", "=", Expr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 26)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = "_", ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(25);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant21(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 26)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = IDENT, IdentStmtRest => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant28(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 26)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BaseType = "int" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 27)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BaseType = "bool" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 27)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", StmtList, "}" => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (3, 28)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Expr> = Expr => ActionFn(176);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action176::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 29)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Expr> = (<Expr> ",")+, Expr => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action177::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 29)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<LhsItem> = LhsItem => ActionFn(182);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action182::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 30)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<LhsItem> = (<LhsItem> ",")+, LhsItem => ActionFn(183);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action183::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 30)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Type> = Type => ActionFn(190);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action190::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 31)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Type> = (<Type> ",")+, Type => ActionFn(191);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action191::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 31)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = Expr => ActionFn(200);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action200::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 32)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> =  => ActionFn(201);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action201::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 32)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(202);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action202::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 32)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(203);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action203::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 32)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(204);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action204::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 33)
    }
    fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(205);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action205::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 33)
    }
    fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(206);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action206::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 33)
    }
    fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(207);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action207::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 33)
    }
    fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 34)
    }
    fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix = "=", Expr => ActionFn(43);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 34)
    }
    fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix = ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant21(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (4, 34)
    }
    fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "|", Expr6 => ActionFn(71);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 35)
    }
    fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(72);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 35)
    }
    fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = PostfixExpr => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 36)
    }
    fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "-", Expr1 => ActionFn(50);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action50::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 37)
    }
    fn __reduce82<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "!", Expr1 => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 37)
    }
    fn __reduce83<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(52);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 37)
    }
    fn __reduce84<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "*", Expr1 => ActionFn(53);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce85<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "*>>", Expr1 => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce86<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "/", Expr1 => ActionFn(55);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce87<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "%", Expr1 => ActionFn(56);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action56::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce88<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(57);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 38)
    }
    fn __reduce89<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "+", Expr2 => ActionFn(58);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 39)
    }
    fn __reduce90<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "-", Expr2 => ActionFn(59);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 39)
    }
    fn __reduce91<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(60);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 39)
    }
    fn __reduce92<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "<", Expr3 => ActionFn(61);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action61::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "<=", Expr3 => ActionFn(62);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action62::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, ">", Expr3 => ActionFn(63);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, ">=", Expr3 => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(65);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 40)
    }
    fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(66);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 41)
    }
    fn __reduce98<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 41)
    }
    fn __reduce99<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(68);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 41)
    }
    fn __reduce100<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&", Expr5 => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 42)
    }
    fn __reduce101<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(70);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 42)
    }
    fn __reduce102<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr? = Expr => ActionFn(137);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 43)
    }
    fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr? =  => ActionFn(138);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action138::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 43)
    }
    fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncDef = IDENT, "(", Comma<Param>, ")", ReturnTypes, Block => ActionFn(212);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant20(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action212::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (6, 44)
    }
    fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncDef = IDENT, "(", Comma<Param>, ")", Block => ActionFn(213);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant20(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action213::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (5, 44)
    }
    fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, ";" => ActionFn(156);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action156::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (4, 45)
    }
    fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type => ActionFn(157);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action157::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (3, 45)
    }
    fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, "=", Literal, ";" => ActionFn(158);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action158::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (6, 45)
    }
    fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, "=", Literal => ActionFn(159);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action159::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (5, 45)
    }
    fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "(", Comma<Expr>, ")" => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (3, 46)
    }
    fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "=", Expr => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (2, 46)
    }
    fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ("[" <Expr> "]")+, "=", Expr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (3, 46)
    }
    fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ":", BaseType, ValidArrayDims, DeclSuffix => ActionFn(33);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (4, 46)
    }
    fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(34);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant21(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (4, 46)
    }
    fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ("[" <Expr> "]")+, ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(35);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant21(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (5, 46)
    }
    fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "(", Comma<Expr>, ")", ("[" <Expr> "]")+, "=", Expr => ActionFn(36);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action36::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (6, 46)
    }
    fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Interface = InterfaceDecl+ => ActionFn(90);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 47)
    }
    fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ReturnTypes, ";" => ActionFn(214);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action214::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (6, 48)
    }
    fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ";" => ActionFn(215);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action215::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (5, 48)
    }
    fn __reduce120<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ReturnTypes => ActionFn(216);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action216::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (5, 48)
    }
    fn __reduce121<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")" => ActionFn(217);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action217::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (4, 48)
    }
    fn __reduce122<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl+ = InterfaceDecl => ActionFn(92);
        let __sym0 = __pop_Variant30(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (1, 49)
    }
    fn __reduce123<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl+ = InterfaceDecl+, InterfaceDecl => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant30(__symbols);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (2, 49)
    }
    fn __reduce124<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = "_" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 50)
    }
    fn __reduce125<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT => ActionFn(46);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 50)
    }
    fn __reduce126<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT, ":", Type => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 50)
    }
    fn __reduce127<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT, ("[" <Expr> "]")+ => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 50)
    }
    fn __reduce128<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = INT_LIT => ActionFn(85);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce129<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", INT_LIT => ActionFn(86);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 51)
    }
    fn __reduce130<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = CHAR_LIT => ActionFn(87);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action87::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce131<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "true" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce132<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "false" => ActionFn(89);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce133<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = "if", Expr, MatchedIf, "else", MatchedIf => ActionFn(17);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant19(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 52)
    }
    fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = "while", Expr, MatchedIf => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 52)
    }
    fn __reduce135<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = AtomicStmt => ActionFn(19);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 52)
    }
    fn __reduce136<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = IDENT, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 53)
    }
    fn __reduce137<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(124);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action124::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (1, 54)
    }
    fn __reduce138<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action125::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (0, 54)
    }
    fn __reduce139<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(73);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce140<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = IDENT, "(", Comma<Expr>, ")" => ActionFn(74);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant21(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce141<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = "length", "(", Expr, ")" => ActionFn(75);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action75::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce142<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = AtomExpr => ActionFn(76);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 55)
    }
    fn __reduce143<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(220);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action220::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (0, 56)
    }
    fn __reduce144<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = UseDecl+ => ActionFn(221);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action221::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 56)
    }
    fn __reduce145<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = TopLevelItem+ => ActionFn(222);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action222::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 56)
    }
    fn __reduce146<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = UseDecl+, TopLevelItem+ => ActionFn(223);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant37(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action223::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (2, 56)
    }
    fn __reduce147<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt = "return", Comma<Expr>, ";" => ActionFn(162);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action162::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (3, 57)
    }
    fn __reduce148<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt = "return", Comma<Expr> => ActionFn(163);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action163::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 57)
    }
    fn __reduce149<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt? = ReturnStmt => ActionFn(103);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (1, 58)
    }
    fn __reduce150<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt? =  => ActionFn(104);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action104::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (0, 58)
    }
    fn __reduce151<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes = ":", Comma1<Type> => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant23(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 59)
    }
    fn __reduce152<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes? = ReturnTypes => ActionFn(111);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 60)
    }
    fn __reduce153<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes? =  => ActionFn(112);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action112::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (0, 60)
    }
    fn __reduce154<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = MatchedIf => ActionFn(15);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 61)
    }
    fn __reduce155<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = UnmatchedIf => ActionFn(16);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 61)
    }
    fn __reduce156<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = ReturnStmt => ActionFn(208);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action208::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 62)
    }
    fn __reduce157<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList =  => ActionFn(209);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action209::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 62)
    }
    fn __reduce158<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = (Stmt ";"?)+, ReturnStmt => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action210::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 62)
    }
    fn __reduce159<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = (Stmt ";"?)+ => ActionFn(211);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action211::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 62)
    }
    fn __reduce160<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem = FuncDef => ActionFn(4);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 63)
    }
    fn __reduce161<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem = GlobalDecl => ActionFn(5);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 63)
    }
    fn __reduce162<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem* =  => ActionFn(116);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action116::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (0, 64)
    }
    fn __reduce163<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem* = TopLevelItem+ => ActionFn(117);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action117::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 64)
    }
    fn __reduce164<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem+ = TopLevelItem => ActionFn(122);
        let __sym0 = __pop_Variant36(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action122::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 65)
    }
    fn __reduce165<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem+ = TopLevelItem+, TopLevelItem => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant36(__symbols);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (2, 65)
    }
    fn __reduce166<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = BaseType => ActionFn(11);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 66)
    }
    fn __reduce167<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = Type, "[", "]" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 66)
    }
    fn __reduce168<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "if", Expr, Stmt => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 67)
    }
    fn __reduce169<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "if", Expr, MatchedIf, "else", UnmatchedIf => ActionFn(21);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant19(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 67)
    }
    fn __reduce170<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "while", Expr, UnmatchedIf => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 67)
    }
    fn __reduce171<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl = "use", IDENT, ";" => ActionFn(164);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action164::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 68)
    }
    fn __reduce172<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl = "use", IDENT => ActionFn(165);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action165::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 68)
    }
    fn __reduce173<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl* =  => ActionFn(118);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action118::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (0, 69)
    }
    fn __reduce174<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl* = UseDecl+ => ActionFn(119);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action119::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 69)
    }
    fn __reduce175<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl+ = UseDecl => ActionFn(120);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 70)
    }
    fn __reduce176<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl+ = UseDecl+, UseDecl => ActionFn(121);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action121::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (2, 70)
    }
    fn __reduce177<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims =  => ActionFn(168);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action168::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 71)
    }
    fn __reduce178<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" "]")+ => ActionFn(169);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 71)
    }
    fn __reduce179<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" <Expr> "]")+ => ActionFn(170);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 71)
    }
    fn __reduce180<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" <Expr> "]")+, ("[" "]")+ => ActionFn(171);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action171::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 71)
    }
    fn __reduce182<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 73)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Interface::InterfaceParser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding, clippy::clone_on_copy, clippy::unit_arg)]
mod __parse__Program {

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(i64),
        Variant3(Option<Token>),
        Variant4((Token, Token)),
        Variant5(alloc::vec::Vec<(Token, Token)>),
        Variant6(Expr),
        Variant7(alloc::vec::Vec<Expr>),
        Variant8(AssignTarget),
        Variant9(alloc::vec::Vec<AssignTarget>),
        Variant10(Param),
        Variant11(alloc::vec::Vec<Param>),
        Variant12(Type),
        Variant13(alloc::vec::Vec<Type>),
        Variant14((Stmt, Option<Token>)),
        Variant15(alloc::vec::Vec<(Stmt, Option<Token>)>),
        Variant16(Option<Expr>),
        Variant17(alloc::vec::Vec<Option<Expr>>),
        Variant18(Vec<Option<Expr>>),
        Variant19(Stmt),
        Variant20(Block),
        Variant21(Vec<Expr>),
        Variant22(Vec<AssignTarget>),
        Variant23(Vec<Type>),
        Variant24(Vec<Param>),
        Variant25(DeclSuffix),
        Variant26(FuncDef),
        Variant27(GlobalDecl),
        Variant28(IdentStmtRest),
        Variant29(Interface),
        Variant30(InterfaceDecl),
        Variant31(alloc::vec::Vec<InterfaceDecl>),
        Variant32(Option<Param>),
        Variant33(Program),
        Variant34(Option<Vec<Expr>>),
        Variant35(Option<Vec<Type>>),
        Variant36(TopLevelItem),
        Variant37(alloc::vec::Vec<TopLevelItem>),
        Variant38(alloc::vec::Vec<String>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0,
        // State 2
        77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 100, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 101, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 16, 0, 18, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, -158, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 16, 0, 18, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, -160, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 16
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, -69, 0, -69, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 17
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 33, 0, 0, 0, 31, 0, 30, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 21
        0, 16, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 19, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 23
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 24
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 25
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, -69, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 26
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, -71, 0, 0, 26, -71, 0, -71, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 27
        0, 16, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 19, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, -69, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 32
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 58, 0, -126, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 36
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 37
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 38
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 39
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 40
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 41
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 42
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 43
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 44
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 45
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 46
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 47
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 48
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 49
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 50
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 51
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, -69, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 54
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 55
        0, -178, -178, -178, -178, 0, 0, 0, 0, 0, 0, 0, 64, 0, -178, -178, 0, -178, -178, -178, -178, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -178, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 57
        0, 0, 0, 0, 0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 16, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0,
        // State 59
        0, -111, -111, -111, -111, 0, 0, 0, 0, 0, 0, 0, 33, 0, -111, -111, 0, -111, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 61
        0, -180, -180, -180, -180, 0, 0, 0, 0, 0, 0, 0, 67, 0, -180, -180, 0, -180, -180, -180, -180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -180, 0, 0, 0,
        // State 62
        0, -76, -76, -76, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, -76, 0, -76, 68, 69, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 207, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 64
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 65
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 66
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 207, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 69
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 70
        0, 0, 0, 0, 0, 133, 0, 0, 134, 132, 24, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 136, 137, 135, 138,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -161, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -162, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -165, 0, 0, 0,
        // State 75
        -176, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -176, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -166, 0, 0, 0,
        // State 79
        -177, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -177, 0, 0, 0,
        // State 80
        -173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -173, 0, 0, 0,
        // State 81
        -172, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -172, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -167, -167, 0, -167, 0, 0, -167, -167, -167, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -167, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 0, 0, 92, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0,
        // State 87
        0, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, -60, -60, 0, -60, -60, 0, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0,
        // State 88
        0, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, -59, -59, 0, -59, -59, 0, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -137, 93, 0, 0, 0, 0, 0, -137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -168, -168, 0, -168, 0, 0, -168, -168, -168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -168, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, -66, 0, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, -136, -136, -136, -136, 0, 0, 0, 0, 0, 0, 0, 0, 0, -136, -136, 0, -136, 0, 0, -136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -136, 0, 0, 0,
        // State 107
        0, -55, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, -55, 0, -55, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0,
        // State 108
        0, -155, 0, -155, -155, 0, 0, 0, 0, 0, 0, 0, 0, 0, -155, -155, 0, -155, 0, 0, -155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -155, 0, 0, 0,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, -36, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, 0, 121, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, -156, 0, -156, -156, 0, 0, 0, 0, 0, 0, 0, 0, 0, -156, -156, 0, -156, 0, 0, -156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -156, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -130, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, -67, 0, 0, 0, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -159, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, -38, 0, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 144, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0,
        // State 120
        0, -35, 0, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0,
        // State 121
        0, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, -61, 0, -61, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0,
        // State 122
        0, -143, -143, -143, -143, 0, 0, 0, 0, 0, 0, -143, -143, -143, -143, -143, 0, -143, -143, 0, -143, -143, -143, -143, -143, -143, -143, 0, -143, -143, -143, -143, -143, -143, -143, -143, -143, 0, 0, 0,
        // State 123
        0, -84, -84, -84, -84, 0, 0, 0, 0, 0, 0, -84, 0, -84, -84, -84, 0, -84, -84, 0, -84, -84, -84, -84, -84, -84, -84, 0, -84, -84, -84, -84, -84, -84, -84, -84, -84, 0, 0, 0,
        // State 124
        0, -89, -89, -89, -89, 0, 0, 0, 0, 0, 0, -89, 0, -89, -89, -89, 0, -89, -89, 0, -89, -89, -89, -89, -89, -89, -89, 0, -89, -89, -89, -89, -89, -89, -89, -89, -89, 0, 0, 0,
        // State 125
        0, -92, -92, -92, -92, 0, 0, 0, 0, 0, 0, -92, 0, -92, -92, -92, 0, -92, -92, 0, -92, -92, -92, 38, 39, 40, 37, 0, -92, -92, -92, -92, -92, -92, -92, -92, -92, 0, 0, 0,
        // State 126
        0, -97, -97, -97, -97, 0, 0, 0, 0, 0, 0, -97, 0, -97, -97, -97, 0, -97, -97, 0, -97, 41, 42, 0, 0, 0, 0, 0, -97, -97, -97, -97, -97, -97, -97, -97, -97, 0, 0, 0,
        // State 127
        0, -100, -100, -100, -100, 0, 0, 0, 0, 0, 0, -100, 0, -100, -100, -100, 0, -100, -100, 0, -100, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 46, -100, -100, -100, -100, -100, 0, 0, 0,
        // State 128
        0, -102, -102, -102, -102, 0, 0, 0, 0, 0, 0, -102, 0, -102, -102, -102, 0, -102, -102, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 47, -102, -102, -102, 0, 0, 0,
        // State 129
        0, -80, -80, -80, -80, 0, 0, 0, 0, 0, 0, -80, 0, -80, -80, -80, 0, -80, -80, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, -80, -80, 0, 0, 0,
        // State 130
        0, -81, -81, -81, -81, 0, 0, 0, 0, 0, 0, -81, 50, -81, -81, -81, 0, -81, -81, 0, -81, -81, -81, -81, -81, -81, -81, 0, -81, -81, -81, -81, -81, -81, -81, -81, -81, 0, 0, 0,
        // State 131
        0, -51, -51, -51, -51, 0, 0, 0, 0, 0, 0, -51, -51, -51, -51, -51, 0, -51, -51, 0, -51, -51, -51, -51, -51, -51, -51, 0, -51, -51, -51, -51, -51, -51, -51, -51, -51, 0, 0, 0,
        // State 132
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        0, -50, -50, -50, -50, 0, 0, 0, 0, 0, 0, -50, -50, -50, -50, -50, 0, -50, -50, 0, -50, -50, -50, -50, -50, -50, -50, 0, -50, -50, -50, -50, -50, -50, -50, -50, -50, 0, 0, 0,
        // State 134
        0, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, -48, -48, -48, -48, -48, 0, -48, -48, 0, -48, -48, -48, -48, -48, -48, -48, 0, -48, -48, -48, -48, -48, -48, -48, -48, -48, 0, 0, 0,
        // State 135
        0, -52, -52, -52, -52, 0, 0, 0, 0, 0, 52, -52, -52, -52, -52, -52, 0, -52, -52, 0, -52, -52, -52, -52, -52, -52, -52, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, 0, 0, 0,
        // State 136
        0, -47, -47, -47, -47, 0, 0, 0, 0, 0, 0, -47, -47, -47, -47, -47, 0, -47, -47, 0, -47, -47, -47, -47, -47, -47, -47, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, 0, 0,
        // State 137
        0, -49, -49, -49, -49, 0, 0, 0, 0, 0, 0, -49, -49, -49, -49, -49, 0, -49, -49, 0, -49, -49, -49, -49, -49, -49, -49, 0, -49, -49, -49, -49, -49, -49, -49, -49, -49, 0, 0, 0,
        // State 138
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -149, 0, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, -68, 0, -68, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 53, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, -58, 0, -58, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0,
        // State 142
        0, 0, 0, 0, 0, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        0, -37, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0,
        // State 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125, -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, -56, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -56, 0, 0, 0,
        // State 148
        0, -155, 59, -155, -155, 0, 0, 0, 0, 0, 0, 0, 0, 0, -155, -155, 0, -155, 0, 0, -155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -155, 0, 0, 0,
        // State 149
        0, -169, 0, -169, -169, 0, 0, 0, 0, 0, 0, 0, 0, 0, -169, -169, 0, -169, 0, 0, -169, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -169, 0, 0, 0,
        // State 150
        0, -83, -83, -83, -83, 0, 0, 0, 0, 0, 0, -83, 0, -83, -83, -83, 0, -83, -83, 0, -83, -83, -83, -83, -83, -83, -83, 0, -83, -83, -83, -83, -83, -83, -83, -83, -83, 0, 0, 0,
        // State 151
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 152
        0, -82, -82, -82, -82, 0, 0, 0, 0, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, 0, -82, -82, -82, -82, -82, -82, -82, 0, -82, -82, -82, -82, -82, -82, -82, -82, -82, 0, 0, 0,
        // State 153
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, -70, 0, -70, 186, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 155
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 0, 0, 0, -14, 0, 0, -14, -14, -14, -14, 0, 0, -14, -14, 0, -14, 0, 0, 0, 0, -14, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, -14,
        // State 157
        0, -135, -135, -135, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, -135, -135, 0, -135, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -135, 0, 0, 0,
        // State 158
        0, -171, 0, -171, -171, 0, 0, 0, 0, 0, 0, 0, 0, 0, -171, -171, 0, -171, 0, 0, -171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -171, 0, 0, 0,
        // State 159
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 160
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 161
        0, -112, -112, -112, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, -112, -112, 0, -112, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -112, 0, 0, 0,
        // State 162
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 163
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 164
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0,
        // State 165
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, -128, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 166
        0, -79, -79, -79, -79, 0, 0, 0, 0, 0, 0, -79, 0, -79, -79, -79, 0, -79, -79, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, -79, -79, 0, 0, 0,
        // State 167
        0, -88, -88, -88, -88, 0, 0, 0, 0, 0, 0, -88, 0, -88, -88, -88, 0, -88, -88, 0, -88, -88, -88, -88, -88, -88, -88, 0, -88, -88, -88, -88, -88, -88, -88, -88, -88, 0, 0, 0,
        // State 168
        0, -85, -85, -85, -85, 0, 0, 0, 0, 0, 0, -85, 0, -85, -85, -85, 0, -85, -85, 0, -85, -85, -85, -85, -85, -85, -85, 0, -85, -85, -85, -85, -85, -85, -85, -85, -85, 0, 0, 0,
        // State 169
        0, -86, -86, -86, -86, 0, 0, 0, 0, 0, 0, -86, 0, -86, -86, -86, 0, -86, -86, 0, -86, -86, -86, -86, -86, -86, -86, 0, -86, -86, -86, -86, -86, -86, -86, -86, -86, 0, 0, 0,
        // State 170
        0, -87, -87, -87, -87, 0, 0, 0, 0, 0, 0, -87, 0, -87, -87, -87, 0, -87, -87, 0, -87, -87, -87, -87, -87, -87, -87, 0, -87, -87, -87, -87, -87, -87, -87, -87, -87, 0, 0, 0,
        // State 171
        0, -90, -90, -90, -90, 0, 0, 0, 0, 0, 0, -90, 0, -90, -90, -90, 0, -90, -90, 0, -90, -90, -90, 38, 39, 40, 37, 0, -90, -90, -90, -90, -90, -90, -90, -90, -90, 0, 0, 0,
        // State 172
        0, -91, -91, -91, -91, 0, 0, 0, 0, 0, 0, -91, 0, -91, -91, -91, 0, -91, -91, 0, -91, -91, -91, 38, 39, 40, 37, 0, -91, -91, -91, -91, -91, -91, -91, -91, -91, 0, 0, 0,
        // State 173
        0, -93, -93, -93, -93, 0, 0, 0, 0, 0, 0, -93, 0, -93, -93, -93, 0, -93, -93, 0, -93, 41, 42, 0, 0, 0, 0, 0, -93, -93, -93, -93, -93, -93, -93, -93, -93, 0, 0, 0,
        // State 174
        0, -94, -94, -94, -94, 0, 0, 0, 0, 0, 0, -94, 0, -94, -94, -94, 0, -94, -94, 0, -94, 41, 42, 0, 0, 0, 0, 0, -94, -94, -94, -94, -94, -94, -94, -94, -94, 0, 0, 0,
        // State 175
        0, -95, -95, -95, -95, 0, 0, 0, 0, 0, 0, -95, 0, -95, -95, -95, 0, -95, -95, 0, -95, 41, 42, 0, 0, 0, 0, 0, -95, -95, -95, -95, -95, -95, -95, -95, -95, 0, 0, 0,
        // State 176
        0, -96, -96, -96, -96, 0, 0, 0, 0, 0, 0, -96, 0, -96, -96, -96, 0, -96, -96, 0, -96, 41, 42, 0, 0, 0, 0, 0, -96, -96, -96, -96, -96, -96, -96, -96, -96, 0, 0, 0,
        // State 177
        0, -99, -99, -99, -99, 0, 0, 0, 0, 0, 0, -99, 0, -99, -99, -99, 0, -99, -99, 0, -99, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 46, -99, -99, -99, -99, -99, 0, 0, 0,
        // State 178
        0, -98, -98, -98, -98, 0, 0, 0, 0, 0, 0, -98, 0, -98, -98, -98, 0, -98, -98, 0, -98, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 46, -98, -98, -98, -98, -98, 0, 0, 0,
        // State 179
        0, -101, -101, -101, -101, 0, 0, 0, 0, 0, 0, -101, 0, -101, -101, -101, 0, -101, -101, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 47, -101, -101, -101, 0, 0, 0,
        // State 180
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 198, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 181
        0, -53, -53, -53, -53, 0, 0, 0, 0, 0, 0, -53, -53, -53, -53, -53, 0, -53, -53, 0, -53, -53, -53, -53, -53, -53, -53, 0, -53, -53, -53, -53, -53, -53, -53, -53, -53, 0, 0, 0,
        // State 182
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 183
        0, -54, -54, -54, -54, 0, 0, 0, 0, 0, 0, -54, -54, -54, -54, -54, 0, -54, -54, 0, -54, -54, -54, -54, -54, -54, -54, 0, -54, -54, -54, -54, -54, -54, -54, -54, -54, 0, 0, 0,
        // State 184
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 185
        0, 0, 0, 0, 0, -15, 0, 0, -15, -15, -15, -15, 0, 0, -15, -15, 0, -15, 0, 0, 0, 0, -15, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, -15,
        // State 186
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 187
        0, -113, -113, -113, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, 0, -113, 0, 0, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -113, 0, 0, 0,
        // State 188
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 189
        0, -179, -179, -179, -179, 0, 0, 0, 0, 0, 0, 0, 204, 0, -179, -179, 0, -179, -179, -179, -179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -179, 0, 0, 0,
        // State 190
        0, -9, -9, -9, -9, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, -9, 0, -9, -9, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0,
        // State 191
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 192
        0, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0,
        // State 193
        0, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, -62, 0, -62, 157, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -62, 0, 0, 0,
        // State 194
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 0, 0, 0, -127, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 195
        0, -134, -134, -134, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, -134, 0, -134, 0, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, 0, 0, 0,
        // State 196
        0, -170, 0, -170, -170, 0, 0, 0, 0, 0, 0, 0, 0, 0, -170, -170, 0, -170, 0, 0, -170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -170, 0, 0, 0,
        // State 197
        0, -140, -140, -140, -140, 0, 0, 0, 0, 0, 0, -140, -140, -140, -140, -140, 0, -140, -140, 0, -140, -140, -140, -140, -140, -140, -140, 0, -140, -140, -140, -140, -140, -140, -140, -140, -140, 0, 0, 0,
        // State 198
        0, -142, -142, -142, -142, 0, 0, 0, 0, 0, 0, -142, -142, -142, -142, -142, 0, -142, -142, 0, -142, -142, -142, -142, -142, -142, -142, 0, -142, -142, -142, -142, -142, -142, -142, -142, -142, 0, 0, 0,
        // State 199
        0, -141, -141, -141, -141, 0, 0, 0, 0, 0, 0, -141, -141, -141, -141, -141, 0, -141, -141, 0, -141, -141, -141, -141, -141, -141, -141, 0, -141, -141, -141, -141, -141, -141, -141, -141, -141, 0, 0, 0,
        // State 200
        0, -10, -10, -10, -10, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, -10, 0, -10, -10, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0,
        // State 201
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 202
        0, -115, -115, -115, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, -115, 0, -115, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, 0, 0, 0,
        // State 203
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 210, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 204
        0, -181, -181, -181, -181, 0, 0, 0, 0, 0, 0, 0, 204, 0, -181, -181, 0, -181, -181, -181, -181, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -181, 0, 0, 0,
        // State 205
        0, -114, -114, -114, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, -114, 0, -114, 0, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0,
        // State 206
        0, -6, -6, -6, -6, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, -6, 0, -6, -6, -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0,
        // State 207
        0, -63, -63, -63, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, -63, 0, -63, 186, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -63, 0, 0, 0,
        // State 208
        0, -116, -116, -116, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, -116, 0, -116, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, 0, 0, 0,
        // State 209
        0, -7, -7, -7, -7, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, -7, 0, -7, -7, -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0,
        // State 210
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 211
        0, -77, -77, -77, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, -77, 0, -77, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -77, 0, 0, 0,
        // State 212
        0, -117, -117, -117, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, -117, 0, -117, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, -117, 0, 0, 0,
        // State 213
        0, -78, -78, -78, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, -78, 0, -78, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -144,
        // State 1
        -146,
        // State 2
        -145,
        // State 3
        -147,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        -161,
        // State 72
        -162,
        // State 73
        -183,
        // State 74
        -165,
        // State 75
        -176,
        // State 76
        0,
        // State 77
        0,
        // State 78
        -166,
        // State 79
        -177,
        // State 80
        -173,
        // State 81
        -172,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        -167,
        // State 86
        -108,
        // State 87
        -60,
        // State 88
        -59,
        // State 89
        0,
        // State 90
        0,
        // State 91
        -107,
        // State 92
        0,
        // State 93
        0,
        // State 94
        -106,
        // State 95
        0,
        // State 96
        -110,
        // State 97
        0,
        // State 98
        -133,
        // State 99
        -132,
        // State 100
        -131,
        // State 101
        -129,
        // State 102
        -168,
        // State 103
        -105,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        0,
        // State 113
        0,
        // State 114
        -109,
        // State 115
        -130,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        0,
        // State 121
        -61,
        // State 122
        0,
        // State 123
        0,
        // State 124
        0,
        // State 125
        0,
        // State 126
        0,
        // State 127
        0,
        // State 128
        0,
        // State 129
        0,
        // State 130
        0,
        // State 131
        0,
        // State 132
        0,
        // State 133
        0,
        // State 134
        0,
        // State 135
        0,
        // State 136
        0,
        // State 137
        0,
        // State 138
        0,
        // State 139
        0,
        // State 140
        0,
        // State 141
        0,
        // State 142
        0,
        // State 143
        0,
        // State 144
        0,
        // State 145
        0,
        // State 146
        0,
        // State 147
        0,
        // State 148
        0,
        // State 149
        0,
        // State 150
        0,
        // State 151
        0,
        // State 152
        0,
        // State 153
        0,
        // State 154
        0,
        // State 155
        0,
        // State 156
        0,
        // State 157
        0,
        // State 158
        0,
        // State 159
        0,
        // State 160
        0,
        // State 161
        0,
        // State 162
        0,
        // State 163
        0,
        // State 164
        0,
        // State 165
        0,
        // State 166
        0,
        // State 167
        0,
        // State 168
        0,
        // State 169
        0,
        // State 170
        0,
        // State 171
        0,
        // State 172
        0,
        // State 173
        0,
        // State 174
        0,
        // State 175
        0,
        // State 176
        0,
        // State 177
        0,
        // State 178
        0,
        // State 179
        0,
        // State 180
        0,
        // State 181
        0,
        // State 182
        0,
        // State 183
        0,
        // State 184
        0,
        // State 185
        0,
        // State 186
        0,
        // State 187
        0,
        // State 188
        0,
        // State 189
        0,
        // State 190
        0,
        // State 191
        0,
        // State 192
        0,
        // State 193
        0,
        // State 194
        0,
        // State 195
        0,
        // State 196
        0,
        // State 197
        0,
        // State 198
        0,
        // State 199
        0,
        // State 200
        0,
        // State 201
        0,
        // State 202
        0,
        // State 203
        0,
        // State 204
        0,
        // State 205
        0,
        // State 206
        0,
        // State 207
        0,
        // State 208
        0,
        // State 209
        0,
        // State 210
        0,
        // State 211
        0,
        // State 212
        0,
        // State 213
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            3 => match state {
                61 => 204,
                _ => 189,
            },
            5 => match state {
                18 => 140,
                34 => 165,
                59 => 201,
                _ => 61,
            },
            8 => match state {
                56 | 60 | 65 | 70 => 64,
                _ => 26,
            },
            11 => 33,
            14 => 6,
            17 => 13,
            20 => 14,
            25 => 122,
            26 => 106,
            27 => match state {
                30 => 55,
                _ => 85,
            },
            28 => match state {
                7 => 94,
                10 => 103,
                _ => 107,
            },
            29 => match state {
                60 => 202,
                65 => 208,
                70 => 213,
                _ => 192,
            },
            30 => match state {
                29 => 160,
                52 => 186,
                67 => 210,
                _ => 144,
            },
            31 => 104,
            32 => match state {
                25 => 153,
                28 => 159,
                51 => 184,
                _ => 138,
            },
            33 => 82,
            34 => 205,
            35 => match state {
                15 => 21,
                17 => 27,
                20 => 147,
                23 => 151,
                26 => 154,
                31 => 161,
                32 | 63 => 162,
                49 => 180,
                50 => 182,
                53 => 187,
                54 | 66 => 188,
                56 | 60 | 65 | 70 => 193,
                64 => 207,
                68 => 211,
                69 => 212,
                _ => 139,
            },
            36 => 123,
            37 => match state {
                22 => 150,
                24 => 152,
                36 => 167,
                37 => 168,
                38 => 169,
                39 => 170,
                _ => 124,
            },
            38 => match state {
                40 => 171,
                41 => 172,
                _ => 125,
            },
            39 => match state {
                42 => 173,
                43 => 174,
                44 => 175,
                45 => 176,
                _ => 126,
            },
            40 => match state {
                46 => 177,
                47 => 178,
                _ => 127,
            },
            41 => match state {
                48 => 179,
                _ => 128,
            },
            42 => match state {
                35 => 166,
                _ => 129,
            },
            44 => 71,
            45 => 72,
            46 => 141,
            50 => match state {
                33 => 163,
                _ => 145,
            },
            51 => 96,
            52 => match state {
                21 => 148,
                27 => 157,
                58 => 195,
                _ => 108,
            },
            53 => match state {
                6 => 89,
                _ => 83,
            },
            55 => 130,
            56 => 73,
            57 => match state {
                14 => 118,
                _ => 109,
            },
            59 => 10,
            61 => match state {
                14 => 119,
                21 => 149,
                _ => 110,
            },
            62 => 111,
            63 => match state {
                1 | 3 => 78,
                _ => 74,
            },
            65 => match state {
                2 => 3,
                _ => 1,
            },
            66 => match state {
                8 => 95,
                11 => 105,
                13 => 116,
                57 => 194,
                _ => 86,
            },
            67 => match state {
                27 => 158,
                58 => 196,
                _ => 112,
            },
            68 => match state {
                2 => 79,
                _ => 75,
            },
            70 => 2,
            71 => 62,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###""use""###,
        r###""if""###,
        r###""else""###,
        r###""while""###,
        r###""return""###,
        r###""length""###,
        r###""int""###,
        r###""bool""###,
        r###""true""###,
        r###""false""###,
        r###""(""###,
        r###"")""###,
        r###""[""###,
        r###""]""###,
        r###""{""###,
        r###""}""###,
        r###"":""###,
        r###"";""###,
        r###"",""###,
        r###""=""###,
        r###""_""###,
        r###""+""###,
        r###""-""###,
        r###""*""###,
        r###""*>>""###,
        r###""/""###,
        r###""%""###,
        r###""!""###,
        r###""<""###,
        r###""<=""###,
        r###"">""###,
        r###"">=""###,
        r###""==""###,
        r###""!=""###,
        r###""&""###,
        r###""|""###,
        r###"IDENT"###,
        r###"INT_LIT"###,
        r###"CHAR_LIT"###,
        r###"STRING_LIT"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i16],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = String;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Program;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 40 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            Token::Use if true => Some(0),
            Token::If if true => Some(1),
            Token::Else if true => Some(2),
            Token::While if true => Some(3),
            Token::Return if true => Some(4),
            Token::Length if true => Some(5),
            Token::IntType if true => Some(6),
            Token::BoolType if true => Some(7),
            Token::True if true => Some(8),
            Token::False if true => Some(9),
            Token::LParen if true => Some(10),
            Token::RParen if true => Some(11),
            Token::LBracket if true => Some(12),
            Token::RBracket if true => Some(13),
            Token::LBrace if true => Some(14),
            Token::RBrace if true => Some(15),
            Token::Colon if true => Some(16),
            Token::Semicolon if true => Some(17),
            Token::Comma if true => Some(18),
            Token::Assign if true => Some(19),
            Token::Underscore if true => Some(20),
            Token::Plus if true => Some(21),
            Token::Minus if true => Some(22),
            Token::Mul if true => Some(23),
            Token::HighMul if true => Some(24),
            Token::Div if true => Some(25),
            Token::Mod if true => Some(26),
            Token::Not if true => Some(27),
            Token::Lt if true => Some(28),
            Token::Le if true => Some(29),
            Token::Gt if true => Some(30),
            Token::Ge if true => Some(31),
            Token::Eq if true => Some(32),
            Token::Ne if true => Some(33),
            Token::And if true => Some(34),
            Token::Or if true => Some(35),
            Token::Identifier(_) if true => Some(36),
            Token::IntLiteral(_) if true => Some(37),
            Token::CharLiteral(_) if true => Some(38),
            Token::StringLiteral(_) if true => Some(39),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 => __Symbol::Variant0(__token),
            36 | 39 => match __token {
                Token::Identifier(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            37 | 38 => match __token {
                Token::IntLiteral(__tok0) | Token::CharLiteral(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 24,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 28,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 29,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 30,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 31,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 33,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 34,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 34,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 34,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 35,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 37,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 37,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 39,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 39,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 40,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 41,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 41,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 42,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 43,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 44,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 44,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 45,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 45,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 45,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 45,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 46,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 46,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 46,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 46,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 46,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 48,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 48,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 48,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 48,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 49,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 51,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 52,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 52,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 53,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 54,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 55,
                }
            }
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            143 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 56,
                }
            }
            144 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            145 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            146 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 56,
                }
            }
            147 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 57,
                }
            }
            148 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 57,
                }
            }
            149 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            150 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 58,
                }
            }
            151 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 59,
                }
            }
            152 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            153 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 60,
                }
            }
            154 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            155 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            156 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            157 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 62,
                }
            }
            158 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 62,
                }
            }
            159 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            160 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            161 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            162 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 64,
                }
            }
            163 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            164 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            165 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 65,
                }
            }
            166 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            167 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 66,
                }
            }
            168 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            169 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 67,
                }
            }
            170 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            171 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 68,
                }
            }
            172 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 68,
                }
            }
            173 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 69,
                }
            }
            174 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            175 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            176 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 70,
                }
            }
            177 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 71,
                }
            }
            178 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            179 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            180 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 71,
                }
            }
            181 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 72,
                }
            }
            182 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {__reduce_index}")
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl Default for ProgramParser { fn default() -> Self { Self::new() } }
    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Program, __lalrpop_util::ParseError<usize, Token, String>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
    >(
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token, String>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                __reduce94(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            95 => {
                __reduce95(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            96 => {
                __reduce96(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            97 => {
                __reduce97(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            98 => {
                __reduce98(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            99 => {
                __reduce99(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            100 => {
                __reduce100(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            101 => {
                __reduce101(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            102 => {
                __reduce102(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            103 => {
                __reduce103(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            104 => {
                __reduce104(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            105 => {
                __reduce105(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            106 => {
                __reduce106(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            107 => {
                __reduce107(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            108 => {
                __reduce108(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            109 => {
                __reduce109(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            110 => {
                __reduce110(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            111 => {
                __reduce111(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            112 => {
                __reduce112(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            113 => {
                __reduce113(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            114 => {
                __reduce114(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            115 => {
                __reduce115(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            116 => {
                __reduce116(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            117 => {
                __reduce117(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            118 => {
                __reduce118(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            119 => {
                __reduce119(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            120 => {
                __reduce120(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            121 => {
                __reduce121(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            122 => {
                __reduce122(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            123 => {
                __reduce123(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            124 => {
                __reduce124(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            125 => {
                __reduce125(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            126 => {
                __reduce126(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            127 => {
                __reduce127(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            128 => {
                __reduce128(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            129 => {
                __reduce129(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            130 => {
                __reduce130(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            131 => {
                __reduce131(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            132 => {
                __reduce132(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            133 => {
                __reduce133(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            134 => {
                __reduce134(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            135 => {
                __reduce135(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            136 => {
                __reduce136(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            137 => {
                __reduce137(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            138 => {
                __reduce138(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            139 => {
                __reduce139(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            140 => {
                __reduce140(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            141 => {
                __reduce141(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            142 => {
                __reduce142(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            143 => {
                __reduce143(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            144 => {
                __reduce144(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            145 => {
                __reduce145(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            146 => {
                __reduce146(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            147 => {
                __reduce147(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            148 => {
                __reduce148(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            149 => {
                __reduce149(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            150 => {
                __reduce150(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            151 => {
                __reduce151(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            152 => {
                __reduce152(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            153 => {
                __reduce153(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            154 => {
                __reduce154(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            155 => {
                __reduce155(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            156 => {
                __reduce156(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            157 => {
                __reduce157(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            158 => {
                __reduce158(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            159 => {
                __reduce159(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            160 => {
                __reduce160(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            161 => {
                __reduce161(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            162 => {
                __reduce162(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            163 => {
                __reduce163(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            164 => {
                __reduce164(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            165 => {
                __reduce165(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            166 => {
                __reduce166(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            167 => {
                __reduce167(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            168 => {
                __reduce168(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            169 => {
                __reduce169(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            170 => {
                __reduce170(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            171 => {
                __reduce171(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            172 => {
                __reduce172(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            173 => {
                __reduce173(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            174 => {
                __reduce174(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            175 => {
                __reduce175(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            176 => {
                __reduce176(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            177 => {
                __reduce177(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            178 => {
                __reduce178(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            179 => {
                __reduce179(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            180 => {
                __reduce180(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            181 => {
                __reduce181(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            182 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant33(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {__action}")
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Stmt, Option<Token>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (Token, Token), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AssignTarget, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Block, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, DeclSuffix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, FuncDef, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, GlobalDecl, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, IdentStmtRest, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Interface, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, InterfaceDecl, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant30(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant32<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant32(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant34<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Vec<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant34(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant35<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<Vec<Type>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant35(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Param, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant33<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant33(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant36<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, TopLevelItem, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant36(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AssignTarget>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Option<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(Token, Token)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<AssignTarget>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<InterfaceDecl>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant31(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Option<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant38<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant38(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant37<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<TopLevelItem>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant37(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? = ";" => ActionFn(114);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action114::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ";"? =  => ActionFn(115);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action115::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]") = "[", "]" => ActionFn(96);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action96::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")* =  => ActionFn(94);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action94::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")* = ("[" "]")+ => ActionFn(95);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")+ = "[", "]" => ActionFn(166);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action166::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" "]")+ = ("[" "]")+, "[", "]" => ActionFn(167);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action167::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]") = "[", Expr, "]" => ActionFn(101);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action101::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 4)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]")+ = "[", Expr, "]" => ActionFn(172);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action172::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 5)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("[" <Expr> "]")+ = ("[" <Expr> "]")+, "[", Expr, "]" => ActionFn(173);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action173::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 5)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action141::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* =  => ActionFn(139);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action139::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(140);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(174);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action174::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 8)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(175);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action175::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 8)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",") = LhsItem, "," => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action134::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")* =  => ActionFn(132);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action132::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")* = (<LhsItem> ",")+ => ActionFn(133);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action133::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 10)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")+ = LhsItem, "," => ActionFn(180);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action180::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 11)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<LhsItem> ",")+ = (<LhsItem> ",")+, LhsItem, "," => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 11)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(128);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action128::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action126::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(127);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(184);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action184::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 14)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(185);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action185::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (3, 14)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",") = Type, "," => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 15)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")* =  => ActionFn(129);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action129::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")* = (<Type> ",")+ => ActionFn(130);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")+ = Type, "," => ActionFn(188);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action188::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 17)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Type> ",")+ = (<Type> ",")+, Type, "," => ActionFn(189);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action189::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (3, 17)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?) = Stmt, ";" => ActionFn(154);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action154::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 18)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?) = Stmt => ActionFn(155);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action155::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)* =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action105::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)* = (Stmt ";"?)+ => ActionFn(106);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = Stmt, ";" => ActionFn(192);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action192::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = Stmt => ActionFn(193);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action193::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = (Stmt ";"?)+, Stmt, ";" => ActionFn(194);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action194::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 20)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (Stmt ";"?)+ = (Stmt ";"?)+, Stmt => ActionFn(195);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action195::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim = "[", "]" => ActionFn(38);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 21)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim = "[", Expr, "]" => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 21)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim* =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action97::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 22)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim* = ArrayDim+ => ActionFn(98);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 22)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim+ = ArrayDim => ActionFn(142);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action142::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDim+ = ArrayDim+, ArrayDim => ActionFn(143);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant16(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action143::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 23)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDimList =  => ActionFn(198);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action198::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 24)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArrayDimList = ArrayDim+ => ActionFn(199);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action199::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = INT_LIT => ActionFn(77);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = CHAR_LIT => ActionFn(78);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = STRING_LIT => ActionFn(79);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "true" => ActionFn(80);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action80::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "false" => ActionFn(81);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = IDENT => ActionFn(82);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action82::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 25)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "(", Expr, ")" => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 25)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomExpr = "{", Comma<Expr>, "}" => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 25)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = Block => ActionFn(23);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 26)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = "_", "=", Expr => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 26)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = "_", ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(25);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant21(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 26)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtomicStmt = IDENT, IdentStmtRest => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant28(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 26)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BaseType = "int" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 27)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // BaseType = "bool" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 27)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Block = "{", StmtList, "}" => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant20(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (3, 28)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Expr> = Expr => ActionFn(176);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action176::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 29)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Expr> = (<Expr> ",")+, Expr => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action177::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 29)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<LhsItem> = LhsItem => ActionFn(182);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action182::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 30)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<LhsItem> = (<LhsItem> ",")+, LhsItem => ActionFn(183);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action183::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 30)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Type> = Type => ActionFn(190);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action190::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 31)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma1<Type> = (<Type> ",")+, Type => ActionFn(191);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action191::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 31)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = Expr => ActionFn(200);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action200::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 32)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> =  => ActionFn(201);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action201::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 32)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(202);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action202::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 32)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(203);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action203::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 32)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(204);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action204::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 33)
    }
    fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(205);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action205::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 33)
    }
    fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(206);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action206::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 33)
    }
    fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(207);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action207::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 33)
    }
    fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action42::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 34)
    }
    fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix = "=", Expr => ActionFn(43);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 34)
    }
    fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclSuffix = ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant21(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (4, 34)
    }
    fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "|", Expr6 => ActionFn(71);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 35)
    }
    fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr6 => ActionFn(72);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 35)
    }
    fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr0 = PostfixExpr => ActionFn(49);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 36)
    }
    fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "-", Expr1 => ActionFn(50);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action50::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 37)
    }
    fn __reduce82<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = "!", Expr1 => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 37)
    }
    fn __reduce83<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr1 = Expr0 => ActionFn(52);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 37)
    }
    fn __reduce84<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "*", Expr1 => ActionFn(53);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action53::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce85<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "*>>", Expr1 => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce86<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "/", Expr1 => ActionFn(55);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce87<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr2, "%", Expr1 => ActionFn(56);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action56::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 38)
    }
    fn __reduce88<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr2 = Expr1 => ActionFn(57);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 38)
    }
    fn __reduce89<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "+", Expr2 => ActionFn(58);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 39)
    }
    fn __reduce90<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr3, "-", Expr2 => ActionFn(59);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 39)
    }
    fn __reduce91<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr3 = Expr2 => ActionFn(60);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action60::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 39)
    }
    fn __reduce92<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "<", Expr3 => ActionFn(61);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action61::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, "<=", Expr3 => ActionFn(62);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action62::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, ">", Expr3 => ActionFn(63);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr4, ">=", Expr3 => ActionFn(64);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 40)
    }
    fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr4 = Expr3 => ActionFn(65);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 40)
    }
    fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "==", Expr4 => ActionFn(66);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 41)
    }
    fn __reduce98<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr5, "!=", Expr4 => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 41)
    }
    fn __reduce99<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr5 = Expr4 => ActionFn(68);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 41)
    }
    fn __reduce100<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr6, "&", Expr5 => ActionFn(69);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 42)
    }
    fn __reduce101<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr6 = Expr5 => ActionFn(70);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 42)
    }
    fn __reduce102<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr? = Expr => ActionFn(137);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action137::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 43)
    }
    fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr? =  => ActionFn(138);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action138::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 43)
    }
    fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncDef = IDENT, "(", Comma<Param>, ")", ReturnTypes, Block => ActionFn(212);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant20(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action212::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (6, 44)
    }
    fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncDef = IDENT, "(", Comma<Param>, ")", Block => ActionFn(213);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant20(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action213::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (5, 44)
    }
    fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, ";" => ActionFn(156);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action156::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (4, 45)
    }
    fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type => ActionFn(157);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action157::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (3, 45)
    }
    fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, "=", Literal, ";" => ActionFn(158);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action158::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (6, 45)
    }
    fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // GlobalDecl = IDENT, ":", Type, "=", Literal => ActionFn(159);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action159::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (5, 45)
    }
    fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "(", Comma<Expr>, ")" => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (3, 46)
    }
    fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "=", Expr => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (2, 46)
    }
    fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ("[" <Expr> "]")+, "=", Expr => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (3, 46)
    }
    fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ":", BaseType, ValidArrayDims, DeclSuffix => ActionFn(33);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (4, 46)
    }
    fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(34);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant21(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (4, 46)
    }
    fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = ("[" <Expr> "]")+, ",", Comma1<LhsItem>, "=", Comma1<Expr> => ActionFn(35);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant21(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant22(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (5, 46)
    }
    fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdentStmtRest = "(", Comma<Expr>, ")", ("[" <Expr> "]")+, "=", Expr => ActionFn(36);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action36::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (6, 46)
    }
    fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Interface = InterfaceDecl+ => ActionFn(90);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action90::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 47)
    }
    fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ReturnTypes, ";" => ActionFn(214);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action214::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (6, 48)
    }
    fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ";" => ActionFn(215);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action215::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (5, 48)
    }
    fn __reduce120<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")", ReturnTypes => ActionFn(216);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action216::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (5, 48)
    }
    fn __reduce121<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl = IDENT, "(", Comma<Param>, ")" => ActionFn(217);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action217::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (4, 48)
    }
    fn __reduce122<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl+ = InterfaceDecl => ActionFn(92);
        let __sym0 = __pop_Variant30(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action92::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (1, 49)
    }
    fn __reduce123<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // InterfaceDecl+ = InterfaceDecl+, InterfaceDecl => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant30(__symbols);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (2, 49)
    }
    fn __reduce124<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = "_" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 50)
    }
    fn __reduce125<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT => ActionFn(46);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 50)
    }
    fn __reduce126<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT, ":", Type => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 50)
    }
    fn __reduce127<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // LhsItem = IDENT, ("[" <Expr> "]")+ => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 50)
    }
    fn __reduce128<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = INT_LIT => ActionFn(85);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action85::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce129<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", INT_LIT => ActionFn(86);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 51)
    }
    fn __reduce130<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = CHAR_LIT => ActionFn(87);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action87::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce131<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "true" => ActionFn(88);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action88::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce132<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "false" => ActionFn(89);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action89::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 51)
    }
    fn __reduce133<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = "if", Expr, MatchedIf, "else", MatchedIf => ActionFn(17);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant19(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 52)
    }
    fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = "while", Expr, MatchedIf => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 52)
    }
    fn __reduce135<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MatchedIf = AtomicStmt => ActionFn(19);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 52)
    }
    fn __reduce136<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = IDENT, ":", Type => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 53)
    }
    fn __reduce137<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(124);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action124::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (1, 54)
    }
    fn __reduce138<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action125::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (0, 54)
    }
    fn __reduce139<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = PostfixExpr, "[", Expr, "]" => ActionFn(73);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce140<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = IDENT, "(", Comma<Expr>, ")" => ActionFn(74);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant21(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce141<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = "length", "(", Expr, ")" => ActionFn(75);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action75::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 55)
    }
    fn __reduce142<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PostfixExpr = AtomExpr => ActionFn(76);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 55)
    }
    fn __reduce143<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(220);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action220::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (0, 56)
    }
    fn __reduce144<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = UseDecl+ => ActionFn(221);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action221::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 56)
    }
    fn __reduce145<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = TopLevelItem+ => ActionFn(222);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action222::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 56)
    }
    fn __reduce146<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = UseDecl+, TopLevelItem+ => ActionFn(223);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant37(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action223::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (2, 56)
    }
    fn __reduce147<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt = "return", Comma<Expr>, ";" => ActionFn(162);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action162::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (3, 57)
    }
    fn __reduce148<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt = "return", Comma<Expr> => ActionFn(163);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action163::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 57)
    }
    fn __reduce149<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt? = ReturnStmt => ActionFn(103);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action103::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (1, 58)
    }
    fn __reduce150<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnStmt? =  => ActionFn(104);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action104::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (0, 58)
    }
    fn __reduce151<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes = ":", Comma1<Type> => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant23(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 59)
    }
    fn __reduce152<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes? = ReturnTypes => ActionFn(111);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 60)
    }
    fn __reduce153<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ReturnTypes? =  => ActionFn(112);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action112::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (0, 60)
    }
    fn __reduce154<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = MatchedIf => ActionFn(15);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 61)
    }
    fn __reduce155<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = UnmatchedIf => ActionFn(16);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 61)
    }
    fn __reduce156<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = ReturnStmt => ActionFn(208);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action208::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 62)
    }
    fn __reduce157<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList =  => ActionFn(209);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action209::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 62)
    }
    fn __reduce158<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = (Stmt ";"?)+, ReturnStmt => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action210::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 62)
    }
    fn __reduce159<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtList = (Stmt ";"?)+ => ActionFn(211);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action211::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 62)
    }
    fn __reduce160<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem = FuncDef => ActionFn(4);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 63)
    }
    fn __reduce161<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem = GlobalDecl => ActionFn(5);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 63)
    }
    fn __reduce162<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem* =  => ActionFn(116);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action116::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (0, 64)
    }
    fn __reduce163<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem* = TopLevelItem+ => ActionFn(117);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action117::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 64)
    }
    fn __reduce164<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem+ = TopLevelItem => ActionFn(122);
        let __sym0 = __pop_Variant36(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action122::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 65)
    }
    fn __reduce165<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TopLevelItem+ = TopLevelItem+, TopLevelItem => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant36(__symbols);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action123::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (2, 65)
    }
    fn __reduce166<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = BaseType => ActionFn(11);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 66)
    }
    fn __reduce167<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Type = Type, "[", "]" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 66)
    }
    fn __reduce168<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "if", Expr, Stmt => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 67)
    }
    fn __reduce169<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "if", Expr, MatchedIf, "else", UnmatchedIf => ActionFn(21);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant19(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (5, 67)
    }
    fn __reduce170<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UnmatchedIf = "while", Expr, UnmatchedIf => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant19(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 67)
    }
    fn __reduce171<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl = "use", IDENT, ";" => ActionFn(164);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action164::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 68)
    }
    fn __reduce172<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl = "use", IDENT => ActionFn(165);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action165::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 68)
    }
    fn __reduce173<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl* =  => ActionFn(118);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action118::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (0, 69)
    }
    fn __reduce174<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl* = UseDecl+ => ActionFn(119);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action119::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 69)
    }
    fn __reduce175<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl+ = UseDecl => ActionFn(120);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 70)
    }
    fn __reduce176<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // UseDecl+ = UseDecl+, UseDecl => ActionFn(121);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action121::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (2, 70)
    }
    fn __reduce177<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims =  => ActionFn(168);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action168::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 71)
    }
    fn __reduce178<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" "]")+ => ActionFn(169);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 71)
    }
    fn __reduce179<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" <Expr> "]")+ => ActionFn(170);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 71)
    }
    fn __reduce180<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ValidArrayDims = ("[" <Expr> "]")+, ("[" "]")+ => ActionFn(171);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action171::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 71)
    }
    fn __reduce181<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Interface = Interface => ActionFn(1);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 72)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Program::ProgramParser;

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
>(
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
>(
    (_, __0, _): (usize, Interface, usize),
) -> Interface
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
>(
    (_, uses, _): (usize, alloc::vec::Vec<String>, usize),
    (_, items, _): (usize, alloc::vec::Vec<TopLevelItem>, usize),
) -> Program
{
    Program { uses, items }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> String
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
>(
    (_, __0, _): (usize, FuncDef, usize),
) -> TopLevelItem
{
    TopLevelItem::Func(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
>(
    (_, __0, _): (usize, GlobalDecl, usize),
) -> TopLevelItem
{
    TopLevelItem::Global(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, ty, _): (usize, Type, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> GlobalDecl
{
    GlobalDecl { name, ty, init: None }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, ty, _): (usize, Type, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> GlobalDecl
{
    GlobalDecl { name, ty, init: Some(e) }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, params, _): (usize, Vec<Param>, usize),
    (_, _, _): (usize, Token, usize),
    (_, ret, _): (usize, Option<Vec<Type>>, usize),
    (_, body, _): (usize, Block, usize),
) -> FuncDef
{
    FuncDef {
            name,
            params,
            returns: ret.unwrap_or_default(),
            body,
        }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, ty, _): (usize, Type, usize),
) -> Param
{
    Param { name, ty }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Vec<Type>, usize),
) -> Vec<Type>
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
>(
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
>(
    (_, t, _): (usize, Type, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
) -> Type
{
    Type::Array(Box::new(t))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
>(
    (_, __0, _): (usize, Token, usize),
) -> Type
{
    Type::Int
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
>(
    (_, __0, _): (usize, Token, usize),
) -> Type
{
    Type::Bool
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
>(
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
>(
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
>(
    (_, _, _): (usize, Token, usize),
    (_, g, _): (usize, Expr, usize),
    (_, then, _): (usize, Stmt, usize),
    (_, _, _): (usize, Token, usize),
    (_, els, _): (usize, Stmt, usize),
) -> Stmt
{
    Stmt::If(g, then, Some(els))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
>(
    (_, _, _): (usize, Token, usize),
    (_, g, _): (usize, Expr, usize),
    (_, body, _): (usize, Stmt, usize),
) -> Stmt
{
    Stmt::While(g, body)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
>(
    (_, __0, _): (usize, Stmt, usize),
) -> Stmt
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
>(
    (_, _, _): (usize, Token, usize),
    (_, g, _): (usize, Expr, usize),
    (_, then, _): (usize, Stmt, usize),
) -> Stmt
{
    Stmt::If(g, then, None)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
>(
    (_, _, _): (usize, Token, usize),
    (_, g, _): (usize, Expr, usize),
    (_, then, _): (usize, Stmt, usize),
    (_, _, _): (usize, Token, usize),
    (_, els, _): (usize, Stmt, usize),
) -> Stmt
{
    Stmt::If(g, then, Some(els))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
>(
    (_, _, _): (usize, Token, usize),
    (_, g, _): (usize, Expr, usize),
    (_, body, _): (usize, Stmt, usize),
) -> Stmt
{
    Stmt::While(g, body)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
>(
    (_, __0, _): (usize, Block, usize),
) -> Stmt
{
    Stmt::Block(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Stmt
{
    Stmt::Assign(vec![AssignTarget::Discard], vec![e])
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, rest, _): (usize, Vec<AssignTarget>, usize),
    (_, _, _): (usize, Token, usize),
    (_, vals, _): (usize, Vec<Expr>, usize),
) -> Stmt
{
    {
            let mut targets = vec![AssignTarget::Discard];
            targets.extend(rest);
            Stmt::Assign(targets, vals)
        }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
>(
    (_, name, _): (usize, String, usize),
    (_, rest, _): (usize, IdentStmtRest, usize),
) -> Stmt
{
    rest.into_stmt(name)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
>(
    (_, _, _): (usize, Token, usize),
    (_, stmts, _): (usize, Block, usize),
    (_, _, _): (usize, Token, usize),
) -> Block
{
    stmts
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
>(
    (_, stmts, _): (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    (_, ret, _): (usize, Option<Vec<Expr>>, usize),
) -> Block
{
    Block { stmts, return_val: ret }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> Vec<Expr>
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
>(
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
) -> IdentStmtRest
{
    IdentStmtRest::ProcCall(args)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> IdentStmtRest
{
    IdentStmtRest::Assign(e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
>(
    (_, indices, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> IdentStmtRest
{
    IdentStmtRest::ArrayAssign(indices, e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
>(
    (_, _, _): (usize, Token, usize),
    (_, base, _): (usize, Type, usize),
    (_, dims, _): (usize, Vec<Option<Expr>>, usize),
    (_, suffix, _): (usize, DeclSuffix, usize),
) -> IdentStmtRest
{
    {
         IdentStmtRest::UnifiedDecl(base, dims, suffix)
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
>(
    (_, _, _): (usize, Token, usize),
    (_, rest, _): (usize, Vec<AssignTarget>, usize),
    (_, _, _): (usize, Token, usize),
    (_, vals, _): (usize, Vec<Expr>, usize),
) -> IdentStmtRest
{
    IdentStmtRest::MultiAssign(rest, vals)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
>(
    (_, indices, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, rest, _): (usize, Vec<AssignTarget>, usize),
    (_, _, _): (usize, Token, usize),
    (_, vals, _): (usize, Vec<Expr>, usize),
) -> IdentStmtRest
{
    IdentStmtRest::MultiArrayAssign(indices, rest, vals)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
>(
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, indices, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> IdentStmtRest
{
    IdentStmtRest::CallIndexAssign(args, indices, e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
>(
    (_, d, _): (usize, alloc::vec::Vec<Option<Expr>>, usize),
) -> Vec<Option<Expr>>
{
    d
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
>(
    (_, __0, _): (usize, Token, usize),
    (_, __1, _): (usize, Token, usize),
) -> Option<Expr>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Option<Expr>
{
    Some(e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
>(
    (_, empty, _): (usize, alloc::vec::Vec<(Token, Token)>, usize),
) -> Vec<Option<Expr>>
{
    vec![None; empty.len()]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
>(
    (_, filled, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, empty, _): (usize, alloc::vec::Vec<(Token, Token)>, usize),
) -> Vec<Option<Expr>>
{
    {
        let mut dims: Vec<Option<Expr>> = filled.into_iter().map(Some).collect();
        dims.extend(empty.into_iter().map(|_| None));
        dims
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> DeclSuffix
{
    DeclSuffix::None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> DeclSuffix
{
    DeclSuffix::Init(e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
>(
    (_, _, _): (usize, Token, usize),
    (_, rest, _): (usize, Vec<AssignTarget>, usize),
    (_, _, _): (usize, Token, usize),
    (_, vals, _): (usize, Vec<Expr>, usize),
) -> DeclSuffix
{
    DeclSuffix::Multi(rest, vals)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
>(
    (_, __0, _): (usize, Token, usize),
) -> AssignTarget
{
    AssignTarget::Discard
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
>(
    (_, name, _): (usize, String, usize),
) -> AssignTarget
{
    AssignTarget::Var(name)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, ty, _): (usize, Type, usize),
) -> AssignTarget
{
    AssignTarget::Decl(name, ty)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
>(
    (_, name, _): (usize, String, usize),
    (_, indices, _): (usize, alloc::vec::Vec<Expr>, usize),
) -> AssignTarget
{
    AssignTarget::ArrayIndex(name, indices)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action50<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::UnaryOp(UnaryOp::Neg, Box::new(e))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action51<
>(
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::UnaryOp(UnaryOp::Not, Box::new(e))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action52<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action53<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Mul, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action54<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::HighMul, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action55<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Div, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action56<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Mod, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action57<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action58<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Add, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action59<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Sub, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action60<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action61<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Lt, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action62<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Le, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action63<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Gt, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action64<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Ge, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action65<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action66<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Eq, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action67<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Ne, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action68<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action69<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::And, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action70<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action71<
>(
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp(BinOp::Or, Box::new(l), Box::new(r))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action72<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action73<
>(
    (_, arr, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
    (_, idx, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    Expr::Index(Box::new(arr), Box::new(idx))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action74<
>(
    (_, func, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    Expr::FuncCall(func, args)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action75<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    Expr::Length(Box::new(e))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action76<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action77<
>(
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::IntLit(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action78<
>(
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::CharLit(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action79<
>(
    (_, __0, _): (usize, String, usize),
) -> Expr
{
    Expr::StringLit(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action80<
>(
    (_, __0, _): (usize, Token, usize),
) -> Expr
{
    Expr::BoolLit(true)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action81<
>(
    (_, __0, _): (usize, Token, usize),
) -> Expr
{
    Expr::BoolLit(false)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action82<
>(
    (_, __0, _): (usize, String, usize),
) -> Expr
{
    Expr::Var(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action83<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action84<
>(
    (_, _, _): (usize, Token, usize),
    (_, elems, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    Expr::ArrayConstructor(elems)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action85<
>(
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::IntLit(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action86<
>(
    (_, _, _): (usize, Token, usize),
    (_, n, _): (usize, i64, usize),
) -> Expr
{
    Expr::IntLit(-n)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action87<
>(
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::CharLit(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action88<
>(
    (_, __0, _): (usize, Token, usize),
) -> Expr
{
    Expr::BoolLit(true)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action89<
>(
    (_, __0, _): (usize, Token, usize),
) -> Expr
{
    Expr::BoolLit(false)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action90<
>(
    (_, decls, _): (usize, alloc::vec::Vec<InterfaceDecl>, usize),
) -> Interface
{
    Interface { decls }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action91<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, params, _): (usize, Vec<Param>, usize),
    (_, _, _): (usize, Token, usize),
    (_, ret, _): (usize, Option<Vec<Type>>, usize),
    (_, _, _): (usize, Option<Token>, usize),
) -> InterfaceDecl
{
    InterfaceDecl { name, params, returns: ret.unwrap_or_default() }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action92<
>(
    (_, __0, _): (usize, InterfaceDecl, usize),
) -> alloc::vec::Vec<InterfaceDecl>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action93<
>(
    (_, v, _): (usize, alloc::vec::Vec<InterfaceDecl>, usize),
    (_, e, _): (usize, InterfaceDecl, usize),
) -> alloc::vec::Vec<InterfaceDecl>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action94<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(Token, Token)>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action95<
>(
    (_, v, _): (usize, alloc::vec::Vec<(Token, Token)>, usize),
) -> alloc::vec::Vec<(Token, Token)>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action96<
>(
    (_, __0, _): (usize, Token, usize),
    (_, __1, _): (usize, Token, usize),
) -> (Token, Token)
{
    (__0, __1)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action97<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Option<Expr>>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action98<
>(
    (_, v, _): (usize, alloc::vec::Vec<Option<Expr>>, usize),
) -> alloc::vec::Vec<Option<Expr>>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action99<
>(
    (_, __0, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action100<
>(
    (_, v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action101<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action102<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Option<Expr>, usize),
) -> Vec<Expr>
{
    match e {
        Some(e) => { v.push(e); v },
        None => v,
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action103<
>(
    (_, __0, _): (usize, Vec<Expr>, usize),
) -> Option<Vec<Expr>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action104<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Vec<Expr>>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action105<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action106<
>(
    (_, v, _): (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action107<
>(
    (_, __0, _): (usize, Stmt, usize),
    (_, __1, _): (usize, Option<Token>, usize),
) -> (Stmt, Option<Token>)
{
    (__0, __1)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action108<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> Vec<Expr>
{
    { v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action109<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<AssignTarget>, usize),
    (_, e, _): (usize, AssignTarget, usize),
) -> Vec<AssignTarget>
{
    { v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action110<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> Vec<Type>
{
    { v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action111<
>(
    (_, __0, _): (usize, Vec<Type>, usize),
) -> Option<Vec<Type>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action112<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Vec<Type>>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action113<
>(
    (_, mut v, _): (usize, alloc::vec::Vec<Param>, usize),
    (_, e, _): (usize, Option<Param>, usize),
) -> Vec<Param>
{
    match e {
        Some(e) => { v.push(e); v },
        None => v,
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action114<
>(
    (_, __0, _): (usize, Token, usize),
) -> Option<Token>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action115<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Token>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action116<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<TopLevelItem>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action117<
>(
    (_, v, _): (usize, alloc::vec::Vec<TopLevelItem>, usize),
) -> alloc::vec::Vec<TopLevelItem>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action118<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action119<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action120<
>(
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action121<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action122<
>(
    (_, __0, _): (usize, TopLevelItem, usize),
) -> alloc::vec::Vec<TopLevelItem>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action123<
>(
    (_, v, _): (usize, alloc::vec::Vec<TopLevelItem>, usize),
    (_, e, _): (usize, TopLevelItem, usize),
) -> alloc::vec::Vec<TopLevelItem>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action124<
>(
    (_, __0, _): (usize, Param, usize),
) -> Option<Param>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action125<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Param>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action126<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Param>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action127<
>(
    (_, v, _): (usize, alloc::vec::Vec<Param>, usize),
) -> alloc::vec::Vec<Param>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action128<
>(
    (_, __0, _): (usize, Param, usize),
    (_, _, _): (usize, Token, usize),
) -> Param
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action129<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Type>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action130<
>(
    (_, v, _): (usize, alloc::vec::Vec<Type>, usize),
) -> alloc::vec::Vec<Type>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action131<
>(
    (_, __0, _): (usize, Type, usize),
    (_, _, _): (usize, Token, usize),
) -> Type
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action132<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<AssignTarget>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action133<
>(
    (_, v, _): (usize, alloc::vec::Vec<AssignTarget>, usize),
) -> alloc::vec::Vec<AssignTarget>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action134<
>(
    (_, __0, _): (usize, AssignTarget, usize),
    (_, _, _): (usize, Token, usize),
) -> AssignTarget
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action135<
>(
    (_, __0, _): (usize, (Stmt, Option<Token>), usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action136<
>(
    (_, v, _): (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    (_, e, _): (usize, (Stmt, Option<Token>), usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action137<
>(
    (_, __0, _): (usize, Expr, usize),
) -> Option<Expr>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action138<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Expr>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action139<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action140<
>(
    (_, v, _): (usize, alloc::vec::Vec<Expr>, usize),
) -> alloc::vec::Vec<Expr>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action141<
>(
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, Token, usize),
) -> Expr
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action142<
>(
    (_, __0, _): (usize, Option<Expr>, usize),
) -> alloc::vec::Vec<Option<Expr>>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action143<
>(
    (_, v, _): (usize, alloc::vec::Vec<Option<Expr>>, usize),
    (_, e, _): (usize, Option<Expr>, usize),
) -> alloc::vec::Vec<Option<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action144<
>(
    (_, __0, _): (usize, (Token, Token), usize),
) -> alloc::vec::Vec<(Token, Token)>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action145<
>(
    (_, v, _): (usize, alloc::vec::Vec<(Token, Token)>, usize),
    (_, e, _): (usize, (Token, Token), usize),
) -> alloc::vec::Vec<(Token, Token)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action146<
>(
    (_, __0, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action147<
>(
    (_, v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action148<
>(
    (_, __0, _): (usize, AssignTarget, usize),
) -> alloc::vec::Vec<AssignTarget>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action149<
>(
    (_, v, _): (usize, alloc::vec::Vec<AssignTarget>, usize),
    (_, e, _): (usize, AssignTarget, usize),
) -> alloc::vec::Vec<AssignTarget>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action150<
>(
    (_, __0, _): (usize, Type, usize),
) -> alloc::vec::Vec<Type>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action151<
>(
    (_, v, _): (usize, alloc::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> alloc::vec::Vec<Type>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action152<
>(
    (_, __0, _): (usize, Param, usize),
) -> alloc::vec::Vec<Param>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action153<
>(
    (_, v, _): (usize, alloc::vec::Vec<Param>, usize),
    (_, e, _): (usize, Param, usize),
) -> alloc::vec::Vec<Param>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action154<
>(
    __0: (usize, Stmt, usize),
    __1: (usize, Token, usize),
) -> (Stmt, Option<Token>)
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action114(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action155<
>(
    __0: (usize, Stmt, usize),
) -> (Stmt, Option<Token>)
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action156<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Type, usize),
    __3: (usize, Token, usize),
) -> GlobalDecl
{
    let __start0 = __3.0;
    let __end0 = __3.2;
    let __temp0 = __action114(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action157<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Type, usize),
) -> GlobalDecl
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action158<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Type, usize),
    __3: (usize, Token, usize),
    __4: (usize, Expr, usize),
    __5: (usize, Token, usize),
) -> GlobalDecl
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action114(
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action159<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Type, usize),
    __3: (usize, Token, usize),
    __4: (usize, Expr, usize),
) -> GlobalDecl
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action160<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Option<Vec<Type>>, usize),
    __5: (usize, Token, usize),
) -> InterfaceDecl
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action114(
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action161<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Option<Vec<Type>>, usize),
) -> InterfaceDecl
{
    let __start0 = __4.2;
    let __end0 = __4.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action162<
>(
    __0: (usize, Token, usize),
    __1: (usize, Vec<Expr>, usize),
    __2: (usize, Token, usize),
) -> Vec<Expr>
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action114(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __0,
        __1,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action163<
>(
    __0: (usize, Token, usize),
    __1: (usize, Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __1.2;
    let __end0 = __1.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __0,
        __1,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action164<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
) -> String
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action114(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action165<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
) -> String
{
    let __start0 = __1.2;
    let __end0 = __1.2;
    let __temp0 = __action115(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action166<
>(
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<(Token, Token)>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action96(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action167<
>(
    __0: (usize, alloc::vec::Vec<(Token, Token)>, usize),
    __1: (usize, Token, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<(Token, Token)>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action96(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action168<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Option<Expr>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action94(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action169<
>(
    __0: (usize, alloc::vec::Vec<(Token, Token)>, usize),
) -> Vec<Option<Expr>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action95(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action170<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
) -> Vec<Option<Expr>>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action94(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action171<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, alloc::vec::Vec<(Token, Token)>, usize),
) -> Vec<Option<Expr>>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action95(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action172<
>(
    __0: (usize, Token, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __2.2;
    let __temp0 = __action101(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action173<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Token, usize),
    __2: (usize, Expr, usize),
    __3: (usize, Token, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __1.0;
    let __end0 = __3.2;
    let __temp0 = __action101(
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action174<
>(
    __0: (usize, Expr, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action141(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action175<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action141(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action147(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action176<
>(
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action139(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action177<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action140(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action178<
>(
    __0: (usize, Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action139(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action179<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action140(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action180<
>(
    __0: (usize, AssignTarget, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<AssignTarget>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action134(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action148(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action181<
>(
    __0: (usize, alloc::vec::Vec<AssignTarget>, usize),
    __1: (usize, AssignTarget, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<AssignTarget>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action134(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action149(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action182<
>(
    __0: (usize, AssignTarget, usize),
) -> Vec<AssignTarget>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action132(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action183<
>(
    __0: (usize, alloc::vec::Vec<AssignTarget>, usize),
    __1: (usize, AssignTarget, usize),
) -> Vec<AssignTarget>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action133(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action184<
>(
    __0: (usize, Param, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action128(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action152(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action185<
>(
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, Param, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<Param>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action128(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action153(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action186<
>(
    __0: (usize, Option<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action187<
>(
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, Option<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action127(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action188<
>(
    __0: (usize, Type, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<Type>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action131(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action150(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action189<
>(
    __0: (usize, alloc::vec::Vec<Type>, usize),
    __1: (usize, Type, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<Type>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action131(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action151(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action190<
>(
    __0: (usize, Type, usize),
) -> Vec<Type>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action129(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action191<
>(
    __0: (usize, alloc::vec::Vec<Type>, usize),
    __1: (usize, Type, usize),
) -> Vec<Type>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action130(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action192<
>(
    __0: (usize, Stmt, usize),
    __1: (usize, Token, usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action154(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action135(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action193<
>(
    __0: (usize, Stmt, usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action155(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action135(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action194<
>(
    __0: (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    __1: (usize, Stmt, usize),
    __2: (usize, Token, usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action154(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action136(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action195<
>(
    __0: (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    __1: (usize, Stmt, usize),
) -> alloc::vec::Vec<(Stmt, Option<Token>)>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action155(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action136(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action196<
>(
    __0: (usize, Option<Vec<Expr>>, usize),
) -> Block
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action105(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action197<
>(
    __0: (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    __1: (usize, Option<Vec<Expr>>, usize),
) -> Block
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action106(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action198<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Option<Expr>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action97(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action199<
>(
    __0: (usize, alloc::vec::Vec<Option<Expr>>, usize),
) -> Vec<Option<Expr>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action98(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action200<
>(
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action137(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action178(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action201<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expr>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action138(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action178(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action202<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action137(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action179(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action203<
>(
    __0: (usize, alloc::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action138(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action179(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action204<
>(
    __0: (usize, Param, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action124(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action186(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action205<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Param>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action186(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action206<
>(
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, Param, usize),
) -> Vec<Param>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action124(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action187(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action207<
>(
    __0: (usize, alloc::vec::Vec<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action187(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action208<
>(
    __0: (usize, Vec<Expr>, usize),
) -> Block
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action103(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action196(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action209<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Block
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action104(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action196(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action210<
>(
    __0: (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
    __1: (usize, Vec<Expr>, usize),
) -> Block
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action103(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action197(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action211<
>(
    __0: (usize, alloc::vec::Vec<(Stmt, Option<Token>)>, usize),
) -> Block
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action104(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action197(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action212<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Vec<Type>, usize),
    __5: (usize, Block, usize),
) -> FuncDef
{
    let __start0 = __4.0;
    let __end0 = __4.2;
    let __temp0 = __action111(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action213<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Block, usize),
) -> FuncDef
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action112(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action214<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Vec<Type>, usize),
    __5: (usize, Token, usize),
) -> InterfaceDecl
{
    let __start0 = __4.0;
    let __end0 = __4.2;
    let __temp0 = __action111(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action160(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action215<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Token, usize),
) -> InterfaceDecl
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action112(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action160(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action216<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Vec<Type>, usize),
) -> InterfaceDecl
{
    let __start0 = __4.0;
    let __end0 = __4.2;
    let __temp0 = __action111(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action161(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action217<
>(
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, Vec<Param>, usize),
    __3: (usize, Token, usize),
) -> InterfaceDecl
{
    let __start0 = __3.2;
    let __end0 = __3.2;
    let __temp0 = __action112(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action161(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action218<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
) -> Program
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action116(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action219<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, alloc::vec::Vec<TopLevelItem>, usize),
) -> Program
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action117(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action220<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action118(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action218(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action221<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action119(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action218(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action222<
>(
    __0: (usize, alloc::vec::Vec<TopLevelItem>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action118(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action219(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action223<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, alloc::vec::Vec<TopLevelItem>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action119(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action219(
        __temp0,
        __1,
    )
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple<>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>>;
}

impl<> __ToTriple<> for (usize, Token, usize)
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>> {
        Ok(self)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), String>
{
    fn to_triple(self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, String>> {
        self.map_err(|error| __lalrpop_util::ParseError::User { error })
    }
}
