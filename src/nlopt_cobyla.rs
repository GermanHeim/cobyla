#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unsafe_op_in_unsafe_fn,
    clippy::needless_return,
    clippy::zero_ptr,
    clippy::toplevel_ref_arg,
    clippy::nonminimal_bool,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::single_match,
    clippy::unnecessary_cast,
    clippy::excessive_precision,
    clippy::too_many_arguments
)]

use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
use num_traits::Float;
use std::slice;

pub(crate) fn nlopt_function_raw_callback<F, T, U>(
    n: libc::c_uint,
    x: *const T,
    _g: *mut T,
    params: *mut libc::c_void,
) -> T
where
    F: Fn(&[T], &mut U) -> T,
    T: Float,
{
    let argument = unsafe { slice::from_raw_parts(x, n as usize) };
    // let gradient = if g.is_null() {
    //     None
    // } else {
    //     Some(unsafe { slice::from_raw_parts_mut(g, n as usize) })
    // };

    // recover FunctionCfg object from supplied params and call
    let f = unsafe { &mut *(params as *mut NLoptFunctionCfg<F, T, U>) };
    let res = (f.objective_fn)(argument, &mut f.user_data);
    #[allow(forgetting_references)]
    std::mem::forget(f);
    res
}

pub(crate) fn nlopt_constraint_raw_callback<F, T, U>(
    n: libc::c_uint,
    x: *const T,
    _g: *mut T,
    params: *mut libc::c_void,
) -> T
where
    F: Fn(&[T], &mut U) -> T,
    T: Float,
{
    // let gradient = if g.is_null() {
    //     None
    // } else {
    //     Some(unsafe { slice::from_raw_parts_mut(g, n as usize) })
    // };
    // (f.constraint_fn)(argument, gradient, &mut f.user_data)
    let f = unsafe { &mut *(params as *mut NLoptConstraintCfg<F, T, U>) };
    let argument = unsafe { slice::from_raw_parts(x, n as usize) };
    (f.constraint_fn)(argument, &mut f.user_data)
}

/// Packs an objective function with a user defined parameter set of type `T`.
pub(crate) struct NLoptFunctionCfg<F, T, U>
where
    F: Fn(&[T], &mut U) -> T,
    T: Float,
{
    pub objective_fn: F,
    pub user_data: U,
    pub(crate) _phantom: std::marker::PhantomData<T>,
}

pub(crate) struct NLoptConstraintCfg<F, T, U>
where
    F: Fn(&[T], &mut U) -> T,
    T: Float,
{
    pub constraint_fn: F,
    pub user_data: U,
    pub(crate) _phantom: std::marker::PhantomData<T>,
}

/// A trait representing objective and constraints functions.
///
/// An objective function takes the form of a closure `f(x: &[f64], user_data: &mut U) -> f64`
///
/// * `x` - `n`-dimensional array
/// * `user_data` - user defined data for objective and constraint functions
pub trait Func<T, U>: Fn(&[T], &mut U) -> T where T: Float {}
impl<F, T, U> Func<T, U> for F
where
    F: Fn(&[T], &mut U) -> T,
    T: Float,
{}

enum Io {
    stderr,
    stdout,
}
fn fprintf(_io: Io, msg: &str) {
    println!("{}", msg);
}

// #![register_tool(c2rust)]
// #![feature(c_variadic, extern_types, register_tool)]
//  {
//     pub type _IO_wide_data;
//     pub type _IO_codecvt;
//     pub type _IO_marker;
//     fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
//     fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
//     fn free(__ptr: *mut libc::c_void);
//     fn abort() -> !;
//     static mut stderr: *mut FILE;
//     fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
//     fn vsnprintf(
//         _: *mut libc::c_char,
//         _: libc::c_ulong,
//         _: *const libc::c_char,
//         _: ::std::ffi::VaList,
//     ) -> libc::c_int;
//     fn sqrt(_: libc::c_double) -> libc::c_double;
//     fn fabs(_: libc::c_double) -> libc::c_double;
//     fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
// }
type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
type size_t = libc::c_ulong;
type __uint32_t = libc::c_uint;
type __off_t = libc::c_long;
type __off64_t = libc::c_long;
type __time_t = libc::c_long;
type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
type va_list = __builtin_va_list;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _IO_FILE {
//     pub _flags: libc::c_int,
//     pub _IO_read_ptr: *mut libc::c_char,
//     pub _IO_read_end: *mut libc::c_char,
//     pub _IO_read_base: *mut libc::c_char,
//     pub _IO_write_base: *mut libc::c_char,
//     pub _IO_write_ptr: *mut libc::c_char,
//     pub _IO_write_end: *mut libc::c_char,
//     pub _IO_buf_base: *mut libc::c_char,
//     pub _IO_buf_end: *mut libc::c_char,
//     pub _IO_save_base: *mut libc::c_char,
//     pub _IO_backup_base: *mut libc::c_char,
//     pub _IO_save_end: *mut libc::c_char,
//     pub _markers: *mut _IO_marker,
//     pub _chain: *mut _IO_FILE,
//     pub _fileno: libc::c_int,
//     pub _flags2: libc::c_int,
//     pub _old_offset: __off_t,
//     pub _cur_column: libc::c_ushort,
//     pub _vtable_offset: libc::c_schar,
//     pub _shortbuf: [libc::c_char; 1],
//     pub _lock: *mut libc::c_void,
//     pub _offset: __off64_t,
//     pub _codecvt: *mut _IO_codecvt,
//     pub _wide_data: *mut _IO_wide_data,
//     pub _freeres_list: *mut _IO_FILE,
//     pub _freeres_buf: *mut libc::c_void,
//     pub __pad5: size_t,
//     pub _mode: libc::c_int,
//     pub _unused2: [libc::c_char; 20],
// }
// pub type _IO_lock_t = ();
// pub type FILE = _IO_FILE;

type nlopt_func<T> = Option<
    fn(
        libc::c_uint,
        *const T,
        *mut T,
        *mut libc::c_void,
    ) -> T,
>;
type nlopt_mfunc<T> = Option<
    unsafe fn(
        libc::c_uint,
        *mut T,
        libc::c_uint,
        *const T,
        *mut T,
        *mut libc::c_void,
    ) -> (),
>;
type nlopt_precond<T> = Option<
    unsafe fn(
        libc::c_uint,
        *const T,
        *const T,
        *mut T,
        *mut libc::c_void,
    ) -> (),
>;
type nlopt_result = libc::c_int;
const NLOPT_NUM_RESULTS: nlopt_result = 7;
const NLOPT_MAXTIME_REACHED: nlopt_result = 6;
const NLOPT_MAXEVAL_REACHED: nlopt_result = 5;
const NLOPT_XTOL_REACHED: nlopt_result = 4;
const NLOPT_FTOL_REACHED: nlopt_result = 3;
const NLOPT_STOPVAL_REACHED: nlopt_result = 2;
const NLOPT_SUCCESS: nlopt_result = 1;
const NLOPT_NUM_FAILURES: nlopt_result = -6;
const NLOPT_FORCED_STOP: nlopt_result = -5;
const NLOPT_ROUNDOFF_LIMITED: nlopt_result = -4;
const NLOPT_OUT_OF_MEMORY: nlopt_result = -3;
const NLOPT_INVALID_ARGS: nlopt_result = -2;
const NLOPT_FAILURE: nlopt_result = -1;
#[derive(Clone)]
#[repr(C)]
pub(crate) struct nlopt_stopping<T> {
    pub n: libc::c_uint,
    pub minf_max: T,
    pub ftol_rel: T,
    pub ftol_abs: T,
    pub xtol_rel: T,
    pub xtol_abs: *const T,
    pub x_weights: *const T,
    pub nevals_p: *mut libc::c_int,
    pub maxeval: libc::c_int,
    pub maxtime: T,
    pub start: T,
    pub force_stop: *mut libc::c_int,
    pub stop_msg: String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct nlopt_constraint<T> {
    pub m: libc::c_uint,
    pub f: nlopt_func<T>,
    pub mf: nlopt_mfunc<T>,
    pub pre: nlopt_precond<T>,
    pub f_data: *mut libc::c_void,
    pub tol: *mut T,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct func_wrap_state<T> {
    pub f: nlopt_func<T>,
    pub f_data: *mut libc::c_void,
    pub m_orig: libc::c_uint,
    pub fc: *mut nlopt_constraint<T>,
    pub p: libc::c_uint,
    pub h: *mut nlopt_constraint<T>,
    pub xtmp: *mut T,
    pub lb: *mut T,
    pub ub: *mut T,
    pub con_tol: *mut T,
    pub scale: *mut T,
    pub stop: *mut nlopt_stopping<T>,
}
const COBYLA_MSG_NONE: C2RustUnnamed = 0;
type cobyla_function<T> = unsafe fn(
    libc::c_int,
    libc::c_int,
    *mut T,
    *mut T,
    *mut T,
    *mut func_wrap_state<T>,
) -> libc::c_int;
type uint32_t = __uint32_t;
type C2RustUnnamed = libc::c_uint;
const COBYLA_MSG_INFO: C2RustUnnamed = 3;
const COBYLA_MSG_ITER: C2RustUnnamed = 2;
const COBYLA_MSG_EXIT: C2RustUnnamed = 1;

unsafe fn nlopt_time_seed() -> libc::c_ulong {
    // let mut tv = libc::timeval {
    //     tv_sec: 0,
    //     tv_usec: 0,
    // };
    // libc::gettimeofday(&mut tv, 0 as *mut libc::timezone);
    //return (tv.tv_sec ^ tv.tv_usec) as libc::c_ulong;
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time flies");
    since_the_epoch.as_millis() as libc::c_ulong
}

    // static mut start_inited: libc::c_int = 0 as libc::c_int;
    // static mut start: libc::timeval = libc::timeval {
    //     tv_sec: 0,
    //     tv_usec: 0,
    // };
    // let mut tv: libc::timeval = libc::timeval {
    //     tv_sec: 0,
    //     tv_usec: 0,
    // };
    // if start_inited == 0 {
    //     start_inited = 1 as libc::c_int;
    //     libc::gettimeofday(&mut start, 0 as *mut libc::timezone);
    // }
    // libc::gettimeofday(&mut tv, 0 as *mut libc::timezone);
    // return (tv.tv_sec - start.tv_sec) as libc::c_double
    //     + 1.0e-6f64 * (tv.tv_usec - start.tv_usec) as libc::c_double;
unsafe fn nlopt_seconds<T: Float>() -> T {
    static mut start_inited: bool = false;
    static mut start: SystemTime = UNIX_EPOCH;
    if !start_inited {
        start_inited = true;
        start = SystemTime::now();
    }
    #[allow(static_mut_refs)]
    T::from(start
        .duration_since(UNIX_EPOCH)
        .expect("Time flies")
        .as_secs_f64()).unwrap()
}
unsafe fn sc<T: Float>(
    mut x: T,
    mut smin: T,
    mut smax: T,
) -> T {
    return smin + x * (smax - smin);
}
unsafe fn vector_norm<T: Float>(
    mut n: libc::c_uint,
    mut vec: *const T,
    mut w: *const T,
    mut scale_min: *const T,
    mut scale_max: *const T,
) -> T {
    let mut i: libc::c_uint = 0;
    let mut ret: T = T::zero();
    if !scale_min.is_null() && !scale_max.is_null() {
        if !w.is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < n {
                ret = ret + *w.offset(i as isize)
                    * (sc(
                        *vec.offset(i as isize),
                        *scale_min.offset(i as isize),
                        *scale_max.offset(i as isize),
                    )
                    .abs());
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            while i < n {
                ret = ret + (sc(
                    *vec.offset(i as isize),
                    *scale_min.offset(i as isize),
                    *scale_max.offset(i as isize),
                ))
                .abs();
                i = i.wrapping_add(1);
            }
        }
    } else if !w.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            ret = ret + *w.offset(i as isize) * (*vec.offset(i as isize)).abs();
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            ret = ret + (*vec.offset(i as isize)).abs();
            i = i.wrapping_add(1);
        }
    }
    return ret;
}
unsafe fn diff_norm<T: Float>(
    mut n: libc::c_uint,
    mut x: *const T,
    mut oldx: *const T,
    mut w: *const T,
    mut scale_min: *const T,
    mut scale_max: *const T,
) -> T {
    let mut i: libc::c_uint = 0;
    let mut ret: T = T::zero();
    if !scale_min.is_null() && !scale_max.is_null() {
        if !w.is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < n {
                ret = ret + *w.offset(i as isize)
                    * (sc(
                        *x.offset(i as isize),
                        *scale_min.offset(i as isize),
                        *scale_max.offset(i as isize),
                    ) - sc(
                        *oldx.offset(i as isize),
                        *scale_min.offset(i as isize),
                        *scale_max.offset(i as isize),
                    ))
                    .abs();
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            while i < n {
                ret = ret + (sc(
                    *x.offset(i as isize),
                    *scale_min.offset(i as isize),
                    *scale_max.offset(i as isize),
                ) - sc(
                    *oldx.offset(i as isize),
                    *scale_min.offset(i as isize),
                    *scale_max.offset(i as isize),
                ))
                .abs();
                i = i.wrapping_add(1);
            }
        }
    } else if !w.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            ret = ret + *w.offset(i as isize) * (*x.offset(i as isize) - *oldx.offset(i as isize)).abs();
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            ret = ret + (*x.offset(i as isize) - *oldx.offset(i as isize)).abs();
            i = i.wrapping_add(1);
        }
    }
    return ret;
}
unsafe fn relstop<T: Float>(
    mut vold: T,
    mut vnew: T,
    mut reltol: T,
    mut abstol: T,
) -> libc::c_int {
    if vold.is_infinite() {
        return 0 as libc::c_int;
    }
    return ((vnew - vold).abs() < abstol
        || (vnew - vold).abs() < reltol * ((vnew).abs() + (vold).abs()) * T::from(0.5).unwrap()
        || reltol > T::zero() && vnew == vold) as libc::c_int;
}

