use crate::{
    ast::scalar_value::ScalarValue,
    ql::{
        lex::{Number as LexNum, Token, lex},
        types::{QlAst, TokenMeta, WithMeta},
    },
};

pub enum ParseError {
    UnexpectedEOF,
}

pub fn parse<'a>(input: &'a str) -> Result<QlAst, ParseError> {
    do_parse(lex(input).into_iter(), 0)
}

fn do_parse<'a>(
    mut stream: impl Iterator<Item = (Token, TokenMeta)>,
    _min_binding_power: usize,
) -> Result<QlAst, ParseError> {
    use ParseError::*;
    use Token::*;

    let (lhs_token, lhs_meta) = stream.next().ok_or(UnexpectedEOF)?;

    let lhs: WithMeta<QlAst> = match lhs_token {
        True => (true.into(), lhs_meta),
        False => (false.into(), lhs_meta),
        Number(number) => match number {
            LexNum::I32(x) => (x.into(), lhs_meta),
            LexNum::I64(x) => (x.into(), lhs_meta),
            LexNum::F32(x) => (x.into(), lhs_meta),
            LexNum::F64(x) => (x.into(), lhs_meta),
        },
        Sym(x) => (QlAst::Symbol(x.into()), lhs_meta),
        SingleQuotedString(x) => (ScalarValue::Text(x.into()).into(), lhs_meta),
        Plus => todo!(),
        Minus => todo!(),
        Mul => todo!(),
        Div => todo!(),
        Pipe => todo!(),
        Eq => todo!(),
        Assign => todo!(),
        NotEq => todo!(),
        LessEq => todo!(),
        GreaterEq => todo!(),
        Less => todo!(),
        Greater => todo!(),
        DoubleQuotedString(_) => todo!(),
        LParen => todo!(),
        RParen => todo!(),
        LCurlyBrace => todo!(),
        RCurlyBrace => todo!(),
        LSquareBrace => todo!(),
        RSquareBrace => todo!(),
        Comma => todo!(),
        Whitespace(_) => todo!(),
        Unknown => todo!(),
    };

    todo!();
}

