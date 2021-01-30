use crate::boxes::{HorizontalBox, TeXBox};
use crate::category::Category;
use crate::dimension::{Dimen, SpringDimen, Unit};
use crate::glue::Glue;
use crate::list::HorizontalListElem;
use crate::math_code::MathCode;
use crate::math_list::{
    AtomKind, MathAtom, MathField, MathList, MathListElem, MathStyle,
    MathSymbol,
};
use crate::parser::boxes::BoxLayout;
use crate::parser::Parser;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Clone)]
enum InterAtomSpacing {
    None,
    ThinSkip,
    ThinSkipNonScript,
    MediumSkipNonScript,
    ThickSkipNonScript,
}

lazy_static! {
    static ref INTER_ATOM_SPACING: HashMap<(AtomKind, AtomKind), InterAtomSpacing> = [
        // 0 1 (2) (3) 0 0 0 (1)
        ((AtomKind::Ord, AtomKind::Ord), InterAtomSpacing::None),
        ((AtomKind::Ord, AtomKind::Op), InterAtomSpacing::ThinSkip),
        ((AtomKind::Ord, AtomKind::Bin), InterAtomSpacing::MediumSkipNonScript),
        ((AtomKind::Ord, AtomKind::Rel), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Ord, AtomKind::Open), InterAtomSpacing::None),
        ((AtomKind::Ord, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Ord, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Ord, AtomKind::Inner), InterAtomSpacing::ThinSkipNonScript),

        // 1 1 * (3) 0 0 0 (1)
        ((AtomKind::Op, AtomKind::Ord), InterAtomSpacing::ThinSkip),
        ((AtomKind::Op, AtomKind::Op), InterAtomSpacing::ThinSkip),
        //((AtomKind::Op, AtomKind::Bin), InterAtomSpacing::None),
        ((AtomKind::Op, AtomKind::Rel), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Op, AtomKind::Open), InterAtomSpacing::None),
        ((AtomKind::Op, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Op, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Op, AtomKind::Inner), InterAtomSpacing::ThinSkipNonScript),

        // (2) (2) * * (2) * * (2)
        ((AtomKind::Bin, AtomKind::Ord), InterAtomSpacing::MediumSkipNonScript),
        ((AtomKind::Bin, AtomKind::Op), InterAtomSpacing::MediumSkipNonScript),
        //((AtomKind::Bin, AtomKind::Bin), InterAtomSpacing::None),
        //((AtomKind::Bin, AtomKind::Rel), InterAtomSpacing::None),
        ((AtomKind::Bin, AtomKind::Open), InterAtomSpacing::MediumSkipNonScript),
        //((AtomKind::Bin, AtomKind::Close), InterAtomSpacing::None),
        //((AtomKind::Bin, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Bin, AtomKind::Inner), InterAtomSpacing::MediumSkipNonScript),

        // (3) (3) * 0 (3) 0 0 (3)
        ((AtomKind::Rel, AtomKind::Ord), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Rel, AtomKind::Op), InterAtomSpacing::ThickSkipNonScript),
        //((AtomKind::Rel, AtomKind::Bin), InterAtomSpacing::None),
        ((AtomKind::Rel, AtomKind::Rel), InterAtomSpacing::None),
        ((AtomKind::Rel, AtomKind::Open), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Rel, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Rel, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Rel, AtomKind::Inner), InterAtomSpacing::ThickSkipNonScript),

        // 0 0 * 0 0 0 0 0
        ((AtomKind::Open, AtomKind::Ord), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Op), InterAtomSpacing::None),
        //((AtomKind::Open, AtomKind::Bin), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Rel), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Open), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Open, AtomKind::Inner), InterAtomSpacing::None),

        // 0 1 (2) (3) 0 0 0 (1)
        ((AtomKind::Close, AtomKind::Ord), InterAtomSpacing::None),
        ((AtomKind::Close, AtomKind::Op), InterAtomSpacing::ThinSkip),
        ((AtomKind::Close, AtomKind::Bin), InterAtomSpacing::MediumSkipNonScript),
        ((AtomKind::Close, AtomKind::Rel), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Close, AtomKind::Open), InterAtomSpacing::None),
        ((AtomKind::Close, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Close, AtomKind::Punct), InterAtomSpacing::None),
        ((AtomKind::Close, AtomKind::Inner), InterAtomSpacing::ThinSkipNonScript),

        // (1) (1) * (1) (1) (1) (1) (1)
        ((AtomKind::Punct, AtomKind::Ord), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Punct, AtomKind::Op), InterAtomSpacing::ThinSkipNonScript),
        //((AtomKind::Punct, AtomKind::Bin), InterAtomSpacing::None),
        ((AtomKind::Punct, AtomKind::Rel), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Punct, AtomKind::Open), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Punct, AtomKind::Close), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Punct, AtomKind::Punct), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Punct, AtomKind::Inner), InterAtomSpacing::ThinSkipNonScript),

        // (1) 1 (2) (3) (1) 0 (1) (1)
        ((AtomKind::Inner, AtomKind::Ord), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Inner, AtomKind::Op), InterAtomSpacing::ThinSkip),
        ((AtomKind::Inner, AtomKind::Bin), InterAtomSpacing::MediumSkipNonScript),
        ((AtomKind::Inner, AtomKind::Rel), InterAtomSpacing::ThickSkipNonScript),
        ((AtomKind::Inner, AtomKind::Open), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Inner, AtomKind::Close), InterAtomSpacing::None),
        ((AtomKind::Inner, AtomKind::Punct), InterAtomSpacing::ThinSkipNonScript),
        ((AtomKind::Inner, AtomKind::Inner), InterAtomSpacing::ThinSkipNonScript),
    ].iter().cloned().collect();
}