unsafe fn nlopt_stop_ftol<T: Float>(
    mut s: *const nlopt_stopping<T>,
    mut f: T,
    mut oldf: T,
) -> libc::c_int {
    return relstop(oldf, f, (*s).ftol_rel, (*s).ftol_abs);
}

unsafe fn nlopt_stop_f<T: Float>(
    mut s: *const nlopt_stopping<T>,
    mut f: T,
    mut oldf: T,
) -> libc::c_int {
    return (f <= (*s).minf_max || nlopt_stop_ftol(s, f, oldf) != 0) as libc::c_int;
}

unsafe fn nlopt_stop_x<T: Float>(
    mut s: *const nlopt_stopping<T>,
    mut x: *const T,
    mut oldx: *const T,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if diff_norm(
        (*s).n,
        x,
        oldx,
        (*s).x_weights,
        0 as *const T,
        0 as *const T,
    ) < (*s).xtol_rel
        * vector_norm(
            (*s).n,
            x,
            (*s).x_weights,
            0 as *const T,
            0 as *const T,
        )
    {
        return 1 as libc::c_int;
    }
    if ((*s).xtol_abs).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*s).n {
        if (*x.offset(i as isize) - *oldx.offset(i as isize)).abs()
            >= *((*s).xtol_abs).offset(i as isize)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}

unsafe fn nlopt_stop_dx<T: Float>(
    mut s: *const nlopt_stopping<T>,
    mut x: *const T,
    mut dx: *const T,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if vector_norm(
        (*s).n,
        dx,
        (*s).x_weights,
        0 as *const T,
        0 as *const T,
    ) < (*s).xtol_rel
        * vector_norm(
            (*s).n,
            x,
            (*s).x_weights,
            0 as *const T,
            0 as *const T,
        )
    {
        return 1 as libc::c_int;
    }
    if ((*s).xtol_abs).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*s).n {
        if (*dx.offset(i as isize)).abs() >= *((*s).xtol_abs).offset(i as isize) {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}

unsafe fn nlopt_stop_xs<T: Float>(
    mut s: *const nlopt_stopping<T>,
    mut xs: *const T,
    mut oldxs: *const T,
    mut scale_min: *const T,
    mut scale_max: *const T,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if diff_norm((*s).n, xs, oldxs, (*s).x_weights, scale_min, scale_max)
        < (*s).xtol_rel * vector_norm((*s).n, xs, (*s).x_weights, scale_min, scale_max)
    {
        return 1 as libc::c_int;
    }
    if ((*s).xtol_abs).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*s).n {
        if (sc(
            *xs.offset(i as isize),
            *scale_min.offset(i as isize),
            *scale_max.offset(i as isize),
        ) - sc(
            *oldxs.offset(i as isize),
            *scale_min.offset(i as isize),
            *scale_max.offset(i as isize),
        ))
        .abs()
            >= *((*s).xtol_abs).offset(i as isize)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}

unsafe fn nlopt_isinf<T: Float>(x: T) -> libc::c_int {
    x.is_infinite() as libc::c_int
}

unsafe fn nlopt_isfinite<T: Float>(x: T) -> libc::c_int {
    x.is_finite() as libc::c_int
}

unsafe fn nlopt_istiny<T: Float>(x: T) -> libc::c_int {
    if x == T::zero() {
        return 1 as libc::c_int;
    } else {
        return (x.abs() < T::min_positive_value()) as libc::c_int;
    };
}

unsafe fn nlopt_isnan<T: Float>(x: T) -> libc::c_int {
    x.is_nan() as i32
}

unsafe fn nlopt_stop_evals<T>(mut s: *const nlopt_stopping<T>) -> libc::c_int {
    return ((*s).maxeval > 0 as libc::c_int && *(*s).nevals_p >= (*s).maxeval) as libc::c_int;
}

unsafe fn nlopt_stop_time_<T: Float>(start: T, maxtime: T) -> libc::c_int {
    return (maxtime > T::zero() && nlopt_seconds::<T>() - start >= maxtime)
        as libc::c_int;
}

unsafe fn nlopt_stop_time<T: Float>(mut s: *const nlopt_stopping<T>) -> libc::c_int {
    return nlopt_stop_time_((*s).start, (*s).maxtime);
}

unsafe fn nlopt_stop_evalstime<T: Float>(mut stop: *const nlopt_stopping<T>) -> libc::c_int {
    return (nlopt_stop_evals(stop) != 0 || nlopt_stop_time(stop) != 0) as libc::c_int;
}

unsafe fn nlopt_stop_forced<T>(mut stop: *const nlopt_stopping<T>) -> libc::c_int {
    return (!((*stop).force_stop).is_null() && *(*stop).force_stop != 0) as libc::c_int;
}
//
// pub unsafe  fn nlopt_vsprintf(
//     mut p: *mut libc::c_char,
//     mut format: *const libc::c_char,
//     mut ap: ::std::ffi::VaList,
// ) -> *mut libc::c_char {
//     let mut len: size_t = (strlen(format)).wrapping_add(128 as libc::c_int as libc::c_ulong);
//     let mut ret: libc::c_int = 0;
//     p = realloc(p as *mut libc::c_void, len) as *mut libc::c_char;
//     if p.is_null() {
//         abort();
//     }
//     loop {
//         ret = vsnprintf(p, len, format, ap.as_va_list());
//         if !(ret < 0 as libc::c_int || ret as size_t >= len) {
//             break;
//         }
//         len = if ret >= 0 as libc::c_int {
//             (ret + 1 as libc::c_int) as size_t
//         } else {
//             len.wrapping_mul(3 as libc::c_int as libc::c_ulong) >> 1 as libc::c_int
//         };
//         p = realloc(p as *mut libc::c_void, len) as *mut libc::c_char;
//         if p.is_null() {
//             abort();
//         }
//     }
//     return p;
// }

unsafe fn nlopt_stop_msg<T>(mut s: *mut nlopt_stopping<T>, msg: &str) {
    (*s).stop_msg = msg.to_string();
}

unsafe fn nlopt_count_constraints<T>(
    mut p: libc::c_uint,
    mut c: *const nlopt_constraint<T>,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < p {
        count = count.wrapping_add((*c.offset(i as isize)).m);
        i = i.wrapping_add(1);
    }
    return count;
}

unsafe fn nlopt_max_constraint_dim<T>(
    mut p: libc::c_uint,
    mut c: *const nlopt_constraint<T>,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut max_dim: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < p {
        if (*c.offset(i as isize)).m > max_dim {
            max_dim = (*c.offset(i as isize)).m;
        }
        i = i.wrapping_add(1);
    }
    return max_dim;
}

unsafe fn nlopt_eval_constraint<T: Float, U>(
    mut result: *mut T,
    mut grad: *mut T,
    mut c: *const nlopt_constraint<T>,
    mut n: libc::c_uint,
    mut x: *const T,
) {
    if ((*c).f).is_some() {
        *result.offset(0 as libc::c_int as isize) =
        // PATCH Weird bug ((*c).f).expect("non-null function pointer") calls the objective function!!!
        // even if (*c), nlopt_constraint object was correctly built with a nlopt_constraint_raw_callback!!! 
        //    ((*c).f).expect("non-null function pointer")(n, x, grad, (*c).f_data);
        // Maybe the U generic parameter required explains it cannot work like with C ???
        nlopt_constraint_raw_callback::<&dyn Func<T, U>, T, U>(n, x, grad, (*c).f_data);
        // relf: Take the opposite to manage cstr as being nonnegative in the end like the original cobyla
        *result.offset(0 as libc::c_int as isize) = T::zero() - *result.offset(0 as libc::c_int as isize)
    } else {
        ((*c).mf).expect("non-null function pointer")((*c).m, result, n, x, grad, (*c).f_data);
    };
}

unsafe fn nlopt_compute_rescaling<T: Float>(
    mut n: libc::c_uint,
    mut dx: *const T,
) -> *mut T {
    let mut space: Box<Vec<T>> = Box::new(vec![T::zero(); usize::try_from(n).unwrap()]);
    let s = space.as_mut_ptr() as *mut T;
    std::mem::forget(space);

    let mut i: libc::c_uint = 0;
    if s.is_null() {
        return 0 as *mut T;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        *s.offset(i as isize) = T::one();
        i = i.wrapping_add(1);
    }
    if n == 1 as libc::c_int as libc::c_uint {
        return s;
    }
    i = 1 as libc::c_int as libc::c_uint;
    while i < n
        && *dx.offset(i as isize)
            == *dx.offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
    {
        i = i.wrapping_add(1);
    }
    if i < n {
        i = 1 as libc::c_int as libc::c_uint;
        while i < n {
            *s.offset(i as isize) = *dx.offset(i as isize) / *dx.offset(0 as libc::c_int as isize);
            i = i.wrapping_add(1);
        }
    }
    return s;
}

unsafe fn nlopt_rescale<T: Float>(
    mut n: libc::c_uint,
    mut s: *const T,
    mut x: *const T,
    mut xs: *mut T,
) {
    let mut i: libc::c_uint = 0;
    if s.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            *xs.offset(i as isize) = *x.offset(i as isize);
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            *xs.offset(i as isize) = *x.offset(i as isize) / *s.offset(i as isize);
            i = i.wrapping_add(1);
        }
    };
}

unsafe fn nlopt_unscale<T: Float>(
    mut n: libc::c_uint,
    mut s: *const T,
    mut x: *const T,
    mut xs: *mut T,
) {
    let mut i: libc::c_uint = 0;
    if s.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            *xs.offset(i as isize) = *x.offset(i as isize);
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            *xs.offset(i as isize) = *x.offset(i as isize) * *s.offset(i as isize);
            i = i.wrapping_add(1);
        }
    };
}

unsafe fn nlopt_new_rescaled<T: Float>(
    mut n: libc::c_uint,
    mut s: *const T,
    mut x: *const T,
) -> *mut T {
    let mut space: Box<Vec<T>> = Box::new(vec![T::zero(); usize::try_from(n).unwrap()]);
    let xs = space.as_mut_ptr() as *mut T;
    std::mem::forget(space);

    if xs.is_null() {
        return 0 as *mut T;
    }
    nlopt_rescale(n, s, x, xs);
    return xs;
}

