// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use middle::ty;
use middle::typeck::check::FnCtxt;
use middle::typeck::infer;
use middle::typeck::infer::resolve_type;
use middle::typeck::infer::resolve::try_resolve_tvar_shallow;

use std::result::{Err, Ok};
use std::result;
use syntax::ast;
use syntax::codemap::Span;
use util::ppaux::Repr;

// Requires that the two types unify, and prints an error message if they
// don't.
pub fn suptype(fcx: &FnCtxt, sp: Span, expected: ty::t, actual: ty::t) {
    suptype_with_fn(fcx, sp, false, expected, actual,
        |sp, e, a, s| { fcx.report_mismatched_types(sp, e, a, s) })
}

pub fn subtype(fcx: &FnCtxt, sp: Span, expected: ty::t, actual: ty::t) {
    suptype_with_fn(fcx, sp, true, actual, expected,
        |sp, a, e, s| { fcx.report_mismatched_types(sp, e, a, s) })
}

pub fn suptype_with_fn(fcx: &FnCtxt,
                       sp: Span,
                       b_is_expected: bool,
                       ty_a: ty::t,
                       ty_b: ty::t,
                       handle_err: |Span, ty::t, ty::t, &ty::type_err|) {
    // n.b.: order of actual, expected is reversed
    match infer::mk_subty(fcx.infcx(), b_is_expected, infer::Misc(sp),
                          ty_b, ty_a) {
      result::Ok(()) => { /* ok */ }
      result::Err(ref err) => {
          handle_err(sp, ty_a, ty_b, err);
      }
    }
}

pub fn eqtype(fcx: &FnCtxt, sp: Span, expected: ty::t, actual: ty::t) {
    match infer::mk_eqty(fcx.infcx(), false, infer::Misc(sp), actual, expected) {
        Ok(()) => { /* ok */ }
        Err(ref err) => {
            fcx.report_mismatched_types(sp, expected, actual, err);
        }
    }
}

// Checks that the type `actual` can be coerced to `expected`.
pub fn coerce(fcx: &FnCtxt, sp: Span, expected: ty::t, expr: &ast::Expr) {
    let expr_ty = fcx.expr_ty(expr);
    debug!("demand::coerce(expected = {}, expr_ty = {})",
           expected.repr(fcx.ccx.tcx),
           expr_ty.repr(fcx.ccx.tcx));
    let expected = if ty::type_needs_infer(expected) {
        resolve_type(fcx.infcx(), expected,
                     try_resolve_tvar_shallow).unwrap_or(expected)
    } else { expected };
    match fcx.mk_assignty(expr, expr_ty, expected) {
      result::Ok(()) => { /* ok */ }
      result::Err(ref err) => {
        fcx.report_mismatched_types(sp, expected, expr_ty, err);
      }
    }
}