impl<'a> Parser<'a> {
    fn is_character_head(&mut self) -> bool {
        let expanded_token = self.peek_expanded_token();
        match self.replace_renamed_token(expanded_token) {
            Some(Token::Char(_, Category::Letter)) => true,
            Some(Token::Char(_, Category::Other)) => true,
            _ => false,
        }
    }

    fn parse_character_to_math_code(&mut self) -> MathCode {
        let expanded_token = self.lex_expanded_token();
        let expanded_renamed_token = self.replace_renamed_token(expanded_token);

        let ch: char = match expanded_renamed_token {
            Some(Token::Char(ch, _)) => ch,
            _ => panic!(),
        };

        self.state.get_math_code(ch)
    }

    fn is_math_character_head(&mut self) -> bool {
        let expanded_token = self.peek_expanded_token();
        if let Some(expanded_renamed_token) =
            self.replace_renamed_token(expanded_token)
        {
            self.state
                .get_math_chardef(&expanded_renamed_token)
                .is_some()
        } else {
            false
        }
    }

    fn parse_math_character_to_math_code(&mut self) -> MathCode {
        let expanded_token = self.lex_expanded_token();
        let expanded_renamed_token =
            self.replace_renamed_token(expanded_token).unwrap();

        if let Some(math_code) =
            self.state.get_math_chardef(&expanded_renamed_token)
        {
            math_code
        } else {
            panic!("Invalid math chardef token: {:?}", expanded_renamed_token);
        }
    }

    fn is_math_symbol_head(&mut self) -> bool {
        self.is_character_head() || self.is_math_character_head()
    }

    fn parse_math_symbol(&mut self) -> MathCode {
        if self.is_character_head() {
            self.parse_character_to_math_code()
        } else if self.is_math_character_head() {
            self.parse_math_character_to_math_code()
        } else {
            panic!("Unimplemented");
        }
    }

    fn parse_math_group(&mut self) -> MathList {
        let begin_group = self.lex_expanded_token();
        match begin_group {
            Some(Token::Char(_, Category::BeginGroup)) => (),
            tok => panic!("Invalid start of math group: {:?}", tok),
        }

        self.state.push_state();

        let math_list = self.parse_math_list();

        self.state.pop_state();

        let end_group = self.lex_expanded_token();
        match end_group {
            Some(Token::Char(_, Category::EndGroup)) => (),
            tok => panic!("Math group didn't end with an EndGroup: {:?}", tok),
        }

        math_list
    }