//   const [lhsToken, lhsMeta] = base;
//
//   let lhs: WithMeta<EraQlAst>;
//
//   originalLhs: switch (lhsToken.t) {
//     case 'keyword': {
//       switch (lhsToken.kw) {
//         case 'let':
//           return doParseLet([lhsToken, lhsMeta], stream);
//         default:
//           return err([[{ t: 'keyword-reserved', kw: lhsToken.kw }, lhsMeta]]);
//       }
//     }
//     case 'operator': {
//       if (lhsToken.op === '||') {
//         const expr = buildEraQLAst(stream, 0);
//         if (expr.isErr()) {
//           return err(expr.error);
//         }
//         if (expr.value === null) {
//           return err([[{ t: 'expected', expected: 'expression' }, lhsMeta]]);
//         }
//
//         const closure: Closure = {
//           t: 'closure',
//           args: [[], lhsMeta],
//           body: expr.value,
//         };
//
//         const combinedRange = combineRanges(lhsMeta.range, expr.value[1].range);
//
//         // Comments are lost here. TODO: preserve comments
//
//         lhs = [
//           closure,
//           { range: combinedRange, comments: { leading: [], trailing: null } },
//         ];
//         break originalLhs;
//       }
//       return err([[{ t: 'unexpected-token' }, lhsMeta]]);
//     }
//     case 'symbol':
//     case 'ident':
//     case 'literal': {
//       // comments propagated via lhsMeta
//       lhs = [lhsToken, lhsMeta];
//       break originalLhs;
//     }
//     case 'template': {
//       const res = doParseTemplate([lhsToken, lhsMeta], stream, {
//         op: null,
//       });
//
//       if (res.isErr()) {
//         return err(res.error);
//       }
//
//       lhs = res.value;
//       break originalLhs;
//     }
//     case 'grouping': {
//       switch (lhsToken.op) {
//         case '|': {
//           const args = doParseFunctionParameters(
//             [lhsToken.op, lhsMeta],
//             stream
//           );
//
//           if (args.isErr()) {
//             return err(args.error);
//           }
//           const expr = buildEraQLAst(stream, 0);
//
//           if (expr.isErr()) {
//             return err(expr.error);
//           }
//
//           if (expr.value === null) {
//             return err([[{ t: 'unexpected-token' }, lhsMeta]]);
//           }
//
//           const closure: Closure = {
//             t: 'closure',
//             args: args.value,
//             body: expr.value,
//           };
//
//           const combindedRange = combineRanges(
//             lhsMeta.range,
//             expr.value[1].range
//           );
//
//           lhs = [
//             closure,
//             {
//               range: combindedRange,
//               comments: { leading: [], trailing: null }, //  TODO: comments are lost here
//             },
//           ];
//           break originalLhs;
//         }
//         case '(': {
//           const child = buildEraQLAst(stream, 0);
//
//           if (child.isErr()) {
//             return err(child.error);
//           }
//
//           if (child.value === null) {
//             return err([[{ t: 'expected', expected: 'expression' }, lhsMeta]]);
//           }
//
//           const closingBrace = stream.consumeIf(
//             (t) => t.t === 'grouping' && t.op === ')'
//           );
//
//           if (closingBrace.isErr()) {
//             return err([
//               [{ t: 'unclosed-deliminator', missing: ')' }, lhsMeta],
//             ]);
//           }
//
//           lhs = child.value;
//           break originalLhs;
//         }
//         case '[': {
//           const res = doParseList([lhsToken.op, lhsMeta], stream);
//
//           if (res.isErr()) {
//             return err(res.error);
//           }
//
//           const combinedRange = combineRanges(
//             lhsMeta.range,
//             ...res.value[0].map((item) => item[1].range),
//             res.value[1].range
//           );
//
//           lhs = [
//             { t: 'list', elems: res.value[0] },
//             {
//               range: combinedRange,
//               comments: { leading: [], trailing: null }, // TODO comments lost here
//             },
//           ];
//
//           break originalLhs;
//         }
//         case '{': {
//           const kvs = doParseKvs([lhsToken.op, lhsMeta], stream);
//
//           if (kvs.isErr()) {
//             return err(kvs.error);
//           }
//
//           lhs = kvs.value;
//           break originalLhs;
//         }
//         default:
//           return err([[{ t: 'expected', expected: 'expression' }, lhsMeta]]);
//       }
//     }
//     default:
//       return err([[{ t: 'unexpected-token', token: lhsToken }, lhsMeta]]);
//   }
//
//   operatorLoop: while (!stream.isFinished()) {
//     const next = stream.peak();
//
//     if (next === null) {
//       break operatorLoop;
//     }
//
//     const pf = postfixBindingPower(next);
//
//     if (pf !== null) {
//       const [leftBindingPower, _rightBindingPower] = pf;
//
//       if (leftBindingPower < minBindingPower) {
//         break operatorLoop;
//       }
//
//       stream.consume();
//       const [nextToken, { range: nextRange, comments: nextComments }] = next;
//
//       if (
//         nextToken.t !== 'operator' &&
//         nextToken.t !== 'symbol' &&
//         nextToken.t !== 'tag'
//       ) {
//         return err([[{ t: 'cant-call' }, { range: nextRange }]]);
//       }
//
//       lhs = [
//         {
//           t: 'call',
//           op: [nextToken, { range: nextRange, comments: nextComments }],
//           args: [lhs],
//           display: 'postfix',
//         },
//         {
//           range: combineRanges(nextRange, lhs[1].range),
//           comments: { leading: [], trailing: null }, // TODO comments lost here
//         },
//       ];
//
//       continue operatorLoop;
//     }
//
//     if (
//       next[0].t === 'template' &&
//       (next[0].kind === 'start' || next[0].kind === 'complete')
//     ) {
//       stream.consume();
//
//       const template = doParseTemplate([next[0], next[1]], stream, {
//         op: lhs,
//       });
//
//       if (template.isErr()) {
//         return err(template.error);
//       }
//
//       lhs = template.value;
//       continue operatorLoop;
//     }
//
//     if (next[0].t === 'ident') {
//       stream.consume();
//
//       const [nextToken, nextMeta] = next;
//
//       lhs = [
//         {
//           t: 'attr-access',
//           in: lhs,
//           attr: [nextToken, nextMeta],
//         },
//         {
//           range: combineRanges(lhs[1].range, nextMeta.range),
//           comments: { leading: [], trailing: null }, // TODO comments lost here
//         },
//       ];
//
//       continue operatorLoop;
//     }
//
//     if (next[0].t === 'grouping' && next[0].op === '(') {
//       stream.consume();
//
//       const list = doParseList([next[0].op, next[1]], stream);
//
//       if (list.isErr()) {
//         return err(list.error);
//       }
//
//       // TODO comments lost here
//       const [argsAst, { range: argsRange, comments: _1 }] = list.value;
//
//       lhs = [
//         { t: 'call', op: lhs, args: argsAst, display: 'fc' },
//         {
//           range: combineRanges(lhs[1].range, argsRange),
//           comments: { leading: [], trailing: null }, // TODO comments lost here
//         },
//       ];
//
//       continue operatorLoop;
//     }
//
//     const inF = infixBindingPower(next);
//
//     if (inF !== null) {
//       const [leftBindingPower, rightBindingPower] = inF;
//
//       if (leftBindingPower < minBindingPower) {
//         break operatorLoop;
//       }
//
//       stream.consume();
//       const rhs = buildEraQLAst(stream, rightBindingPower);
//
//       if (rhs.isErr()) {
//         return err(rhs.error);
//       }
//
//       if (rhs.value === null) {
//         return err([[{ t: 'expected', expected: 'expression' }, next[1]]]);
//       }
//
//       const [nextToken, nextMeta] = next;
//
//       if (
//         nextToken.t !== 'operator' &&
//         nextToken.t !== 'symbol' &&
//         nextToken.t !== 'tag'
//       ) {
//         return err([[{ t: 'cant-call' }, nextMeta]]);
//       }
//
//       lhs = [
//         {
//           t: 'call',
//           args: [lhs, rhs.value],
//           op: [nextToken, nextMeta],
//           display: 'infix',
//         },
//         {
//           range: combineRanges(lhs[1].range, rhs.value[1].range),
//           comments: { leading: [], trailing: null }, // TODO comments lost here
//         },
//       ];
//
//       continue operatorLoop;
//     }
//
//     break;
//   }
//
//   return ok(lhs);
// };
//
// /**
//  * Assumes that the let keyword has already been consumed
//  */
// function doParseLet(
//   [_letToken, letTokenMeta]: WithMeta<KwToken>,
//   stream: TokenStream
// ): ParseRes<WithMeta<EraQlAst>> {
//   const { range: letTokenRange, comments: letTokenComments } = letTokenMeta;
//   const { leading: letTokenLeadingComments, trailing: _1 } = letTokenComments; // TODO preserve comments
//
//   const ident = stream.consume();
//
//   if (ident === null) {
//     return err([
//       [{ t: 'expected', expected: 'identifier' }, { range: letTokenRange }],
//     ]);
//   }
//   const [sym, { range: symRange, comments: symComments }] = ident;
//   if (sym.t !== 'symbol') {
//     return err([[{ t: 'expected', expected: 'symbol' }, { range: symRange }]]);
//   }
//
//   const eqToken = stream.consume();
//
//   if (eqToken === null) {
//     return err([[{ t: 'expected', expected: '=' }, { range: symRange }]]);
//   }
//
//   const [eq, { range: eqRange, comments: _2 }] = eqToken; //  TODO preserve comments
//
//   if (eq.t !== 'operator' || eq.op !== '=') {
//     return err([[{ t: 'expected', expected: '=' }, { range: eqRange }]]);
//   }
//
//   const bindingExprResult = buildEraQLAst(stream, 2);
//
//   if (bindingExprResult === null) {
//     return err([
//       [{ t: 'expected', expected: 'expression' }, { range: eqRange }],
//     ]);
//   }
//
//   if (bindingExprResult.isErr()) {
//     return err(bindingExprResult.error);
//   }
//
//   if (bindingExprResult.value === null) {
//     return err([
//       [{ t: 'expected', expected: 'expression' }, { range: eqRange }],
//     ]);
//   }
//   const [bindingExpr, { range: bindingExprRange, comments: bindingComments }] =
//     bindingExprResult.value;
//
//   const semicolonWithMeta = stream.consume();
//   if (semicolonWithMeta === null) {
//     return err([
//       [{ t: 'expected', expected: 'semicolon' }, { range: bindingExprRange }],
//     ]);
//   }
//   const [
//     semicolon,
//     {
//       range: semicolonRange,
//       comments: _5, // TODO preserve comments
//     },
//   ] = semicolonWithMeta;
//   if (semicolon.t !== 'operator' || semicolon.op !== ';') {
//     return err([
//       [{ t: 'expected', expected: 'semicolon' }, { range: semicolonRange }],
//     ]);
//   }
//
//   const bodyResult = buildEraQLAst(stream, 0);
//
//   if (bodyResult.isErr()) {
//     return err(bodyResult.error);
//   }
//
//   if (bodyResult.value == null) {
//     return err([
//       [{ t: 'expected', expected: 'expression' }, { range: eqRange }],
//     ]);
//   }
//   const [body, { range: bodyRange, comments: bodyComments }] = bodyResult.value;
//
//   // In theory, could also be fine to just take the combined range of symRange and bodyRange
//   const combinedRange = combineRanges(
//     symRange,
//     eqRange,
//     bindingExprRange,
//     bodyRange
//   );
//
//   const letExpr: Let = {
//     t: 'let',
//     sym: [sym, { range: symRange, comments: symComments }],
//     bindingExpr: [
//       bindingExpr,
//       { range: bindingExprRange, comments: bindingComments },
//     ],
//     body: [body, { range: bodyRange, comments: bodyComments }],
//   };
//
//   return ok([
//     letExpr,
//     {
//       range: combinedRange,
//       comments: { leading: letTokenLeadingComments, trailing: null }, // TODO preserve comments
//     },
//   ]);
// }
//
// const doParseList = (
//   start: ['[' | '(', TokenMeta],
//   stream: TokenStream
// ): ParseRes<WithMeta<WithMeta<EraQlAst>[]>> => {
//   const elements: WithMeta<EraQlAst>[] = [];
//   const [startOp, { range: startRange, comments: _1 }] = start; // TODO comments lost here
//   let fullSpan = startRange;
//
//   const opposite: T.Grouping = ({ '[': ']', '(': ')' } as const)[startOp];
//
//   const missingClosingError: WithRange<ParseError> = [
//     { t: 'unclosed-deliminator', missing: opposite },
//     { range: startRange },
//   ];
//
//   // eslint-disable-next-line no-constant-condition
//   kvLoop: while (true) {
//     const next = stream.peak();
//
//     if (next === null) {
//       return err([missingClosingError]);
//     }
//
//     // TODO comments lost here
//     const [nextToken, { range: nextRange, comments: _2 }] = next;
//
//     if (nextToken.t === 'grouping' && nextToken.op === opposite) {
//       fullSpan = combineRanges(fullSpan, nextRange);
//       stream.consume();
//       break kvLoop;
//     }
//
//     const item = buildEraQLAst(stream, 0);
//
//     if (item.isErr()) {
//       return err(item.error);
//     }
//
//     Assert.assert(item.value !== null, 'Item shouldnt be null');
//
//     elements.push(item.value);
//     fullSpan = combineRanges(fullSpan, item.value[1].range);
//
//     const afterElement = stream.peak();
//
//     if (afterElement === null) {
//       return err([missingClosingError]);
//     }
//
//     // TODO comments lost here
//     const [afterElementToken, { range: afterElementRange, comments: _3 }] =
//       afterElement;
//
//     if (afterElementToken.t === 'grouping' && afterElementToken.op === ',') {
//       fullSpan = combineRanges(fullSpan, afterElementRange);
//       stream.consume();
//       continue kvLoop;
//     } else {
//       if (
//         afterElementToken.t === 'grouping' &&
//         afterElementToken.op === opposite
//       ) {
//         fullSpan = combineRanges(fullSpan, afterElementRange);
//         stream.consume();
//         break kvLoop;
//       } else {
//         return err([
//           [{ t: 'expected', expected: ',' }, { range: afterElementRange }],
//         ]);
//       }
//     }
//   }
//
//   // TODO comments lost here
//   return ok([
//     elements,
//     { range: fullSpan, comments: { leading: [], trailing: null } },
//   ]);
// };
//
// const doParseTemplate = (
//   [start, startMeta]: WithMeta<TemplateToken>,
//   stream: TokenStream,
//   { op }: { op: WithMeta<EraQlAst> | null }
// ): ParseRes<WithMeta<EraQlAst>> => {
//   if (start.kind === 'complete') {
//     const lit: WithMeta<EraQlAst> = [T.lit(start.contents), startMeta];
//     if (op === null) {
//       return ok(lit);
//     } else {
//       const call: EraQlAst = {
//         t: 'call',
//         op,
//         args: [lit],
//         display: 'template',
//       };
//       return ok([call, startMeta]);
//     }
//   }
//
//   if (start.kind === 'continue' || start.kind === 'end') {
//     let range: TokenMeta['range'] | null = null;
//
//     if (startMeta.range !== null) {
//       const [start, _end] = startMeta.range;
//       range = [start, start + 2];
//     }
//
//     return err([[{ t: 'unexpected-token' }, { range }]]);
//   }
//
//   const args: WithMeta<EraQlAst>[] = [[T.lit(start.contents), startMeta]];
//
//   // eslint-disable-next-line no-constant-condition
//   template: while (true) {
//     const expr = buildEraQLAst(stream, 0);
//
//     if (expr.isErr()) {
//       return err(expr.error);
//     }
//
//     if (expr.value === null) {
//       return err([[{ t: 'expected', expected: 'expression' }, startMeta]]);
//     }
//
//     args.push(expr.value);
//     const next = stream.consume();
//
//     if (next === null) {
//       return err([[{ t: 'expected', expected: '}}' }, startMeta]]);
//     }
//
//     const [nextToken, nextMeta] = next;
//
//     if (
//       nextToken.t !== 'template' ||
//       nextToken.kind === 'start' ||
//       nextToken.kind === 'complete'
//     ) {
//       return err([[{ t: 'expected', expected: '}}' }, nextMeta]]);
//     }
//
//     args.push([T.lit(nextToken.contents), nextMeta]);
//
//     if (nextToken.kind === 'continue') {
//       continue template;
//     } else {
//       break template;
//     }
//   }
//
//   const res: EraQlAst = {
//     t: 'call',
//     op: op === null ? [T.sym('format'), startMeta] : op,
//     args,
//     display: 'template',
//   };
//
//   const combinedRange = combineRanges(
//     startMeta.range,
//     ...args.map((arg) => arg[1].range)
//   );
//
//   // Comments are lost here. TODO: preserve comments
//   return ok([
//     res,
//     { range: combinedRange, comments: { leading: [], trailing: null } },
//   ]);
// };
//
// const doParseKvs = (
//   // TODO comments lost here
//   [_start, { range: startRange, comments: _1 }]: WithMeta<'{'>,
//   stream: TokenStream
// ): ParseRes<WithMeta<EraQlAst>> => {
//   const assocEntries: WithMeta<AssocEntry>[] = [];
//   let fullSpan = startRange;
//
//   const missingClosingError: WithRange<ParseError> = [
//     {
//       t: 'unclosed-deliminator',
//       missing: '}',
//     },
//     { range: startRange },
//   ];
//
//   // eslint-disable-next-line no-constant-condition
//   kvLoop: while (true) {
//     const next = stream.peak();
//
//     if (next === null) {
//       return err([missingClosingError]);
//     }
//
//     // TODO comments lost here
//     const [nextToken, { range: nextRange, comments: _2 }] = next;
//
//     if (nextToken.t === 'grouping' && nextToken.op === '}') {
//       fullSpan = combineRanges(fullSpan, nextRange);
//       stream.consume();
//       break kvLoop;
//     }
//
//     const entry = doParseAssocEntry(stream);
//
//     if (entry.isErr()) {
//       return err(entry.error);
//     }
//
//     assocEntries.push(entry.value);
//
//     const afterEntry = stream.peak();
//
//     if (afterEntry === null) {
//       return err([missingClosingError]);
//     }
//
//     // TODO comments lost here
//     const [afterEntryToken, { range: afterEntryRange, comments: _3 }] =
//       afterEntry;
//
//     if (afterEntryToken.t === 'grouping' && afterEntryToken.op === ',') {
//       fullSpan = combineRanges(fullSpan, afterEntryRange);
//       stream.consume();
//       continue kvLoop;
//     } else {
//       if (afterEntryToken.t === 'grouping' && afterEntryToken.op === '}') {
//         fullSpan = combineRanges(fullSpan, afterEntryRange);
//         stream.consume();
//         break kvLoop;
//       } else {
//         return err([
//           [{ t: 'expected', expected: ',' }, { range: afterEntryRange }],
//         ]);
//       }
//     }
//   }
//
//   const combinedRange = combineRanges(startRange, fullSpan);
//
//   return ok([
//     { t: 'assoc', entries: assocEntries },
//     { range: combinedRange, comments: { leading: [], trailing: null } }, // TODO comments lost here
//   ]);
// };
//
// /**
//  *
//  * (Caller responsible for handling stream being empty
//  * because returned error needs range info.)
//  */
// const doParseAssocEntry = (
//   stream: TokenStream
// ): ParseRes<WithMeta<AssocEntry>> => {
//   const [next, nextMeta] = assertExists(stream.peak());
//
//   if (next.t === 'operator' && next.op === '...') {
//     stream.consume();
//
//     const spreadOperand = buildEraQLAst(stream, 0);
//
//     if (spreadOperand.isErr()) {
//       return err(spreadOperand.error);
//     }
//
//     if (spreadOperand.value === null) {
//       const parseError: WithRange<ParseError> = [
//         { t: 'expected', expected: 'expression' },
//         nextMeta,
//       ];
//       return err([parseError]);
//     }
//
//     const entry: AssocEntry = {
//       t: 'assoc-spread',
//       val: spreadOperand.value,
//     };
//
//     const fullRange = combineRanges(
//       nextMeta.range,
//       spreadOperand.value[1].range
//     );
//
//     // TODO comments lost here
//     return ok([
//       entry,
//       { range: fullRange, comments: { leading: [], trailing: null } },
//     ]);
//   } else {
//     return doParseKV(stream);
//   }
// };
//
// const doParseKV = (stream: TokenStream): ParseRes<WithMeta<AssocEntry>> => {
//   const key = buildEraQLAst(stream, 0);
//
//   if (key.isErr()) {
//     return err(key.error);
//   }
//
//   Assert.assert(key.value !== null, 'Key must be set here');
//
//   let sym: string;
//   const [keyToken, { range: keyRange, comments: keyComments }] = key.value;
//
//   switch (keyToken.t) {
//     case 'symbol':
//       sym = keyToken.sym;
//       break;
//     case 'literal':
//       if (
//         typeof keyToken.val === 'string' ||
//         typeof keyToken.val === 'number'
//       ) {
//         sym = keyToken.val.toString();
//       } else {
//         return err([
//           [{ t: 'expected', expected: 'struct-key' }, { range: keyRange }],
//         ]);
//       }
//       break;
//     case 'call':
//     case 'list':
//     case 'assoc':
//     case 'ident':
//     case 'closure':
//     case 'attr-access':
//     case 'let': {
//       return err([
//         [{ t: 'expected', expected: 'struct-key' }, { range: keyRange }],
//       ]);
//     }
//     default:
//       return Assert.unreachable(keyToken);
//   }
//
//   const sep = stream.consumeIf((t) => t.t === 'operator' && t.op === ':');
//
//   if (sep.isErr()) {
//     const next = stream.peak();
//     Assert.assert(next !== null);
//     return err([[{ t: 'expected', expected: [':'] }, next[1]]]);
//   }
//
//   // TODO comments lost here
//   const [_sepToken, { range: sepRange, comments: _4 }] = sep.value;
//
//   const val = buildEraQLAst(stream, 0);
//
//   if (val.isErr()) {
//     return err(val.error);
//   }
//
//   Assert.assert(val.value !== null, 'Cant be empty');
//
//   const kv: AssocEntry = {
//     t: 'assoc-kv',
//     key: [sym, { range: keyRange, comments: keyComments }],
//     val: val.value,
//   };
//
//   const combinedRange = combineRanges(keyRange, sepRange, val.value[1].range);
//
//   // TODO comments lost here
//   return ok([
//     kv,
//     { range: combinedRange, comments: { leading: [], trailing: null } },
//   ]);
// };
//
// const doParseFunctionParameters = (
//   start: WithMeta<'|'>,
//   stream: TokenStream
// ): ParseRes<WithMeta<{ readonly sym: WithMeta<string> }[]>> => {
//   const args: { readonly sym: WithMeta<string> }[] = [];
//   const [_start, { range: startRange, comments: _ }] = start; // TODO Comments lost here
//   let fullSpan = startRange;
//
//   const missingClosingBar: WithRange<ParseError> = [
//     { t: 'unclosed-deliminator', missing: '|' },
//     { range: startRange },
//   ];
//
//   // eslint-disable-next-line no-constant-condition
//   argLoop: while (true) {
//     const next = stream.peak();
//
//     if (next === null) {
//       return err([missingClosingBar]);
//     }
//
//     const [nextToken, { range: nextRange, comments: _1 }] = next; // TODO Comments lost here
//
//     if (nextToken.t === 'grouping' && nextToken.op === '|') {
//       fullSpan = combineRanges(fullSpan, nextRange);
//       stream.consume();
//       break argLoop;
//     }
//
//     const item = buildEraQLAst(stream, 0);
//
//     if (item.isErr()) {
//       return err(item.error);
//     }
//
//     Assert.assert(item.value !== null, 'Item shouldnt be null');
//
//     const [itemToken, { range: itemRange, comments: itemComments }] =
//       item.value;
//
//     if (itemToken.t === 'symbol') {
//       fullSpan = combineRanges(fullSpan, itemRange);
//       args.push({
//         sym: [itemToken.sym, { range: itemRange, comments: itemComments }],
//       });
//     } else {
//       return err([
//         [{ t: 'expected', expected: 'expression' }, { range: nextRange }],
//       ]);
//     }
//     fullSpan = combineRanges(fullSpan, itemRange);
//
//     const afterElement = stream.peak();
//
//     if (afterElement === null) {
//       return err([missingClosingBar]);
//     }
//
//     const [afterElementToken, { range: afterElementRange, comments: _3 }] =
//       afterElement; // TODO comments lost here
//
//     if (afterElementToken.t === 'grouping' && afterElementToken.op === ',') {
//       fullSpan = combineRanges(fullSpan, afterElementRange);
//       stream.consume();
//       continue argLoop;
//     } else {
//       if (afterElementToken.t === 'grouping' && afterElementToken.op === '|') {
//         fullSpan = combineRanges(fullSpan, afterElementRange);
//         stream.consume();
//         break argLoop;
//       } else {
//         return err([
//           [{ t: 'expected', expected: ',' }, { range: afterElementRange }],
//         ]);
//       }
//     }
//   }
//
//   return ok([
//     args,
//     { range: fullSpan, comments: { leading: [], trailing: null } }, // TODO: comments lost here
//   ]);
// };
//