unsafe fn nlopt_reorder_bounds<T: Float>(
    mut n: libc::c_uint,
    mut lb: *mut T,
    mut ub: *mut T,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        if *lb.offset(i as isize) > *ub.offset(i as isize) {
            let mut t: T = *lb.offset(i as isize);
            *lb.offset(i as isize) = *ub.offset(i as isize);
            *ub.offset(i as isize) = t;
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn func_wrap<T: Float, U>(
    mut ni: libc::c_int,
    mut _mi: libc::c_int,
    mut x: *mut T,
    mut f: *mut T,
    mut con: *mut T,
    mut s: *mut func_wrap_state<T>,
) -> libc::c_int {
    let mut n: libc::c_uint = ni as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut xtmp: *mut T = (*s).xtmp;
    let mut lb: *const T = (*s).lb;
    let mut ub: *const T = (*s).ub;
    j = 0 as libc::c_int as libc::c_uint;
    while j < n {
        if *x.offset(j as isize) < *lb.offset(j as isize) {
            *xtmp.offset(j as isize) = *lb.offset(j as isize);
        } else if *x.offset(j as isize) > *ub.offset(j as isize) {
            *xtmp.offset(j as isize) = *ub.offset(j as isize);
        } else {
            *xtmp.offset(j as isize) = *x.offset(j as isize);
        }
        j = j.wrapping_add(1);
    }
    nlopt_unscale(n, (*s).scale, xtmp, xtmp);
    *f = ((*s).f).expect("non-null function pointer")(
        n,
        xtmp,
        0 as *mut T,
        (*s).f_data,
    );
    if nlopt_stop_forced((*s).stop) != 0 {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < (*s).m_orig {
        nlopt_eval_constraint::<T, U>(
            con.offset(i as isize),
            0 as *mut T,
            ((*s).fc).offset(j as isize),
            n,
            xtmp,
        );
        if nlopt_stop_forced((*s).stop) != 0 {
            return 1 as libc::c_int;
        }
        k = 0 as libc::c_int as libc::c_uint;
        while k < (*((*s).fc).offset(j as isize)).m {
            *con.offset(i.wrapping_add(k) as isize) = T::zero() - *con.offset(i.wrapping_add(k) as isize);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add((*((*s).fc).offset(j as isize)).m);
        j = j.wrapping_add(1);
    }
    j = 0 as libc::c_int as libc::c_uint;
    while j < (*s).p {
        nlopt_eval_constraint::<T, U>(
            con.offset(i as isize),
            0 as *mut T,
            ((*s).h).offset(j as isize),
            n,
            xtmp,
        );
        if nlopt_stop_forced((*s).stop) != 0 {
            return 1 as libc::c_int;
        }
        k = 0 as libc::c_int as libc::c_uint;
        while k < (*((*s).h).offset(j as isize)).m {
            *con.offset(
                i.wrapping_add((*((*s).h).offset(j as isize)).m)
                    .wrapping_add(k) as isize,
            ) = T::zero() - *con.offset(i.wrapping_add(k) as isize);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(
            (2 as libc::c_int as libc::c_uint).wrapping_mul((*((*s).h).offset(j as isize)).m),
        );
        j = j.wrapping_add(1);
    }
    j = 0 as libc::c_int as libc::c_uint;
    while j < n {
        if nlopt_isinf(*lb.offset(j as isize)) == 0 {
            let fresh1 = i;
            i = i.wrapping_add(1);
            *con.offset(fresh1 as isize) = *x.offset(j as isize) - *lb.offset(j as isize);
        }
        if nlopt_isinf(*ub.offset(j as isize)) == 0 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *con.offset(fresh2 as isize) = *ub.offset(j as isize) - *x.offset(j as isize);
        }
        j = j.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
pub(crate) unsafe fn cobyla_minimize<T: Float + std::fmt::Debug + std::fmt::Display + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign, U>(
    mut n: libc::c_uint,
    mut f: nlopt_func<T>,
    mut f_data: *mut libc::c_void,
    mut m: libc::c_uint,
    mut fc: *mut nlopt_constraint<T>,
    mut p: libc::c_uint,
    mut h: *mut nlopt_constraint<T>,
    mut lb: *const T,
    mut ub: *const T,
    mut x: *mut T,
    mut minf: *mut T,
    mut stop: *mut nlopt_stopping<T>,
    mut dx: *const T,
) -> nlopt_result {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut s: func_wrap_state<T> = func_wrap_state {
        f: None,
        f_data: 0 as *mut libc::c_void,
        m_orig: 0,
        fc: 0 as *mut nlopt_constraint<T>,
        p: 0,
        h: 0 as *mut nlopt_constraint<T>,
        xtmp: 0 as *mut T,
        lb: 0 as *mut T,
        ub: 0 as *mut T,
        con_tol: 0 as *mut T,
        scale: 0 as *mut T,
        stop: 0 as *mut nlopt_stopping<T>,
    };
    let mut ret: nlopt_result = 0 as nlopt_result;
    let mut rhobeg: T = T::zero();
    let mut rhoend: T = T::zero();
    s.f = f;
    s.f_data = f_data;
    s.m_orig = m;
    s.fc = fc;
    s.p = p;
    s.h = h;
    s.stop = stop;
    s.scale = 0 as *mut T;
    s.con_tol = s.scale;
    s.xtmp = s.con_tol;
    s.ub = s.xtmp;
    s.lb = s.ub;
    s.scale = nlopt_compute_rescaling(n, dx);
    if (s.scale).is_null() {
        ret = NLOPT_OUT_OF_MEMORY;
    } else {
        j = 0 as libc::c_int as libc::c_uint;
        loop {
            if !(j < n) {
                current_block = 15652330335145281839;
                break;
            }
            if *(s.scale).offset(j as isize) == T::zero()
                || nlopt_isfinite(*(s.scale).offset(j as isize)) == 0
            {
                nlopt_stop_msg(
                    stop,
                    &format!(
                        "invalid scaling {:?} of dimension {}: possible over/underflow?",
                        *(s.scale).offset(j as isize),
                        j
                    ),
                );
                ret = NLOPT_INVALID_ARGS;
                current_block = 762786280471819104;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        match current_block {
            762786280471819104 => {}
            _ => {
                s.lb = nlopt_new_rescaled(n, s.scale, lb);
                if (s.lb).is_null() {
                    ret = NLOPT_OUT_OF_MEMORY;
                } else {
                    s.ub = nlopt_new_rescaled(n, s.scale, ub);
                    if (s.ub).is_null() {
                        ret = NLOPT_OUT_OF_MEMORY;
                    } else {
                        nlopt_reorder_bounds(n, s.lb, s.ub);

                        let mut space: Box<Vec<T>> =
                            Box::new(vec![T::zero(); usize::try_from(n).unwrap()]);
                        s.xtmp = space.as_mut_ptr() as *mut T;
                        std::mem::forget(space);

                        if (s.xtmp).is_null() {
                            ret = NLOPT_OUT_OF_MEMORY;
                        } else {
                            rhobeg = (*dx.offset(0 as libc::c_int as isize)
                                / *(s.scale).offset(0 as libc::c_int as isize))
                            .abs();
                            rhoend = (*stop).xtol_rel * rhobeg;
                            if !((*stop).xtol_abs).is_null() {
                                j = 0 as libc::c_int as libc::c_uint;
                                while j < n {
                                    if rhoend
                                        < *((*stop).xtol_abs).offset(j as isize)
                                            / (*(s.scale).offset(j as isize)).abs()
                                    {
                                        rhoend = *((*stop).xtol_abs).offset(j as isize)
                                            / (*(s.scale).offset(j as isize)).abs();
                                    }
                                    j = j.wrapping_add(1);
                                }
                            }
                            m = (nlopt_count_constraints(m, fc)).wrapping_add(
                                (2 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(nlopt_count_constraints(p, h)),
                            );
                            j = 0 as libc::c_int as libc::c_uint;
                            while j < n {
                                if nlopt_isinf(*lb.offset(j as isize)) == 0 {
                                    m = m.wrapping_add(1);
                                }
                                if nlopt_isinf(*ub.offset(j as isize)) == 0 {
                                    m = m.wrapping_add(1);
                                }
                                j = j.wrapping_add(1);
                            }

                            if m > 0 {
                                let mut space: Box<Vec<T>> =
                                    Box::new(vec![T::zero(); usize::try_from(m).unwrap()]);
                                s.con_tol = space.as_mut_ptr() as *mut T;
                                std::mem::forget(space);
                            }

                            if m != 0 && (s.con_tol).is_null() {
                                ret = NLOPT_OUT_OF_MEMORY;
                            } else {
                                j = 0 as libc::c_int as libc::c_uint;
                                while j < m {
                                    *(s.con_tol).offset(j as isize) =
                                        T::zero();
                                    j = j.wrapping_add(1);
                                }
                                i = 0 as libc::c_int as libc::c_uint;
                                j = i;
                                while i < s.m_orig {
                                    let mut ji: libc::c_uint = j;
                                    let mut jnext: libc::c_uint =
                                        j.wrapping_add((*fc.offset(i as isize)).m);
                                    while j < jnext {
                                        *(s.con_tol).offset(j as isize) =
                                            *((*fc.offset(i as isize)).tol)
                                                .offset(j.wrapping_sub(ji) as isize);
                                        j = j.wrapping_add(1);
                                    }
                                    i = i.wrapping_add(1);
                                }
                                i = 0 as libc::c_int as libc::c_uint;
                                while i < s.p {
                                    let mut ji_0: libc::c_uint = j;
                                    let mut jnext_0: libc::c_uint =
                                        j.wrapping_add((*h.offset(i as isize)).m);
                                    while j < jnext_0 {
                                        *(s.con_tol).offset(j as isize) =
                                            *((*h.offset(i as isize)).tol)
                                                .offset(j.wrapping_sub(ji_0) as isize);
                                        j = j.wrapping_add(1);
                                    }
                                    ji_0 = j;
                                    jnext_0 = j.wrapping_add((*h.offset(i as isize)).m);
                                    while j < jnext_0 {
                                        *(s.con_tol).offset(j as isize) =
                                            *((*h.offset(i as isize)).tol)
                                                .offset(j.wrapping_sub(ji_0) as isize);
                                        j = j.wrapping_add(1);
                                    }
                                    i = i.wrapping_add(1);
                                }
                                nlopt_rescale(n, s.scale, x, x);
                                ret = cobyla(
                                    n as libc::c_int,
                                    m as libc::c_int,
                                    x,
                                    minf,
                                    rhobeg,
                                    rhoend,
                                    stop,
                                    s.lb,
                                    s.ub,
                                    COBYLA_MSG_NONE as libc::c_int,
                                    Some(
                                        func_wrap::<T, U>
                                            as unsafe fn(
                                                libc::c_int,
                                                libc::c_int,
                                                *mut T,
                                                *mut T,
                                                *mut T,
                                                *mut func_wrap_state<T>,
                                            )
                                                -> libc::c_int,
                                    ),
                                    &mut s,
                                );
                                nlopt_unscale(n, s.scale, x, x);
                                j = 0 as libc::c_int as libc::c_uint;
                                while j < n {
                                    if *x.offset(j as isize) < *lb.offset(j as isize) {
                                        *x.offset(j as isize) = *lb.offset(j as isize);
                                    }
                                    if *x.offset(j as isize) > *ub.offset(j as isize) {
                                        *x.offset(j as isize) = *ub.offset(j as isize);
                                    }
                                    j = j.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // free(s.con_tol as *mut libc::c_void);
    // free(s.xtmp as *mut libc::c_void);
    // free(s.ub as *mut libc::c_void);
    // free(s.lb as *mut libc::c_void);
    // free(s.scale as *mut libc::c_void);

    if m > 0 {
        let _ = Box::from_raw(s.con_tol);
    }
    let _ = Box::from_raw(s.xtmp);
    let _ = Box::from_raw(s.ub);
    let _ = Box::from_raw(s.lb);
    let _ = Box::from_raw(s.scale);

    return ret;
}
unsafe fn lcg_rand(mut seed: *mut uint32_t) -> uint32_t {
    *seed = (*seed)
        .wrapping_mul(1103515245 as libc::c_int as libc::c_uint)
        .wrapping_add(12345 as libc::c_int as libc::c_uint);
    return *seed;
}
unsafe fn lcg_urand<T: Float>(
    mut seed: *mut uint32_t,
    mut a: T,
    mut b: T,
) -> T {
    // Generate random value in range [a, b] from uint32 random seed
    // Using generic T throughout, no conversion through f64
    let rand_val = T::from(lcg_rand(seed)).unwrap();
    let max_val = T::from(u32::MAX).unwrap();
    return a + rand_val * (b - a) / max_val;
}

unsafe fn cobyla<T: Float + std::fmt::Display + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign>(
    mut n: libc::c_int,
    mut m: libc::c_int,
    mut x: *mut T,
    mut minf: *mut T,
    mut rhobeg: T,
    mut rhoend: T,
    mut stop: *mut nlopt_stopping<T>,
    mut lb: *const T,
    mut ub: *const T,
    mut iprint: libc::c_int,
    mut calcfc: Option<cobyla_function<T>>,
    mut state: *mut func_wrap_state<T>,
) -> nlopt_result {
    let mut icon: libc::c_int = 0;
    let mut isim: libc::c_int = 0;
    let mut isigb: libc::c_int = 0;
    let mut idatm: libc::c_int = 0;
    let mut iveta: libc::c_int = 0;
    let mut isimi: libc::c_int = 0;
    let mut ivsig: libc::c_int = 0;
    let mut iwork: libc::c_int = 0;
    let mut ia: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut mpp: libc::c_int = 0;
    let mut _iact: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut _w: *mut T = 0 as *mut T;
    let mut rc: nlopt_result = 0 as nlopt_result;
    *(*stop).nevals_p = 0 as libc::c_int;
    if n == 0 as libc::c_int {
        if iprint >= 1 as libc::c_int {
            fprintf(Io::stderr, "cobyla: N==0.");
        }
        return NLOPT_SUCCESS;
    }
    if n < 0 as libc::c_int || m < 0 as libc::c_int {
        if iprint >= 1 as libc::c_int {
            fprintf(Io::stderr, "cobyla: N<0 or M<0.");
        }
        return NLOPT_INVALID_ARGS;
    }

    let space_size = n * (3 as libc::c_int * n + 2 as libc::c_int * m + 11 as libc::c_int)
        + 4 as libc::c_int * m
        + 6 as libc::c_int;
    let mut space: Box<Vec<T>> =
        Box::new(vec![T::zero(); usize::try_from(space_size).unwrap()]);
    let mut w = space.as_mut_ptr() as *mut T;
    std::mem::forget(space);

    if w.is_null() {
        if iprint >= 1 as libc::c_int {
            fprintf(Io::stderr, "cobyla: memory allocation error");
        }
        return NLOPT_OUT_OF_MEMORY;
    }
    // iact = malloc(
    //     ((m + 1 as libc::c_int) as libc::c_uint as libc::c_ulong)
    //         .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    // ) as *mut libc::c_int;

    let space_size = m + 1;
    let mut space: Box<Vec<libc::c_int>> =
        Box::new(vec![0; usize::try_from(space_size).unwrap()]);
    let mut iact = space.as_mut_ptr() as *mut libc::c_int;
    std::mem::forget(space);

    if iact.is_null() {
        if iprint >= 1 as libc::c_int {
            fprintf(Io::stderr, "cobyla: memory allocation error.");
        }
        //free(w as *mut libc::c_void);
        let _ = Box::from_raw(w);
        return NLOPT_OUT_OF_MEMORY;
    }
    iact = iact.offset(-1);
    w = w.offset(-1);
    x = x.offset(-1);
    lb = lb.offset(-1);
    ub = ub.offset(-1);
    mpp = m + 2 as libc::c_int;
    icon = 1 as libc::c_int;
    isim = icon + mpp;
    isimi = isim + n * n + n;
    idatm = isimi + n * n;
    ia = idatm + n * mpp + mpp;
    ivsig = ia + m * n + n;
    iveta = ivsig + n;
    isigb = iveta + n;
    idx = isigb + n;
    iwork = idx + n;
    rc = cobylb(
        &mut n,
        &mut m,
        &mut mpp,
        &mut *x.offset(1 as libc::c_int as isize),
        minf,
        &mut rhobeg,
        rhoend,
        stop,
        &*lb.offset(1 as libc::c_int as isize),
        &*ub.offset(1 as libc::c_int as isize),
        &mut iprint,
        &mut *w.offset(icon as isize),
        &mut *w.offset(isim as isize),
        &mut *w.offset(isimi as isize),
        &mut *w.offset(idatm as isize),
        &mut *w.offset(ia as isize),
        &mut *w.offset(ivsig as isize),
        &mut *w.offset(iveta as isize),
        &mut *w.offset(isigb as isize),
        &mut *w.offset(idx as isize),
        &mut *w.offset(iwork as isize),
        &mut *iact.offset(1 as libc::c_int as isize),
        calcfc,
        state,
    );
    iact = iact.offset(1);
    w = w.offset(1);
    // free(w as *mut libc::c_void);
    // free(iact as *mut libc::c_void);
    let _ = Box::from_raw(w);
    let _ = Box::from_raw(iact);
    return rc;
}
unsafe fn cobylb<T: Float + std::fmt::Display + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign>(
    mut n: *mut libc::c_int,
    mut m: *mut libc::c_int,
    mut mpp: *mut libc::c_int,
    mut x: *mut T,
    mut minf: *mut T,
    mut rhobeg: *mut T,
    mut rhoend: T,
    mut stop: *mut nlopt_stopping<T>,
    mut lb: *const T,
    mut ub: *const T,
    mut iprint: *mut libc::c_int,
    mut con: *mut T,
    mut sim: *mut T,
    mut simi: *mut T,
    mut datmat: *mut T,
    mut a: *mut T,
    mut vsig: *mut T,
    mut veta: *mut T,
    mut sigbar: *mut T,
    mut dx: *mut T,
    mut w: *mut T,
    mut iact: *mut libc::c_int,
    mut calcfc: Option<cobyla_function<T>>,
    mut state: *mut func_wrap_state<T>,
) -> nlopt_result {
    let mut current_block: u64;
    let mut sim_dim1: libc::c_int = 0;
    let mut sim_offset: libc::c_int = 0;
    let mut simi_dim1: libc::c_int = 0;
    let mut simi_offset: libc::c_int = 0;
    let mut datmat_dim1: libc::c_int = 0;
    let mut datmat_offset: libc::c_int = 0;
    let mut a_dim1: libc::c_int = 0;
    let mut a_offset: libc::c_int = 0;
    let mut i__1: libc::c_int = 0;
    let mut i__2: libc::c_int = 0;
    let mut i__3: libc::c_int = 0;
    let mut d__1: T = T::zero();
    let mut d__2: T = T::zero();
    let mut alpha: T = T::zero();
    let mut delta: T = T::zero();
    let mut denom: T = T::zero();
    let mut tempa: T = T::zero();
    let mut barmu: T = T::zero();
    let mut beta: T = T::zero();
    let mut cmin: T = T::zero();
    let mut cmax: T = T::zero();
    let mut cvmaxm: T = T::zero();
    let mut dxsign: T = T::zero();
    let mut prerem: T = T::zero();
    let mut edgmax: T = T::zero();
    let mut pareta: T = T::zero();
    let mut prerec: T = T::zero();
    let mut phimin: T = T::zero();
    let mut parsig: T = T::zero();
    let mut gamma_: T = T::zero();
    let mut phi: T = T::zero();
    let mut rho: T = T::zero();
    let mut sum: T = T::zero();
    let mut ratio: T = T::zero();
    let mut vmold: T = T::zero();
    let mut parmu: T = T::zero();
    let mut error: T = T::zero();
    let mut vmnew: T = T::zero();
    let mut resmax: T = T::zero();
    let mut cvmaxp: T = T::zero();
    let mut resnew: T = T::zero();
    let mut trured: T = T::zero();
    let mut temp: T = T::zero();
    let mut wsig: T = T::zero();
    let mut f: T = T::zero();
    let mut weta: T = T::zero();
    let mut i__: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut idxnew: libc::c_int = 0;
    let mut iflag: libc::c_int = 0 as libc::c_int;
    let mut iptemp: libc::c_int = 0;
    let mut isdirn: libc::c_int = 0;
    let mut izdota: libc::c_int = 0;
    let mut ivmc: libc::c_int = 0;
    let mut ivmd: libc::c_int = 0;
    let mut mp: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut ibrnch: libc::c_int = 0;
    let mut nbest: libc::c_int = 0;
    let mut ifull: libc::c_int = 0 as libc::c_int;
    let mut iptem: libc::c_int = 0;
    let mut jdrop: libc::c_int = 0;
    let mut rc: nlopt_result = NLOPT_SUCCESS;
    let mut seed: uint32_t = (*n + *m) as uint32_t;
    let mut feasible: libc::c_int = 0;
    *minf = T::infinity();
    a_dim1 = *n;
    a_offset = 1 as libc::c_int + a_dim1 * 1 as libc::c_int;
    a = a.offset(-(a_offset as isize));
    simi_dim1 = *n;
    simi_offset = 1 as libc::c_int + simi_dim1 * 1 as libc::c_int;
    simi = simi.offset(-(simi_offset as isize));
    sim_dim1 = *n;
    sim_offset = 1 as libc::c_int + sim_dim1 * 1 as libc::c_int;
    sim = sim.offset(-(sim_offset as isize));
    datmat_dim1 = *mpp;
    datmat_offset = 1 as libc::c_int + datmat_dim1 * 1 as libc::c_int;
    datmat = datmat.offset(-(datmat_offset as isize));
    x = x.offset(-1);
    con = con.offset(-1);
    vsig = vsig.offset(-1);
    veta = veta.offset(-1);
    sigbar = sigbar.offset(-1);
    dx = dx.offset(-1);
    w = w.offset(-1);
    iact = iact.offset(-1);
    lb = lb.offset(-1);
    ub = ub.offset(-1);
    iptem = if *n <= 4 as libc::c_int {
        *n
    } else {
        4 as libc::c_int
    };
    iptemp = iptem + 1 as libc::c_int;
    np = *n + 1 as libc::c_int;
    mp = *m + 1 as libc::c_int;
    alpha = T::from(0.25).unwrap();
    beta = T::from(2.1).unwrap();
    gamma_ = T::from(0.5).unwrap();
    delta = T::from(1.1).unwrap();
    rho = *rhobeg;
    parmu = T::zero();
    if *iprint >= 2 as libc::c_int {
        fprintf(
            Io::stderr,
            &format!(
                "cobyla: the initial value of RHO is {} and PARMU is set to zero.",
                rho
            ),
        );
    }
    temp = T::one() / rho;
    i__1 = *n;
    i__ = 1 as libc::c_int;
    while i__ <= i__1 {
        let mut rhocur: T = T::zero();
        *sim.offset((i__ + np * sim_dim1) as isize) = *x.offset(i__ as isize);
        i__2 = *n;
        j = 1 as libc::c_int;
        while j <= i__2 {
            *sim.offset((i__ + j * sim_dim1) as isize) = T::zero();
            *simi.offset((i__ + j * simi_dim1) as isize) = T::zero();
            j += 1;
        }
        rhocur = rho;
        if *x.offset(i__ as isize) + rhocur > *ub.offset(i__ as isize) {
            if *x.offset(i__ as isize) - rhocur >= *lb.offset(i__ as isize) {
                rhocur = -rhocur;
            } else if *ub.offset(i__ as isize) - *x.offset(i__ as isize)
                > *x.offset(i__ as isize) - *lb.offset(i__ as isize)
            {
                rhocur = T::from(0.5).unwrap() * (*ub.offset(i__ as isize) - *x.offset(i__ as isize));
            } else {
                rhocur = T::from(0.5).unwrap() * (*x.offset(i__ as isize) - *lb.offset(i__ as isize));
            }
        }
        *sim.offset((i__ + i__ * sim_dim1) as isize) = rhocur;
        *simi.offset((i__ + i__ * simi_dim1) as isize) = T::one() / rhocur;
        i__ += 1;
    }
    jdrop = np;
    ibrnch = 0 as libc::c_int;
    'c_6104: loop {
        if nlopt_stop_forced(stop) != 0 {
            rc = NLOPT_FORCED_STOP;
        } else if *(*stop).nevals_p > 0 as libc::c_int {
            if nlopt_stop_evals(stop) != 0 {
                rc = NLOPT_MAXEVAL_REACHED;
            } else if nlopt_stop_time(stop) != 0 {
                rc = NLOPT_MAXTIME_REACHED;
            }
        }
        if rc as libc::c_int != NLOPT_SUCCESS as libc::c_int {
            current_block = 16949430136398296108;
            break;
        }
        let ref mut fresh3 = *(*stop).nevals_p;
        *fresh3 += 1;
        if calcfc.expect("non-null function pointer")(
            *n,
            *m,
            &mut *x.offset(1 as libc::c_int as isize),
            &mut f,
            &mut *con.offset(1 as libc::c_int as isize),
            state,
        ) != 0
        {
            if *iprint >= 1 as libc::c_int {
                fprintf(Io::stderr, "cobyla: user requested end of minimization");
            }
            rc = NLOPT_FORCED_STOP;
            current_block = 16949430136398296108;
            break;
        } else {
            resmax = T::zero();
            feasible = 1 as libc::c_int;
            if *m > 0 as libc::c_int {
                i__1 = *m;
                k = 1 as libc::c_int;
                while k <= i__1 {
                    d__1 = resmax;
                    d__2 = -*con.offset(k as isize);
                    resmax = if d__1 >= d__2 { d__1 } else { d__2 };
                    if d__2 > *((*state).con_tol).offset((k - 1 as libc::c_int) as isize) {
                        feasible = 0 as libc::c_int;
                    }
                    k += 1;
                }
            }
            if f < (*stop).minf_max && feasible != 0 {
                rc = NLOPT_STOPVAL_REACHED;
                current_block = 10710279849762345920;
                break;
            } else {
                if *(*stop).nevals_p == *iprint - 1 as libc::c_int || *iprint == 3 as libc::c_int {
                    fprintf(
                        Io::stderr,
                        &format!(
                            "cobyla: NFVALS = {}, F ={}, MAXCV ={}",
                            *(*stop).nevals_p,
                            f,
                            resmax
                        ),
                    );
                    i__1 = iptem;
                    fprintf(Io::stderr, "cobyla: X =");
                    i__ = 1 as libc::c_int;
                    while i__ <= i__1 {
                        if i__ > 1 as libc::c_int {
                            fprintf(Io::stderr, "  ");
                        }
                        fprintf(Io::stderr, &format!("{}", *x.offset(i__ as isize)));
                        i__ += 1;
                    }
                    if iptem < *n {
                        i__1 = *n;
                        i__ = iptemp;
                        while i__ <= i__1 {
                            if (i__ - 1 as libc::c_int) % 4 as libc::c_int == 0 {
                                fprintf(Io::stderr, "\ncobyla:  ");
                            }
                            fprintf(Io::stderr, &format!("{}", *x.offset(i__ as isize)));
                            i__ += 1;
                        }
                    }
                    fprintf(Io::stderr, "");
                }
                *con.offset(mp as isize) = f;
                *con.offset(*mpp as isize) = resmax;
                if ibrnch == 1 as libc::c_int {
                    vmold = *datmat.offset((mp + np * datmat_dim1) as isize)
                        + parmu * *datmat.offset((*mpp + np * datmat_dim1) as isize);
                    vmnew = f + parmu * resmax;
                    trured = vmold - vmnew;
                    if parmu == T::zero() && f == *datmat.offset((mp + np * datmat_dim1) as isize) {
                        prerem = prerec;
                        trured = *datmat.offset((*mpp + np * datmat_dim1) as isize) - resmax;
                    }
                    ratio = T::zero();
                    if trured <= T::from(0.0).unwrap() {
                        ratio = T::one();
                    }
                    jdrop = 0 as libc::c_int;
                    i__1 = *n;
                    j = 1 as libc::c_int;
                    while j <= i__1 {
                        temp = T::zero();
                        i__2 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__2 {
                            temp += *simi.offset((j + i__ * simi_dim1) as isize)
                                * *dx.offset(i__ as isize);
                            i__ += 1;
                        }
                        temp = temp.abs();
                        if temp > ratio {
                            jdrop = j;
                            ratio = temp;
                        }
                        *sigbar.offset(j as isize) = temp * *vsig.offset(j as isize);
                        j += 1;
                    }
                    edgmax = delta * rho;
                    l = 0 as libc::c_int;
                    i__1 = *n;
                    j = 1 as libc::c_int;
                    while j <= i__1 {
                        if *sigbar.offset(j as isize) >= parsig
                            || *sigbar.offset(j as isize) >= *vsig.offset(j as isize)
                        {
                            temp = *veta.offset(j as isize);
                            if trured > T::zero() {
                                temp = T::zero();
                                i__2 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__2 {
                                    d__1 = *dx.offset(i__ as isize)
                                        - *sim.offset((i__ + j * sim_dim1) as isize);
                                    temp += d__1 * d__1;
                                    i__ += 1;
                                }
                                temp = temp.sqrt();
                            }
                            if temp > edgmax {
                                l = j;
                                edgmax = temp;
                            }
                        }
                        j += 1;
                    }
                    if l > 0 as libc::c_int {
                        jdrop = l;
                    }
                    if jdrop == 0 as libc::c_int {
                        current_block = 17974563553836679504;
                    } else {
                        temp = T::zero();
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            *sim.offset((i__ + jdrop * sim_dim1) as isize) =
                                *dx.offset(i__ as isize);
                            temp += *simi.offset((jdrop + i__ * simi_dim1) as isize)
                                * *dx.offset(i__ as isize);
                            i__ += 1;
                        }
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            *simi.offset((jdrop + i__ * simi_dim1) as isize) /= temp;
                            i__ += 1;
                        }
                        i__1 = *n;
                        j = 1 as libc::c_int;
                        while j <= i__1 {
                            if j != jdrop {
                                temp = T::zero();
                                i__2 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__2 {
                                    temp += *simi.offset((j + i__ * simi_dim1) as isize)
                                        * *dx.offset(i__ as isize);
                                    i__ += 1;
                                }
                                i__2 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__2 {
                                    *simi.offset((j + i__ * simi_dim1) as isize) -=
                                        temp * *simi.offset((jdrop + i__ * simi_dim1) as isize);
                                    i__ += 1;
                                }
                            }
                            j += 1;
                        }
                        i__1 = *mpp;
                        k = 1 as libc::c_int;
                        while k <= i__1 {
                            *datmat.offset((k + jdrop * datmat_dim1) as isize) =
                                *con.offset(k as isize);
                            k += 1;
                        }
                        if trured > T::zero() && trured >= prerem * T::from(0.1).unwrap() {
                            if trured >= prerem * T::from(0.9).unwrap() && trured <= prerem * T::from(1.1).unwrap() && iflag != 0
                            {
                                rho = rho * T::from(2.0).unwrap();
                            }
                            current_block = 16207618807156029286;
                        } else {
                            current_block = 17974563553836679504;
                        }
                    }
                } else {
                    i__1 = *mpp;
                    k = 1 as libc::c_int;
                    while k <= i__1 {
                        *datmat.offset((k + jdrop * datmat_dim1) as isize) =
                            *con.offset(k as isize);
                        k += 1;
                    }
                    if !(*(*stop).nevals_p > np) {
                        if jdrop <= *n {
                            if *datmat.offset((mp + np * datmat_dim1) as isize) <= f {
                                *x.offset(jdrop as isize) =
                                    *sim.offset((jdrop + np * sim_dim1) as isize);
                            } else {
                                let mut rhocur_0: T = *x.offset(jdrop as isize)
                                    - *sim.offset((jdrop + np * sim_dim1) as isize);
                                *sim.offset((jdrop + np * sim_dim1) as isize) =
                                    *x.offset(jdrop as isize);
                                i__1 = *mpp;
                                k = 1 as libc::c_int;
                                while k <= i__1 {
                                    *datmat.offset((k + jdrop * datmat_dim1) as isize) =
                                        *datmat.offset((k + np * datmat_dim1) as isize);
                                    *datmat.offset((k + np * datmat_dim1) as isize) =
                                        *con.offset(k as isize);
                                    k += 1;
                                }
                                i__1 = jdrop;
                                k = 1 as libc::c_int;
                                while k <= i__1 {
                                    *sim.offset((jdrop + k * sim_dim1) as isize) = -rhocur_0;
                                    temp = T::zero();
                                    i__2 = jdrop;
                                    i__ = k;
                                    while i__ <= i__2 {
                                        temp -= *simi.offset((i__ + k * simi_dim1) as isize);
                                        i__ += 1;
                                    }
                                    *simi.offset((jdrop + k * simi_dim1) as isize) = temp;
                                    k += 1;
                                }
                            }
                        }
                        if *(*stop).nevals_p <= *n {
                            jdrop = *(*stop).nevals_p;
                            *x.offset(jdrop as isize) +=
                                *sim.offset((jdrop + jdrop * sim_dim1) as isize);
                            continue;
                        }
                    }
                    ibrnch = 1 as libc::c_int;
                    current_block = 16207618807156029286;
                }
                'c_6122: loop {
                    match current_block {
                        17974563553836679504 => {
                            if iflag == 0 as libc::c_int {
                                ibrnch = 0 as libc::c_int;
                                current_block = 16207618807156029286;
                            } else {
                                let mut fbest: T = if ifull == 1 as libc::c_int {
                                    f
                                } else {
                                    *datmat.offset((mp + np * datmat_dim1) as isize)
                                };
                                if fbest < *minf && nlopt_stop_ftol(stop, fbest, *minf) != 0 {
                                    rc = NLOPT_FTOL_REACHED;
                                    current_block = 16949430136398296108;
                                    break 'c_6104;
                                } else {
                                    *minf = fbest;
                                    if rho > rhoend {
                                        rho = rho * T::from(0.5).unwrap();
                                        if rho <= rhoend * T::from(1.5).unwrap() {
                                            rho = rhoend;
                                        }
                                        if parmu > T::zero() {
                                            denom = T::zero();
                                            i__1 = mp;
                                            k = 1 as libc::c_int;
                                            while k <= i__1 {
                                                cmin =
                                                    *datmat.offset((k + np * datmat_dim1) as isize);
                                                cmax = cmin;
                                                i__2 = *n;
                                                i__ = 1 as libc::c_int;
                                                while i__ <= i__2 {
                                                    d__1 = cmin;
                                                    d__2 = *datmat
                                                        .offset((k + i__ * datmat_dim1) as isize);
                                                    cmin = if d__1 <= d__2 { d__1 } else { d__2 };
                                                    d__1 = cmax;
                                                    d__2 = *datmat
                                                        .offset((k + i__ * datmat_dim1) as isize);
                                                    cmax = if d__1 >= d__2 { d__1 } else { d__2 };
                                                    i__ += 1;
                                                }
                                                if k <= *m && cmin < cmax * T::from(0.5).unwrap() {
                                                    temp = (if cmax >= T::zero() {
                                                        cmax
                                                    } else {
                                                        T::zero()
                                                    }) - cmin;
                                                    if denom <= T::zero() {
                                                        denom = temp;
                                                    } else {
                                                        denom = if denom <= temp {
                                                            denom
                                                        } else {
                                                            temp
                                                        };
                                                    }
                                                }
                                                k += 1;
                                            }
                                            if denom == T::zero() {
                                                parmu = T::zero();
                                            } else if cmax - cmin < parmu * denom {
                                                parmu = (cmax - cmin) / denom;
                                            }
                                        }
                                        if *iprint >= 2 as libc::c_int {
                                            fprintf(
                                                Io::stderr,
                                                &format!(
                                                    "cobyla: reduction in RHO to {} and PARMU ={}",
                                                    rho, parmu
                                                ),
                                            );
                                        }
                                        if *iprint == 2 as libc::c_int {
                                            fprintf(
                                                Io::stderr,
                                                &format!(
                                                    "cobyla: NFVALS = {}, F ={}, MAXCV ={}",
                                                    *(*stop).nevals_p,
                                                    *datmat
                                                        .offset((mp + np * datmat_dim1) as isize),
                                                    *datmat
                                                        .offset((*mpp + np * datmat_dim1) as isize)
                                                ),
                                            );
                                            fprintf(Io::stderr, "cobyla: X =");
                                            i__1 = iptem;
                                            i__ = 1 as libc::c_int;
                                            while i__ <= i__1 {
                                                if i__ > 1 as libc::c_int {
                                                    fprintf(Io::stderr, "  ");
                                                }
                                                fprintf(
                                                    Io::stderr,
                                                    &format!(
                                                        "{}",
                                                        *sim.offset((i__ + np * sim_dim1) as isize)
                                                    ),
                                                );
                                                i__ += 1;
                                            }
                                            if iptem < *n {
                                                i__1 = *n;
                                                i__ = iptemp;
                                                while i__ <= i__1 {
                                                    if (i__ - 1 as libc::c_int) % 4 as libc::c_int
                                                        == 0
                                                    {
                                                        fprintf(Io::stderr, "\ncobyla:  ");
                                                    }
                                                    fprintf(
                                                        Io::stderr,
                                                        &format!("{}", *x.offset(i__ as isize)),
                                                    );
                                                    i__ += 1;
                                                }
                                            }
                                            fprintf(Io::stderr, "");
                                        }
                                        current_block = 16207618807156029286;
                                    } else {
                                        rc = (if rhoend > T::zero() {
                                            NLOPT_XTOL_REACHED as libc::c_int
                                        } else {
                                            NLOPT_ROUNDOFF_LIMITED as libc::c_int
                                        })
                                            as nlopt_result;
                                        if *iprint >= 1 as libc::c_int {
                                            fprintf(Io::stderr, "cobyla: normal return.");
                                        }
                                        if ifull == 1 as libc::c_int {
                                            current_block = 10710279849762345920;
                                            break 'c_6104;
                                        } else {
                                            current_block = 16949430136398296108;
                                            break 'c_6104;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            phimin = *datmat.offset((mp + np * datmat_dim1) as isize)
                                + parmu * *datmat.offset((*mpp + np * datmat_dim1) as isize);
                            nbest = np;
                            i__1 = *n;
                            j = 1 as libc::c_int;
                            while j <= i__1 {
                                temp = *datmat.offset((mp + j * datmat_dim1) as isize)
                                    + parmu * *datmat.offset((*mpp + j * datmat_dim1) as isize);
                                if temp < phimin {
                                    nbest = j;
                                    phimin = temp;
                                } else if temp == phimin && parmu == T::zero() {
                                    if *datmat.offset((*mpp + j * datmat_dim1) as isize)
                                        < *datmat.offset((*mpp + nbest * datmat_dim1) as isize)
                                    {
                                        nbest = j;
                                    }
                                }
                                j += 1;
                            }
                            if nbest <= *n {
                                i__1 = *mpp;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    temp = *datmat.offset((i__ + np * datmat_dim1) as isize);
                                    *datmat.offset((i__ + np * datmat_dim1) as isize) =
                                        *datmat.offset((i__ + nbest * datmat_dim1) as isize);
                                    *datmat.offset((i__ + nbest * datmat_dim1) as isize) = temp;
                                    i__ += 1;
                                }
                                i__1 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    temp = *sim.offset((i__ + nbest * sim_dim1) as isize);
                                    *sim.offset((i__ + nbest * sim_dim1) as isize) = T::zero();
                                    *sim.offset((i__ + np * sim_dim1) as isize) += temp;
                                    tempa = T::zero();
                                    i__2 = *n;
                                    k = 1 as libc::c_int;
                                    while k <= i__2 {
                                        *sim.offset((i__ + k * sim_dim1) as isize) -= temp;
                                        tempa -= *simi.offset((k + i__ * simi_dim1) as isize);
                                        k += 1;
                                    }
                                    *simi.offset((nbest + i__ * simi_dim1) as isize) = tempa;
                                    i__ += 1;
                                }
                            }
                            error = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                i__2 = *n;
                                j = 1 as libc::c_int;
                                while j <= i__2 {
                                    temp = T::zero();
                                    if i__ == j {
                                        temp = temp + T::from(-1.0).unwrap();
                                    }
                                    i__3 = *n;
                                    k = 1 as libc::c_int;
                                    while k <= i__3 {
                                        if *sim.offset((k + j * sim_dim1) as isize)
                                            != T::zero()
                                        {
                                            temp += *simi.offset((i__ + k * simi_dim1) as isize)
                                                * *sim.offset((k + j * sim_dim1) as isize);
                                        }
                                        k += 1;
                                    }
                                    d__1 = error;
                                    d__2 = temp.abs();
                                    error = if d__1 >= d__2 { d__1 } else { d__2 };
                                    j += 1;
                                }
                                i__ += 1;
                            }
                            if error > T::from(0.1).unwrap() {
                                if *iprint >= 1 as libc::c_int {
                                    fprintf(
                                        Io::stderr,
                                        "cobyla: rounding errors are becoming damaging.",
                                    );
                                }
                                rc = NLOPT_ROUNDOFF_LIMITED;
                                current_block = 16949430136398296108;
                                break 'c_6104;
                            } else {
                                i__2 = mp;
                                k = 1 as libc::c_int;
                                while k <= i__2 {
                                    *con.offset(k as isize) =
                                        -*datmat.offset((k + np * datmat_dim1) as isize);
                                    i__1 = *n;
                                    j = 1 as libc::c_int;
                                    while j <= i__1 {
                                        *w.offset(j as isize) = *datmat
                                            .offset((k + j * datmat_dim1) as isize)
                                            + *con.offset(k as isize);
                                        j += 1;
                                    }
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        temp = T::zero();
                                        i__3 = *n;
                                        j = 1 as libc::c_int;
                                        while j <= i__3 {
                                            temp += *w.offset(j as isize)
                                                * *simi.offset((j + i__ * simi_dim1) as isize);
                                            j += 1;
                                        }
                                        if k == mp {
                                            temp = -temp;
                                        }
                                        *a.offset((i__ + k * a_dim1) as isize) = temp;
                                        i__ += 1;
                                    }
                                    k += 1;
                                }
                                iflag = 1 as libc::c_int;
                                parsig = alpha * rho;
                                pareta = beta * rho;
                                i__1 = *n;
                                j = 1 as libc::c_int;
                                while j <= i__1 {
                                    wsig = T::zero();
                                    weta = T::zero();
                                    i__2 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__2 {
                                        d__1 = *simi.offset((j + i__ * simi_dim1) as isize);
                                        wsig += d__1 * d__1;
                                        d__1 = *sim.offset((i__ + j * sim_dim1) as isize);
                                        weta += d__1 * d__1;
                                        i__ += 1;
                                    }
                                    *vsig.offset(j as isize) = T::one() / wsig.sqrt();
                                    *veta.offset(j as isize) = weta.sqrt();
                                    if *vsig.offset(j as isize) < parsig
                                        || *veta.offset(j as isize) > pareta
                                    {
                                        iflag = 0 as libc::c_int;
                                    }
                                    j += 1;
                                }
                                if ibrnch == 1 as libc::c_int || iflag == 1 as libc::c_int {
                                    iz = 1 as libc::c_int;
                                    izdota = iz + *n * *n;
                                    ivmc = izdota + *n;
                                    isdirn = ivmc + mp;
                                    idxnew = isdirn + *n;
                                    ivmd = idxnew + *n;
                                    rc = trstlp(
                                        n,
                                        m,
                                        &mut *a.offset(a_offset as isize),
                                        &mut *con.offset(1 as libc::c_int as isize),
                                        &mut rho,
                                        &mut *dx.offset(1 as libc::c_int as isize),
                                        &mut ifull,
                                        &mut *iact.offset(1 as libc::c_int as isize),
                                        &mut *w.offset(iz as isize),
                                        &mut *w.offset(izdota as isize),
                                        &mut *w.offset(ivmc as isize),
                                        &mut *w.offset(isdirn as isize),
                                        &mut *w.offset(idxnew as isize),
                                        &mut *w.offset(ivmd as isize),
                                    );
                                    if rc as libc::c_int != NLOPT_SUCCESS as libc::c_int {
                                        current_block = 16949430136398296108;
                                        break 'c_6104;
                                    }
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        let mut xi_0: T =
                                            *sim.offset((i__ + np * sim_dim1) as isize);
                                        if xi_0 + *dx.offset(i__ as isize)
                                            > *ub.offset(i__ as isize)
                                        {
                                            *dx.offset(i__ as isize) =
                                                *ub.offset(i__ as isize) - xi_0;
                                        }
                                        if xi_0 + *dx.offset(i__ as isize)
                                            < *lb.offset(i__ as isize)
                                        {
                                            *dx.offset(i__ as isize) =
                                                xi_0 - *lb.offset(i__ as isize);
                                        }
                                        i__ += 1;
                                    }
                                    if ifull == 0 as libc::c_int {
                                        temp = T::zero();
                                        i__1 = *n;
                                        i__ = 1 as libc::c_int;
                                        while i__ <= i__1 {
                                            d__1 = *dx.offset(i__ as isize);
                                            temp += d__1 * d__1;
                                            i__ += 1;
                                        }
                                        if temp < rho * T::from(0.25).unwrap() * rho {
                                            ibrnch = 1 as libc::c_int;
                                            current_block = 17974563553836679504;
                                            continue;
                                        }
                                    }
                                    resnew = T::zero();
                                    *con.offset(mp as isize) = T::zero();
                                    i__1 = mp;
                                    k = 1 as libc::c_int;
                                    while k <= i__1 {
                                        sum = *con.offset(k as isize);
                                        i__2 = *n;
                                        i__ = 1 as libc::c_int;
                                        while i__ <= i__2 {
                                            sum -= *a.offset((i__ + k * a_dim1) as isize)
                                                * *dx.offset(i__ as isize);
                                            i__ += 1;
                                        }
                                        if k < mp {
                                            resnew = if resnew >= sum { resnew } else { sum };
                                        }
                                        k += 1;
                                    }
                                    barmu = T::zero();
                                    prerec =
                                        *datmat.offset((*mpp + np * datmat_dim1) as isize) - resnew;
                                    if prerec > T::zero() {
                                        barmu = sum / prerec;
                                    }
                                    if !(parmu < barmu * T::from(1.5).unwrap()) {
                                        break;
                                    }
                                    parmu = barmu * T::from(2.0).unwrap();
                                    if *iprint >= 2 as libc::c_int {
                                        fprintf(
                                            Io::stderr,
                                            &format!("cobyla: increase in PARMU to {}", parmu),
                                        );
                                    }
                                    phi = *datmat.offset((mp + np * datmat_dim1) as isize)
                                        + parmu
                                            * *datmat.offset((*mpp + np * datmat_dim1) as isize);
                                    i__1 = *n;
                                    j = 1 as libc::c_int;
                                    loop {
                                        if !(j <= i__1) {
                                            break 'c_6122;
                                        }
                                        temp = *datmat.offset((mp + j * datmat_dim1) as isize)
                                            + parmu
                                                * *datmat.offset((*mpp + j * datmat_dim1) as isize);
                                        if temp < phi {
                                            current_block = 16207618807156029286;
                                            break;
                                        }
                                        if temp == phi && parmu == T::zero() {
                                            if *datmat.offset((*mpp + j * datmat_dim1) as isize)
                                                < *datmat.offset((*mpp + np * datmat_dim1) as isize)
                                            {
                                                current_block = 16207618807156029286;
                                                break;
                                            }
                                        }
                                        j += 1;
                                    }
                                } else {
                                    jdrop = 0 as libc::c_int;
                                    temp = pareta;
                                    i__1 = *n;
                                    j = 1 as libc::c_int;
                                    while j <= i__1 {
                                        if *veta.offset(j as isize) > temp {
                                            jdrop = j;
                                            temp = *veta.offset(j as isize);
                                        }
                                        j += 1;
                                    }
                                    if jdrop == 0 as libc::c_int {
                                        i__1 = *n;
                                        j = 1 as libc::c_int;
                                        while j <= i__1 {
                                            if *vsig.offset(j as isize) < temp {
                                                jdrop = j;
                                                temp = *vsig.offset(j as isize);
                                            }
                                            j += 1;
                                        }
                                    }
                                    temp = gamma_ * rho * *vsig.offset(jdrop as isize);
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        *dx.offset(i__ as isize) =
                                            temp * *simi.offset((jdrop + i__ * simi_dim1) as isize);
                                        i__ += 1;
                                    }
                                    cvmaxp = T::zero();
                                    cvmaxm = T::zero();
                                    i__1 = mp;
                                    k = 1 as libc::c_int;
                                    while k <= i__1 {
                                        sum = T::zero();
                                        i__2 = *n;
                                        i__ = 1 as libc::c_int;
                                        while i__ <= i__2 {
                                            sum += *a.offset((i__ + k * a_dim1) as isize)
                                                * *dx.offset(i__ as isize);
                                            i__ += 1;
                                        }
                                        if k < mp {
                                            temp = *datmat.offset((k + np * datmat_dim1) as isize);
                                            d__1 = cvmaxp;
                                            d__2 = -sum - temp;
                                            cvmaxp = if d__1 >= d__2 { d__1 } else { d__2 };
                                            d__1 = cvmaxm;
                                            d__2 = sum - temp;
                                            cvmaxm = if d__1 >= d__2 { d__1 } else { d__2 };
                                        }
                                        k += 1;
                                    }
                                    dxsign = T::one();
                                    if parmu * (cvmaxp - cvmaxm) > sum + sum {
                                        dxsign = T::from(-1.0).unwrap();
                                    }
                                    temp = T::zero();
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        *dx.offset(i__ as isize) = dxsign
                                            * *dx.offset(i__ as isize)
                                            * lcg_urand(
                                                &mut seed,
                                                T::from(0.01).unwrap(),
                                                T::one(),
                                            );
                                        let mut xi: T =
                                            *sim.offset((i__ + np * sim_dim1) as isize);
                                        loop {
                                            if xi + *dx.offset(i__ as isize)
                                                > *ub.offset(i__ as isize)
                                            {
                                                *dx.offset(i__ as isize) =
                                                    -*dx.offset(i__ as isize);
                                            }
                                            if !(xi + *dx.offset(i__ as isize)
                                                < *lb.offset(i__ as isize))
                                            {
                                                break;
                                            }
                                            if xi - *dx.offset(i__ as isize)
                                                <= *ub.offset(i__ as isize)
                                            {
                                                *dx.offset(i__ as isize) =
                                                    -*dx.offset(i__ as isize);
                                                break;
                                            } else {
                                                *dx.offset(i__ as isize) = *dx.offset(i__ as isize) * T::from(0.5).unwrap();
                                            }
                                        }
                                        *sim.offset((i__ + jdrop * sim_dim1) as isize) =
                                            *dx.offset(i__ as isize);
                                        temp += *simi.offset((jdrop + i__ * simi_dim1) as isize)
                                            * *dx.offset(i__ as isize);
                                        i__ += 1;
                                    }
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        *simi.offset((jdrop + i__ * simi_dim1) as isize) /= temp;
                                        i__ += 1;
                                    }
                                    i__1 = *n;
                                    j = 1 as libc::c_int;
                                    while j <= i__1 {
                                        if j != jdrop {
                                            temp = T::zero();
                                            i__2 = *n;
                                            i__ = 1 as libc::c_int;
                                            while i__ <= i__2 {
                                                temp += *simi
                                                    .offset((j + i__ * simi_dim1) as isize)
                                                    * *dx.offset(i__ as isize);
                                                i__ += 1;
                                            }
                                            i__2 = *n;
                                            i__ = 1 as libc::c_int;
                                            while i__ <= i__2 {
                                                *simi.offset((j + i__ * simi_dim1) as isize) -= temp
                                                    * *simi
                                                        .offset((jdrop + i__ * simi_dim1) as isize);
                                                i__ += 1;
                                            }
                                        }
                                        *x.offset(j as isize) = *sim
                                            .offset((j + np * sim_dim1) as isize)
                                            + *dx.offset(j as isize);
                                        j += 1;
                                    }
                                    continue 'c_6104;
                                }
                            }
                        }
                    }
                }
                prerem = parmu * prerec - sum;
                i__1 = *n;
                i__ = 1 as libc::c_int;
                while i__ <= i__1 {
                    *x.offset(i__ as isize) =
                        *sim.offset((i__ + np * sim_dim1) as isize) + *dx.offset(i__ as isize);
                    i__ += 1;
                }
                ibrnch = 1 as libc::c_int;
            }
        }
    }
    match current_block {
        16949430136398296108 => {
            i__1 = *n;
            i__ = 1 as libc::c_int;
            while i__ <= i__1 {
                *x.offset(i__ as isize) = *sim.offset((i__ + np * sim_dim1) as isize);
                i__ += 1;
            }
            f = *datmat.offset((mp + np * datmat_dim1) as isize);
            resmax = *datmat.offset((*mpp + np * datmat_dim1) as isize);
        }
        _ => {}
    }
    *minf = f;
    if *iprint >= 1 as libc::c_int {
        fprintf(
            Io::stderr,
            &format!(
                "cobyla: NFVALS = {}, F ={}, MAXCV ={}\n",
                *(*stop).nevals_p,
                f,
                resmax
            ),
        );
        i__1 = iptem;
        fprintf(Io::stderr, "cobyla: X =");
        i__ = 1 as libc::c_int;
        while i__ <= i__1 {
            if i__ > 1 as libc::c_int {
                fprintf(Io::stderr, "  ");
            }
            fprintf(Io::stderr, &format!("{}", *x.offset(i__ as isize)));
            i__ += 1;
        }
        if iptem < *n {
            i__1 = *n;
            i__ = iptemp;
            while i__ <= i__1 {
                if (i__ - 1 as libc::c_int) % 4 as libc::c_int == 0 {
                    fprintf(Io::stderr, "\ncobyla:  ");
                }
                fprintf(Io::stderr, &format!("{}", *x.offset(i__ as isize)));
                i__ += 1;
            }
        }
        fprintf(Io::stderr, "");
    }
    rc
}
unsafe fn trstlp<T: Float + std::fmt::Display + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign>(
    mut n: *mut libc::c_int,
    mut m: *mut libc::c_int,
    mut a: *mut T,
    mut b: *mut T,
    mut rho: *mut T,
    mut dx: *mut T,
    mut ifull: *mut libc::c_int,
    mut iact: *mut libc::c_int,
    mut z__: *mut T,
    mut zdota: *mut T,
    mut vmultc: *mut T,
    mut sdirn: *mut T,
    mut dxnew: *mut T,
    mut vmultd: *mut T,
) -> nlopt_result {
    let mut current_block: u64;
    let mut a_dim1: libc::c_int = 0;
    let mut a_offset: libc::c_int = 0;
    let mut z_dim1: libc::c_int = 0;
    let mut z_offset: libc::c_int = 0;
    let mut i__1: libc::c_int = 0;
    let mut i__2: libc::c_int = 0;
    let mut d__1: T = T::zero();
    let mut d__2: T = T::zero();
    let mut alpha: T = T::zero();
    let mut tempa: T = T::zero();
    let mut beta: T = T::zero();
    let mut optnew: T = T::zero();
    let mut stpful: T = T::zero();
    let mut sum: T = T::zero();
    let mut tot: T = T::zero();
    let mut acca: T = T::zero();
    let mut accb: T = T::zero();
    let mut ratio: T = T::zero();
    let mut vsave: T = T::zero();
    let mut zdotv: T = T::zero();
    let mut zdotw: T = T::zero();
    let mut dd: T = T::zero();
    let mut sd: T = T::zero();
    let mut sp: T = T::zero();
    let mut ss: T = T::zero();
    let mut resold: T = T::zero();
    let mut zdvabs: T = T::zero();
    let mut zdwabs: T = T::zero();
    let mut sumabs: T = T::zero();
    let mut resmax: T = T::zero();
    let mut optold: T = T::zero();
    let mut spabs: T = T::zero();
    let mut temp: T = T::zero();
    let mut step: T = T::zero();
    let mut icount: libc::c_int = 0;
    let mut i__: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut isave: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut kl: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut kw: libc::c_int = 0;
    let mut nact: libc::c_int = 0;
    let mut icon: libc::c_int = 0 as libc::c_int;
    let mut mcon: libc::c_int = 0;
    let mut nactx: libc::c_int = 0 as libc::c_int;
    z_dim1 = *n;
    z_offset = 1 as libc::c_int + z_dim1 * 1 as libc::c_int;
    z__ = z__.offset(-(z_offset as isize));
    a_dim1 = *n;
    a_offset = 1 as libc::c_int + a_dim1 * 1 as libc::c_int;
    a = a.offset(-(a_offset as isize));
    b = b.offset(-1);
    dx = dx.offset(-1);
    iact = iact.offset(-1);
    zdota = zdota.offset(-1);
    vmultc = vmultc.offset(-1);
    sdirn = sdirn.offset(-1);
    dxnew = dxnew.offset(-1);
    vmultd = vmultd.offset(-1);
    *ifull = 1 as libc::c_int;
    mcon = *m;
    nact = 0 as libc::c_int;
    resmax = T::zero();
    i__1 = *n;
    i__ = 1 as libc::c_int;
    while i__ <= i__1 {
        i__2 = *n;
        j = 1 as libc::c_int;
        while j <= i__2 {
            *z__.offset((i__ + j * z_dim1) as isize) = T::zero();
            j += 1;
        }
        *z__.offset((i__ + i__ * z_dim1) as isize) = T::one();
        *dx.offset(i__ as isize) = T::zero();
        i__ += 1;
    }
    if *m >= 1 as libc::c_int {
        i__1 = *m;
        k = 1 as libc::c_int;
        while k <= i__1 {
            if *b.offset(k as isize) > resmax {
                resmax = *b.offset(k as isize);
                icon = k;
            }
            k += 1;
        }
        i__1 = *m;
        k = 1 as libc::c_int;
        while k <= i__1 {
            *iact.offset(k as isize) = k;
            *vmultc.offset(k as isize) = resmax - *b.offset(k as isize);
            k += 1;
        }
    }
    if resmax == T::zero() {
        current_block = 11188143500741601598;
    } else {
        i__1 = *n;
        i__ = 1 as libc::c_int;
        while i__ <= i__1 {
            *sdirn.offset(i__ as isize) = T::zero();
            i__ += 1;
        }
        current_block = 13859042411183768487;
    }
    'c_8601: loop {
        match current_block {
            11188143500741601598 => {
                mcon = *m + 1 as libc::c_int;
                icon = mcon;
                *iact.offset(mcon as isize) = mcon;
                *vmultc.offset(mcon as isize) = T::zero();
                current_block = 13859042411183768487;
            }
            _ => {
                optold = T::zero();
                icount = 0 as libc::c_int;
                loop {
                    if mcon == *m {
                        optnew = resmax;
                    } else {
                        optnew = T::zero();
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            optnew -= *dx.offset(i__ as isize)
                                * *a.offset((i__ + mcon * a_dim1) as isize);
                            i__ += 1;
                        }
                    }
                    if icount == 0 as libc::c_int || optnew < optold {
                        optold = optnew;
                        nactx = nact;
                        icount = 3 as libc::c_int;
                    } else if nact > nactx {
                        nactx = nact;
                        icount = 3 as libc::c_int;
                    } else {
                        icount -= 1;
                        if icount == 0 as libc::c_int {
                            break;
                        }
                    }
                    if icon <= nact {
                        if icon < nact {
                            isave = *iact.offset(icon as isize);
                            vsave = *vmultc.offset(icon as isize);
                            k = icon;
                            loop {
                                kp = k + 1 as libc::c_int;
                                kk = *iact.offset(kp as isize);
                                sp = T::zero();
                                i__1 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    sp += *z__.offset((i__ + k * z_dim1) as isize)
                                        * *a.offset((i__ + kk * a_dim1) as isize);
                                    i__ += 1;
                                }
                                d__1 = *zdota.offset(kp as isize);
                                temp = (sp * sp + d__1 * d__1).sqrt();
                                alpha = *zdota.offset(kp as isize) / temp;
                                beta = sp / temp;
                                *zdota.offset(kp as isize) = alpha * *zdota.offset(k as isize);
                                *zdota.offset(k as isize) = temp;
                                i__1 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    temp = alpha * *z__.offset((i__ + kp * z_dim1) as isize)
                                        + beta * *z__.offset((i__ + k * z_dim1) as isize);
                                    *z__.offset((i__ + kp * z_dim1) as isize) = alpha
                                        * *z__.offset((i__ + k * z_dim1) as isize)
                                        - beta * *z__.offset((i__ + kp * z_dim1) as isize);
                                    *z__.offset((i__ + k * z_dim1) as isize) = temp;
                                    i__ += 1;
                                }
                                *iact.offset(k as isize) = kk;
                                *vmultc.offset(k as isize) = *vmultc.offset(kp as isize);
                                k = kp;
                                if !(k < nact) {
                                    break;
                                }
                            }
                            *iact.offset(k as isize) = isave;
                            *vmultc.offset(k as isize) = vsave;
                        }
                        nact -= 1;
                        if mcon > *m {
                            current_block = 15623375721314334080;
                        } else {
                            temp = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                temp += *sdirn.offset(i__ as isize)
                                    * *z__.offset(
                                        (i__ + (nact + 1 as libc::c_int) * z_dim1) as isize,
                                    );
                                i__ += 1;
                            }
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                *sdirn.offset(i__ as isize) -= temp
                                    * *z__.offset(
                                        (i__ + (nact + 1 as libc::c_int) * z_dim1) as isize,
                                    );
                                i__ += 1;
                            }
                            current_block = 9824026811267781195;
                        }
                    } else {
                        kk = *iact.offset(icon as isize);
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            *dxnew.offset(i__ as isize) = *a.offset((i__ + kk * a_dim1) as isize);
                            i__ += 1;
                        }
                        tot = T::zero();
                        k = *n;
                        while k > nact {
                            sp = T::zero();
                            spabs = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                temp = *z__.offset((i__ + k * z_dim1) as isize)
                                    * *dxnew.offset(i__ as isize);
                                sp += temp;
                                spabs += (temp).abs();
                                i__ += 1;
                            }
                            acca = spabs + (sp).abs() * T::from(0.1).unwrap();
                            accb = spabs + (sp).abs() * T::from(0.2).unwrap();
                            if spabs >= acca || acca >= accb {
                                sp = T::zero();
                            }
                            if tot == T::zero() {
                                tot = sp;
                            } else {
                                kp = k + 1 as libc::c_int;
                                temp = (sp * sp + tot * tot).sqrt();
                                alpha = sp / temp;
                                beta = tot / temp;
                                tot = temp;
                                i__1 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    temp = alpha * *z__.offset((i__ + k * z_dim1) as isize)
                                        + beta * *z__.offset((i__ + kp * z_dim1) as isize);
                                    *z__.offset((i__ + kp * z_dim1) as isize) = alpha
                                        * *z__.offset((i__ + kp * z_dim1) as isize)
                                        - beta * *z__.offset((i__ + k * z_dim1) as isize);
                                    *z__.offset((i__ + k * z_dim1) as isize) = temp;
                                    i__ += 1;
                                }
                            }
                            k -= 1;
                        }
                        if tot != T::zero() {
                            nact += 1;
                            *zdota.offset(nact as isize) = tot;
                            *vmultc.offset(icon as isize) = *vmultc.offset(nact as isize);
                            *vmultc.offset(nact as isize) = T::zero();
                        } else {
                            ratio = T::from(-1.0).unwrap();
                            k = nact;
                            loop {
                                zdotv = T::zero();
                                zdvabs = T::zero();
                                i__1 = *n;
                                i__ = 1 as libc::c_int;
                                while i__ <= i__1 {
                                    temp = *z__.offset((i__ + k * z_dim1) as isize)
                                        * *dxnew.offset(i__ as isize);
                                    zdotv += temp;
                                    zdvabs += (temp).abs();
                                    i__ += 1;
                                }
                                acca = zdvabs + (zdotv).abs() * T::from(0.1).unwrap();
                                accb = zdvabs + (zdotv).abs() * T::from(0.2).unwrap();
                                if zdvabs < acca && acca < accb {
                                    temp = zdotv / *zdota.offset(k as isize);
                                    if temp > T::zero() && *iact.offset(k as isize) <= *m {
                                        tempa = *vmultc.offset(k as isize) / temp;
                                        if ratio < T::zero() || tempa < ratio {
                                            ratio = tempa;
                                        }
                                    }
                                    if k >= 2 as libc::c_int {
                                        kw = *iact.offset(k as isize);
                                        i__1 = *n;
                                        i__ = 1 as libc::c_int;
                                        while i__ <= i__1 {
                                            *dxnew.offset(i__ as isize) -=
                                                temp * *a.offset((i__ + kw * a_dim1) as isize);
                                            i__ += 1;
                                        }
                                    }
                                    *vmultd.offset(k as isize) = temp;
                                } else {
                                    *vmultd.offset(k as isize) = T::zero();
                                }
                                k -= 1;
                                if !(k > 0 as libc::c_int) {
                                    break;
                                }
                            }
                            if ratio < T::zero() {
                                break;
                            }
                            i__1 = nact;
                            k = 1 as libc::c_int;
                            while k <= i__1 {
                                d__1 = T::zero();
                                d__2 =
                                    *vmultc.offset(k as isize) - ratio * *vmultd.offset(k as isize);
                                *vmultc.offset(k as isize) = if d__1 >= d__2 { d__1 } else { d__2 };
                                k += 1;
                            }
                            if icon < nact {
                                isave = *iact.offset(icon as isize);
                                vsave = *vmultc.offset(icon as isize);
                                k = icon;
                                loop {
                                    kp = k + 1 as libc::c_int;
                                    kw = *iact.offset(kp as isize);
                                    sp = T::zero();
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        sp += *z__.offset((i__ + k * z_dim1) as isize)
                                            * *a.offset((i__ + kw * a_dim1) as isize);
                                        i__ += 1;
                                    }
                                    d__1 = *zdota.offset(kp as isize);
                                    temp = (sp * sp + d__1 * d__1).sqrt();
                                    alpha = *zdota.offset(kp as isize) / temp;
                                    beta = sp / temp;
                                    *zdota.offset(kp as isize) = alpha * *zdota.offset(k as isize);
                                    *zdota.offset(k as isize) = temp;
                                    i__1 = *n;
                                    i__ = 1 as libc::c_int;
                                    while i__ <= i__1 {
                                        temp = alpha * *z__.offset((i__ + kp * z_dim1) as isize)
                                            + beta * *z__.offset((i__ + k * z_dim1) as isize);
                                        *z__.offset((i__ + kp * z_dim1) as isize) = alpha
                                            * *z__.offset((i__ + k * z_dim1) as isize)
                                            - beta * *z__.offset((i__ + kp * z_dim1) as isize);
                                        *z__.offset((i__ + k * z_dim1) as isize) = temp;
                                        i__ += 1;
                                    }
                                    *iact.offset(k as isize) = kw;
                                    *vmultc.offset(k as isize) = *vmultc.offset(kp as isize);
                                    k = kp;
                                    if !(k < nact) {
                                        break;
                                    }
                                }
                                *iact.offset(k as isize) = isave;
                                *vmultc.offset(k as isize) = vsave;
                            }
                            temp = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                temp += *z__.offset((i__ + nact * z_dim1) as isize)
                                    * *a.offset((i__ + kk * a_dim1) as isize);
                                i__ += 1;
                            }
                            if temp == T::zero() {
                                break;
                            }
                            *zdota.offset(nact as isize) = temp;
                            *vmultc.offset(icon as isize) = T::zero();
                            *vmultc.offset(nact as isize) = ratio;
                        }
                        *iact.offset(icon as isize) = *iact.offset(nact as isize);
                        *iact.offset(nact as isize) = kk;
                        if mcon > *m && kk != mcon {
                            k = nact - 1 as libc::c_int;
                            sp = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                sp += *z__.offset((i__ + k * z_dim1) as isize)
                                    * *a.offset((i__ + kk * a_dim1) as isize);
                                i__ += 1;
                            }
                            d__1 = *zdota.offset(nact as isize);
                            temp = (sp * sp + d__1 * d__1).sqrt();
                            alpha = *zdota.offset(nact as isize) / temp;
                            beta = sp / temp;
                            *zdota.offset(nact as isize) = alpha * *zdota.offset(k as isize);
                            *zdota.offset(k as isize) = temp;
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                temp = alpha * *z__.offset((i__ + nact * z_dim1) as isize)
                                    + beta * *z__.offset((i__ + k * z_dim1) as isize);
                                *z__.offset((i__ + nact * z_dim1) as isize) = alpha
                                    * *z__.offset((i__ + k * z_dim1) as isize)
                                    - beta * *z__.offset((i__ + nact * z_dim1) as isize);
                                *z__.offset((i__ + k * z_dim1) as isize) = temp;
                                i__ += 1;
                            }
                            *iact.offset(nact as isize) = *iact.offset(k as isize);
                            *iact.offset(k as isize) = kk;
                            temp = *vmultc.offset(k as isize);
                            *vmultc.offset(k as isize) = *vmultc.offset(nact as isize);
                            *vmultc.offset(nact as isize) = temp;
                        }
                        if mcon > *m {
                            current_block = 15623375721314334080;
                        } else {
                            kk = *iact.offset(nact as isize);
                            temp = T::zero();
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                temp += *sdirn.offset(i__ as isize)
                                    * *a.offset((i__ + kk * a_dim1) as isize);
                                i__ += 1;
                            }
                            temp = temp + T::from(-1.0).unwrap();
                            temp = temp / *zdota.offset(nact as isize);
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                *sdirn.offset(i__ as isize) -=
                                    temp * *z__.offset((i__ + nact * z_dim1) as isize);
                                i__ += 1;
                            }
                            current_block = 9824026811267781195;
                        }
                    }
                    match current_block {
                        15623375721314334080 => {
                            temp = T::one() / *zdota.offset(nact as isize);
                            i__1 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__1 {
                                *sdirn.offset(i__ as isize) =
                                    temp * *z__.offset((i__ + nact * z_dim1) as isize);
                                i__ += 1;
                            }
                        }
                        _ => {}
                    }
                    dd = *rho * *rho;
                    sd = T::zero();
                    ss = T::zero();
                    i__1 = *n;
                    i__ = 1 as libc::c_int;
                    while i__ <= i__1 {
                        d__1 = *dx.offset(i__ as isize);
                        if (d__1).abs() >= *rho * T::from(1e-6).unwrap() {
                            d__2 = *dx.offset(i__ as isize);
                            dd = dd - d__2 * d__2;
                        }
                        sd = sd + *dx.offset(i__ as isize) * *sdirn.offset(i__ as isize);
                        d__1 = *sdirn.offset(i__ as isize);
                        ss = ss + d__1 * d__1;
                        i__ += 1;
                    }
                    if dd <= T::zero() {
                        break;
                    }
                    temp = (ss * dd).sqrt();
                    if (sd).abs() >= temp * T::from(1e-6).unwrap() {
                        temp = (ss * dd + sd * sd).sqrt();
                    }
                    stpful = dd / (temp + sd);
                    step = stpful;
                    if mcon == *m {
                        acca = step + resmax * T::from(0.1).unwrap();
                        accb = step + resmax * T::from(0.2).unwrap();
                        if step >= acca || acca >= accb {
                            current_block = 11188143500741601598;
                            continue 'c_8601;
                        }
                        step = if step <= resmax { step } else { resmax };
                    }
                    if nlopt_isinf(step) != 0 {
                        return NLOPT_ROUNDOFF_LIMITED;
                    }
                    i__1 = *n;
                    i__ = 1 as libc::c_int;
                    while i__ <= i__1 {
                        *dxnew.offset(i__ as isize) =
                            *dx.offset(i__ as isize) + step * *sdirn.offset(i__ as isize);
                        i__ += 1;
                    }
                    if mcon == *m {
                        resold = resmax;
                        resmax = T::zero();
                        i__1 = nact;
                        k = 1 as libc::c_int;
                        while k <= i__1 {
                            kk = *iact.offset(k as isize);
                            temp = *b.offset(kk as isize);
                            i__2 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__2 {
                                temp -= *a.offset((i__ + kk * a_dim1) as isize)
                                    * *dxnew.offset(i__ as isize);
                                i__ += 1;
                            }
                            resmax = if resmax >= temp { resmax } else { temp };
                            k += 1;
                        }
                    }
                    k = nact;
                    loop {
                        zdotw = T::zero();
                        zdwabs = T::zero();
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            temp = *z__.offset((i__ + k * z_dim1) as isize)
                                * *dxnew.offset(i__ as isize);
                            zdotw += temp;
                            zdwabs += (temp).abs();
                            i__ += 1;
                        }
                        acca = zdwabs + (zdotw).abs() * T::from(0.1).unwrap();
                        accb = zdwabs + (zdotw).abs() * T::from(0.2).unwrap();
                        if zdwabs >= acca || acca >= accb {
                            zdotw = T::zero();
                        }
                        *vmultd.offset(k as isize) = zdotw / *zdota.offset(k as isize);
                        if !(k >= 2 as libc::c_int) {
                            break;
                        }
                        kk = *iact.offset(k as isize);
                        i__1 = *n;
                        i__ = 1 as libc::c_int;
                        while i__ <= i__1 {
                            *dxnew.offset(i__ as isize) -= *vmultd.offset(k as isize)
                                * *a.offset((i__ + kk * a_dim1) as isize);
                            i__ += 1;
                        }
                        k -= 1;
                    }
                    if mcon > *m {
                        d__1 = T::zero();
                        d__2 = *vmultd.offset(nact as isize);
                        *vmultd.offset(nact as isize) = if d__1 >= d__2 { d__1 } else { d__2 };
                    }
                    i__1 = *n;
                    i__ = 1 as libc::c_int;
                    while i__ <= i__1 {
                        *dxnew.offset(i__ as isize) =
                            *dx.offset(i__ as isize) + step * *sdirn.offset(i__ as isize);
                        i__ += 1;
                    }
                    if mcon > nact {
                        kl = nact + 1 as libc::c_int;
                        i__1 = mcon;
                        k = kl;
                        while k <= i__1 {
                            kk = *iact.offset(k as isize);
                            sum = resmax - *b.offset(kk as isize);
                            d__1 = *b.offset(kk as isize);
                            sumabs = resmax + (d__1).abs();
                            i__2 = *n;
                            i__ = 1 as libc::c_int;
                            while i__ <= i__2 {
                                temp = *a.offset((i__ + kk * a_dim1) as isize)
                                    * *dxnew.offset(i__ as isize);
                                sum += temp;
                                sumabs += (temp).abs();
                                i__ += 1;
                            }
                            acca = sumabs + (sum).abs() * T::from(0.1).unwrap();
                            accb = sumabs + (sum).abs() * T::from(0.2).unwrap();
                            if sumabs >= acca || acca >= accb {
                                sum = T::zero();
                            }
                            *vmultd.offset(k as isize) = sum;
                            k += 1;
                        }
                    }
                    ratio = T::one();
                    icon = 0 as libc::c_int;
                    i__1 = mcon;
                    k = 1 as libc::c_int;
                    while k <= i__1 {
                        if *vmultd.offset(k as isize) < T::zero() {
                            temp = *vmultc.offset(k as isize)
                                / (*vmultc.offset(k as isize) - *vmultd.offset(k as isize));
                            if temp < ratio {
                                ratio = temp;
                                icon = k;
                            }
                        }
                        k += 1;
                    }
                    temp = T::one() - ratio;
                    i__1 = *n;
                    i__ = 1 as libc::c_int;
                    while i__ <= i__1 {
                        *dx.offset(i__ as isize) =
                            temp * *dx.offset(i__ as isize) + ratio * *dxnew.offset(i__ as isize);
                        i__ += 1;
                    }
                    i__1 = mcon;
                    k = 1 as libc::c_int;
                    while k <= i__1 {
                        d__1 = T::zero();
                        d__2 =
                            temp * *vmultc.offset(k as isize) + ratio * *vmultd.offset(k as isize);
                        *vmultc.offset(k as isize) = if d__1 >= d__2 { d__1 } else { d__2 };
                        k += 1;
                    }
                    if mcon == *m {
                        resmax = resold + ratio * (resmax - resold);
                    }
                    if icon > 0 as libc::c_int {
                        continue;
                    }
                    if step == stpful {
                        break 'c_8601;
                    } else {
                        current_block = 11188143500741601598;
                        continue 'c_8601;
                    }
                }
                if mcon == *m {
                    current_block = 11188143500741601598;
                    continue;
                }
                *ifull = 0 as libc::c_int;
                break;
            }
        }
    }
    return NLOPT_SUCCESS;
}