    fn parse_math_field(&mut self) -> MathField {
        self.parse_filler_expanded();

        if self.is_math_symbol_head() {
            let math_code = self.parse_math_symbol();

            MathField::Symbol(MathSymbol::from_math_code(&math_code))
        } else {
            MathField::MathList(self.parse_math_group())
        }
    }

    fn is_math_superscript_head(&mut self) -> bool {
        let expanded_token = self.peek_expanded_token();
        match self.replace_renamed_token(expanded_token) {
            Some(Token::Char(_, Category::Superscript)) => true,
            _ => false,
        }
    }

    fn parse_math_superscript(&mut self, atom: MathAtom) -> MathAtom {
        self.lex_expanded_token();

        if atom.has_superscript() {
            panic!("Double superscript");
        }

        let superscript = self.parse_math_field();
        atom.with_superscript(superscript)
    }

    fn is_math_subscript_head(&mut self) -> bool {
        let expanded_token = self.peek_expanded_token();
        match self.replace_renamed_token(expanded_token) {
            Some(Token::Char(_, Category::Subscript)) => true,
            _ => false,
        }
    }

    fn parse_math_subscript(&mut self, atom: MathAtom) -> MathAtom {
        self.lex_expanded_token();

        if atom.has_subscript() {
            panic!("Double subscript");
        }

        let subscript = self.parse_math_field();
        atom.with_subscript(subscript)
    }

    fn is_style_change_head(&mut self) -> bool {
        self.is_next_expanded_token_in_set_of_primitives(&[
            "displaystyle",
            "textstyle",
            "scriptstyle",
            "scriptscriptstyle",
        ])
    }

    fn parse_style_change(&mut self) -> MathStyle {
        let tok = self.lex_expanded_token().unwrap();

        if self.state.is_token_equal_to_prim(&tok, "displaystyle") {
            MathStyle::DisplayStyle
        } else if self.state.is_token_equal_to_prim(&tok, "textstyle") {
            MathStyle::TextStyle
        } else if self.state.is_token_equal_to_prim(&tok, "scriptstyle") {
            MathStyle::ScriptStyle
        } else if self.state.is_token_equal_to_prim(&tok, "scriptscriptstyle") {
            MathStyle::ScriptScriptStyle
        } else {
            panic!("Invalid style change");
        }
    }

    pub fn parse_math_list(&mut self) -> MathList {
        let mut current_list = Vec::new();

        loop {
            if self.is_math_symbol_head() {
                let math_code = self.parse_math_symbol();

                current_list.push(MathListElem::Atom(
                    MathAtom::from_math_code(&math_code),
                ));
            } else if self.is_math_superscript_head()
                || self.is_math_subscript_head()
            {
                let is_superscript = self.is_math_superscript_head();

                let last_atom = match current_list.pop() {
                    Some(MathListElem::Atom(atom)) => atom,
                    Some(other_elem) => {
                        current_list.push(other_elem);
                        MathAtom::empty_ord()
                    }
                    None => MathAtom::empty_ord(),
                };

                current_list.push(MathListElem::Atom(if is_superscript {
                    self.parse_math_superscript(last_atom)
                } else {
                    self.parse_math_subscript(last_atom)
                }));
            } else if self.is_assignment_head() {
                self.parse_assignment();
            } else if self.is_style_change_head() {
                let style_change = self.parse_style_change();
                current_list.push(MathListElem::StyleChange(style_change));
            } else {
                match self.peek_expanded_token() {
                    Some(Token::Char(_, Category::EndGroup)) => break,
                    Some(Token::Char(_, Category::MathShift)) => break,
                    None => break,
                    _ => panic!("unimplemented"),
                }
            }
        }

        current_list
    }

    fn get_skip_for_atom_pair(
        &mut self,
        left_type: &AtomKind,
        right_type: &AtomKind,
        style: &MathStyle,
    ) -> Option<Glue> {
        // TODO: These should come from the state variables \thinmuskip,
        // \mediummuskip, and \thickmuskip.
        // TODO: These should be MuGlue, not plain Glue
        let thinskip = Glue {
            space: Dimen::from_unit(3.0, Unit::Point),
            stretch: SpringDimen::Dimen(Dimen::zero()),
            shrink: SpringDimen::Dimen(Dimen::zero()),
        };
        let mediumskip = Glue {
            space: Dimen::from_unit(4.0, Unit::Point),
            stretch: SpringDimen::Dimen(Dimen::from_unit(2.0, Unit::Point)),
            shrink: SpringDimen::Dimen(Dimen::from_unit(4.0, Unit::Point)),
        };
        let thickskip = Glue {
            space: Dimen::from_unit(5.0, Unit::Point),
            stretch: SpringDimen::Dimen(Dimen::from_unit(5.0, Unit::Point)),
            shrink: SpringDimen::Dimen(Dimen::zero()),
        };

        if let Some(space) = INTER_ATOM_SPACING.get(&(*left_type, *right_type))
        {
            match (space, style.is_script()) {
                (InterAtomSpacing::None, _) => None,
                (InterAtomSpacing::ThinSkip, _) => Some(thinskip),
                (InterAtomSpacing::ThinSkipNonScript, false) => Some(thinskip),
                (InterAtomSpacing::ThinSkipNonScript, true) => None,
                (InterAtomSpacing::MediumSkipNonScript, false) => {
                    Some(mediumskip)
                }
                (InterAtomSpacing::MediumSkipNonScript, true) => None,
                (InterAtomSpacing::ThickSkipNonScript, false) => {
                    Some(thickskip)
                }
                (InterAtomSpacing::ThickSkipNonScript, true) => None,
            }
        } else {
            panic!("Invalid atom type pair: {:?}/{:?}", left_type, right_type);
        }
    }

    pub fn convert_math_list_to_horizontal_list(
        &mut self,
        list: MathList,
        start_style: MathStyle,
    ) -> Vec<HorizontalListElem> {
        let mut elems_after_first_pass: MathList = Vec::new();
        let mut current_style = start_style.clone();

        for elem in list {
            match elem {
                MathListElem::Atom(mut atom) => {
                    match atom.nucleus {
                        Some(MathField::Symbol(symbol)) => {
                            let char_elem = HorizontalListElem::Char {
                                chr: symbol.position_number as char,
                                // TODO figure out what goes here
                                font: self.state.get_current_font(),
                            };

                            let hbox = self
                                .add_to_natural_layout_horizontal_box(
                                    HorizontalBox::empty(),
                                    char_elem,
                                );

                            atom.nucleus = Some(MathField::TeXBox(
                                TeXBox::HorizontalBox(hbox),
                            ));
                        }
                        Some(MathField::TeXBox(_)) => {
                            // Nothing to do
                        }
                        Some(MathField::MathList(list)) => {
                            let hlist = self
                                .convert_math_list_to_horizontal_list(
                                    list,
                                    current_style.clone(),
                                );
                            let hbox = self.combine_horizontal_list_into_horizontal_box_with_layout(hlist, &BoxLayout::Natural);

                            atom.nucleus = Some(MathField::TeXBox(
                                TeXBox::HorizontalBox(hbox),
                            ));
                        }
                        None => {}
                    }

                    if atom.has_subscript() || atom.has_superscript() {
                        panic!("Unimplemented superscript/subscript");
                    }

                    elems_after_first_pass.push(MathListElem::Atom(atom));
                }
                MathListElem::StyleChange(new_style) => {
                    current_style = new_style.clone();
                    elems_after_first_pass
                        .push(MathListElem::StyleChange(new_style));
                }
                _ => {
                    panic!("unimplemented math list elem: {:?}", elem);
                }
            }
        }

        let mut resulting_horizontal_list: Vec<HorizontalListElem> = Vec::new();
        let mut maybe_last_atom_kind: Option<AtomKind> = None;
        let mut current_style = start_style.clone();

        for elem in elems_after_first_pass {
            match elem {
                MathListElem::Atom(atom) => {
                    if let Some(last_atom_kind) = maybe_last_atom_kind {
                        if let Some(skip) = self.get_skip_for_atom_pair(
                            &last_atom_kind,
                            &atom.kind,
                            &current_style,
                        ) {
                            resulting_horizontal_list
                                .push(HorizontalListElem::HSkip(skip));
                        }
                    }

                    if atom.has_subscript() || atom.has_superscript() {
                        panic!("Atoms should be sub/superscript free in second pass!");
                    }

                    match atom.nucleus {
                        Some(MathField::TeXBox(texbox)) => {
                            resulting_horizontal_list
                                .push(HorizontalListElem::Box(texbox));
                        }
                        None => {}
                        _ => {
                            panic!("Atom nucleuses should only be boxes in second pass!");
                        }
                    }

                    maybe_last_atom_kind = Some(atom.kind);
                }
                MathListElem::StyleChange(new_style) => {
                    current_style = new_style;
                }
                _ => {
                    panic!("unimplemented math list elem: {:?}");
                }
            }
        }

        resulting_horizontal_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::with_parser;

    #[test]
    fn it_parses_math_symbols() {
        with_parser(&["a2*%"], |parser| {
            assert_eq!(
                parser.parse_math_symbol(),
                MathCode::from_number(0x7161)
            );
            assert_eq!(
                parser.parse_math_symbol(),
                MathCode::from_number(0x7032)
            );
            assert_eq!(
                parser.parse_math_symbol(),
                MathCode::from_number(0x002a)
            );
        });
    }

    #[test]
    fn it_parses_math_symbols_from_chardefs() {
        with_parser(&[r"\let\x=z%", r"\x%"], |parser| {
            parser.parse_assignment();

            assert_eq!(
                parser.parse_math_symbol(),
                MathCode::from_number(0x717a)
            );
        });
    }

    #[test]
    fn it_parses_basic_atoms_in_math_lists() {
        with_parser(&[r"a*%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::Atom(MathAtom::from_math_code(
                        &MathCode::from_number(0x7161)
                    )),
                    MathListElem::Atom(MathAtom::from_math_code(
                        &MathCode::from_number(0x002a)
                    )),
                ]
            );
        });
    }

    #[test]
    fn it_parses_basic_math_groups() {
        with_parser(&[r"{a}%"], |parser| {
            assert_eq!(
                parser.parse_math_group(),
                vec![MathListElem::Atom(MathAtom::from_math_code(
                    &MathCode::from_number(0x7161)
                )),],
            );
        });
    }

    #[test]
    #[should_panic(expected = "Invalid start of math group")]
    fn it_fails_parsing_math_groups_not_starting_with_begin_group() {
        with_parser(&[r"a%"], |parser| {
            parser.parse_math_group();
        });
    }

    #[test]
    #[should_panic(expected = "Math group didn't end with an EndGroup")]
    fn it_fails_parsing_math_groups_not_ending_with_end_group() {
        with_parser(&[r"{a%"], |parser| {
            parser.parse_math_group();
        });
    }

    #[test]
    fn it_scopes_assignments_in_math_fields() {
        with_parser(&[r"\count 0=1%", r"a^{\count 0=2}%"], |parser| {
            parser.parse_math_list();

            assert_eq!(parser.state.get_count(0), 1);
        });
    }

    #[test]
    fn it_parses_symbols_as_math_fields() {
        with_parser(&[r"a2%"], |parser| {
            assert_eq!(
                parser.parse_math_field(),
                MathField::Symbol(MathSymbol::from_math_code(
                    &MathCode::from_number(0x7161)
                ))
            );
            assert_eq!(
                parser.parse_math_field(),
                MathField::Symbol(MathSymbol::from_math_code(
                    &MathCode::from_number(0x7032)
                ))
            );
        })
    }

    #[test]
    fn it_parses_groups_as_math_fields() {
        with_parser(&[r"{ab}{}%"], |parser| {
            assert_eq!(
                parser.parse_math_field(),
                MathField::MathList(vec![
                    MathListElem::Atom(MathAtom::from_math_code(
                        &MathCode::from_number(0x7161)
                    )),
                    MathListElem::Atom(MathAtom::from_math_code(
                        &MathCode::from_number(0x7162)
                    )),
                ],)
            );
            assert_eq!(parser.parse_math_field(), MathField::MathList(vec![],));
        });
    }

    #[test]
    fn it_ignores_filler_before_math_fields() {
        with_parser(&[r"  a   {a}%"], |parser| {
            assert_eq!(
                parser.parse_math_field(),
                MathField::Symbol(MathSymbol::from_math_code(
                    &MathCode::from_number(0x7161)
                ))
            );
            assert_eq!(
                parser.parse_math_field(),
                MathField::MathList(vec![MathListElem::Atom(
                    MathAtom::from_math_code(&MathCode::from_number(0x7161))
                ),],)
            );
        });
    }

    #[test]
    fn it_parses_superscripts() {
        let a_code = MathCode::from_number(0x7161);
        let b_code = MathCode::from_number(0x7162);

        with_parser(&[r"a^a%", r"a^{ab}%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::Atom(
                        MathAtom::from_math_code(&a_code).with_superscript(
                            MathField::Symbol(MathSymbol::from_math_code(
                                &a_code
                            ))
                        )
                    ),
                    MathListElem::Atom(
                        MathAtom::from_math_code(&a_code).with_superscript(
                            MathField::MathList(vec![
                                MathListElem::Atom(MathAtom::from_math_code(
                                    &a_code
                                )),
                                MathListElem::Atom(MathAtom::from_math_code(
                                    &b_code
                                )),
                            ])
                        )
                    )
                ],
            );
        });
    }

    #[test]
    fn it_parses_superscripts_at_beginning_of_lists() {
        let a_code = MathCode::from_number(0x7161);

        with_parser(&[r"^a%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![MathListElem::Atom(
                    MathAtom::empty_ord().with_superscript(MathField::Symbol(
                        MathSymbol::from_math_code(&a_code)
                    ))
                ),],
            );
        });
    }

    #[test]
    #[should_panic(expected = "Double superscript")]
    fn it_fails_on_multiple_superscripts() {
        with_parser(&[r"a^a^a%"], |parser| {
            parser.parse_math_list();
        });
    }

    #[test]
    #[should_panic(expected = "Double superscript")]
    fn it_fails_on_multiple_superscripts_after_subscript() {
        with_parser(&[r"a^a_a^a%"], |parser| {
            parser.parse_math_list();
        });
    }

    #[test]
    fn it_parses_subscripts() {
        let a_code = MathCode::from_number(0x7161);
        let b_code = MathCode::from_number(0x7162);

        with_parser(&[r"a_a%", r"a_{ab}%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::Atom(
                        MathAtom::from_math_code(&a_code).with_subscript(
                            MathField::Symbol(MathSymbol::from_math_code(
                                &a_code
                            ))
                        )
                    ),
                    MathListElem::Atom(
                        MathAtom::from_math_code(&a_code).with_subscript(
                            MathField::MathList(vec![
                                MathListElem::Atom(MathAtom::from_math_code(
                                    &a_code
                                )),
                                MathListElem::Atom(MathAtom::from_math_code(
                                    &b_code
                                )),
                            ])
                        )
                    )
                ],
            );
        });
    }

    #[test]
    fn it_parses_subscripts_at_beginning_of_lists() {
        let a_code = MathCode::from_number(0x7161);

        with_parser(&[r"_a%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![MathListElem::Atom(MathAtom::empty_ord().with_subscript(
                    MathField::Symbol(MathSymbol::from_math_code(&a_code))
                )),],
            );
        });
    }

    #[test]
    #[should_panic(expected = "Double subscript")]
    fn it_fails_on_multiple_subscripts() {
        with_parser(&[r"a_a_a%"], |parser| {
            parser.parse_math_list();
        });
    }

    #[test]
    #[should_panic(expected = "Double subscript")]
    fn it_fails_on_multiple_subscripts_after_superscript() {
        with_parser(&[r"a_a^a_a%"], |parser| {
            parser.parse_math_list();
        });
    }

    #[test]
    fn it_parses_mathchardefs() {
        let a_code = MathCode::from_number(0x7161);
        let b_code = MathCode::from_number(0x7162);
        let c_code = MathCode::from_number(0x7163);

        with_parser(&[r"\hello%", r"a\hello b%"], |parser| {
            let tok = parser.lex_unexpanded_token().unwrap();
            parser.state.set_math_chardef(false, &tok, &c_code);

            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::Atom(MathAtom::from_math_code(&a_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&c_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&b_code)),
                ],
            );
        });
    }

    #[test]
    fn it_parses_assignments_in_math_mode() {
        let a_code = MathCode::from_number(0x7161);
        let b_code = MathCode::from_number(0x7162);
        let c_code = MathCode::from_number(0x7163);

        with_parser(&[r"a\def\x #1{a#1b}%", r"b\x c%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::Atom(MathAtom::from_math_code(&a_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&b_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&a_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&c_code)),
                    MathListElem::Atom(MathAtom::from_math_code(&b_code)),
                ]
            );
        });

        with_parser(&[r"a\def\x{b}_\x%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![MathListElem::Atom(
                    MathAtom::from_math_code(&a_code).with_subscript(
                        MathField::Symbol(MathSymbol::from_math_code(&b_code))
                    )
                ),]
            );
        });
    }

    #[test]
    fn it_parses_style_changes() {
        with_parser(
            &[r"\displaystyle \textstyle \scriptstyle \scriptscriptstyle%"],
            |parser| {
                assert_eq!(
                    parser.parse_math_list(),
                    vec![
                        MathListElem::StyleChange(MathStyle::DisplayStyle),
                        MathListElem::StyleChange(MathStyle::TextStyle),
                        MathListElem::StyleChange(MathStyle::ScriptStyle),
                        MathListElem::StyleChange(MathStyle::ScriptScriptStyle),
                    ]
                );
            },
        );
    }

    #[test]
    fn it_parses_superscripts_after_non_atoms() {
        let a_code = MathCode::from_number(0x7161);

        with_parser(&[r"\displaystyle ^a%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![
                    MathListElem::StyleChange(MathStyle::DisplayStyle),
                    MathListElem::Atom(MathAtom::empty_ord().with_superscript(
                        MathField::Symbol(MathSymbol::from_math_code(&a_code))
                    )),
                ],
            );
        });
    }

    #[test]
    fn it_ends_on_math_shifts() {
        let a_code = MathCode::from_number(0x7161);

        with_parser(&[r"a$%"], |parser| {
            assert_eq!(
                parser.parse_math_list(),
                vec![MathListElem::Atom(MathAtom::from_math_code(&a_code)),]
            );

            assert_eq!(
                parser.lex_expanded_token(),
                Some(Token::Char('$', Category::MathShift))
            );
        });
    }

    #[test]
    fn it_produces_empty_horizontal_lists_from_empty_math_lists() {
        with_parser(&[r"%"], |parser| {
            let math_list = parser.parse_math_list();
            assert_eq!(
                parser.convert_math_list_to_horizontal_list(
                    math_list,
                    MathStyle::TextStyle
                ),
                vec![]
            );
        });
    }

    #[test]
    fn it_produces_single_characters_from_single_atom_math_lists() {
        with_parser(&[r"\hbox{a}a%"], |parser| {
            let hbox = parser.parse_box().unwrap();
            let math_list = parser.parse_math_list();
            assert_eq!(
                parser.convert_math_list_to_horizontal_list(
                    math_list,
                    MathStyle::TextStyle
                ),
                vec![HorizontalListElem::Box(hbox)]
            );
        });
    }

    #[test]
    fn it_produces_multiple_characters_from_multiple_ord_math_lists() {
        with_parser(&[r"\hbox{a}\hbox{b}ab%"], |parser| {
            let hbox_a = parser.parse_box().unwrap();
            let hbox_b = parser.parse_box().unwrap();
            let math_list = parser.parse_math_list();
            assert_eq!(
                parser.convert_math_list_to_horizontal_list(
                    math_list,
                    MathStyle::TextStyle
                ),
                vec![
                    HorizontalListElem::Box(hbox_a),
                    HorizontalListElem::Box(hbox_b)
                ]
            );
        });
    }

    #[test]
    fn it_adds_space_between_atoms_of_different_types_in_math_lists() {
        // o = ord
        // p = op
        // b = bin
        // r = rel
        // n = open
        // c = close
        // t = punct
        with_parser(
            &[
                r"\hbox{%",
                r"\def\,{\hskip 3pt}%",
                r"\def\>{\hskip 4pt plus 2pt minus 4pt}%",
                r"\def\;{\hskip 5pt plus 5pt}%",
                r"\def\o{\hbox{o}}%",
                r"\def\p{\hbox{p}}%",
                r"\def\b{\hbox{b}}%",
                r"\def\r{\hbox{r}}%",
                r"\def\n{\hbox{n}}%",
                r"\def\c{\hbox{c}}%",
                r"\def\t{\hbox{t}}%",
                r"\o\o\,\p\,\o\>\b\>\o\;\r\;\o\n\o\c\o\t\,\o\,%",
                r"\p\,\p\;\r\;\p\n\p\c\,\p\t\,\p\,\o\>%",
                r"\b\>\n\o\>\b\>\o\;%",
                r"\r\r\;\n\r\c\;\r\t\,\r\;%",
                r"\n\n\c\n\t\,\n%",
                r"\c\c\t\,\c%",
                r"\t\,\t%",
                r"}%",
                r#"\mathcode`o="016F%"#,
                r#"\mathcode`p="1170%"#,
                r#"\mathcode`b="2162%"#,
                r#"\mathcode`r="3172%"#,
                r#"\mathcode`n="416E%"#,
                r#"\mathcode`c="5163%"#,
                r#"\mathcode`t="6174%"#,
                r"oopoboronocoto%",
                r"pprpnpcptpo%",
                r"bnobo%",
                r"rrnrcrtr%",
                r"nncntn%",
                r"cctc%",
                r"tt%",
            ],
            |parser| {
                let parsed_box = parser.parse_box().unwrap();
                let hlist = if let TeXBox::HorizontalBox(hbox) = parsed_box {
                    hbox.list
                } else {
                    panic!("Invalid parsed box: {:?}", parsed_box);
                };

                let math_list = parser.parse_math_list();
                assert_eq!(
                    parser.convert_math_list_to_horizontal_list(
                        math_list,
                        MathStyle::TextStyle
                    ),
                    hlist
                );
            },
        );
    }

    #[test]
    fn it_does_not_add_some_inter_atom_space_in_script_styles() {
        // o = ord
        // b = bin
        // r = rel
        // p = punct
        with_parser(
            &[
                r"\hbox{%",
                r"\def\,{\hskip 3pt}%",
                r"\def\>{\hskip 4pt plus 2pt minus 4pt}%",
                r"\def\;{\hskip 5pt plus 5pt}%",
                r"\def\o{\hbox{o}}%",
                r"\def\b{\hbox{b}}%",
                r"\def\r{\hbox{r}}%",
                r"\def\p{\hbox{p}}%",
                r"\o\;\r\p\,\o\>\b\>%",
                r"\o\;\r\p\,\o\>\b%",
                r"\o\r\p\o\b%",
                r"\o\r\p\o\b%",
                r"}%",
                r#"\mathcode`o="016F%"#,
                r#"\mathcode`b="2162%"#,
                r#"\mathcode`r="3172%"#,
                r#"\mathcode`p="6170%"#,
                r"\displaystyle orpob%",
                r"\textstyle orpob%",
                r"\scriptstyle orpob%",
                r"\scriptscriptstyle orpob%",
            ],
            |parser| {
                let parsed_box = parser.parse_box().unwrap();
                let hlist = if let TeXBox::HorizontalBox(hbox) = parsed_box {
                    hbox.list
                } else {
                    panic!("Invalid parsed box: {:?}", parsed_box);
                };

                let math_list = parser.parse_math_list();
                assert_eq!(
                    parser.convert_math_list_to_horizontal_list(
                        math_list,
                        MathStyle::TextStyle
                    ),
                    hlist
                );
            },
        );
    }
}
