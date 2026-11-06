use crate::state::{StackItem, State};
use crate::value::{vc, Value};

pub const OPS: [&str; 21] = [
    "+", "-", "++", "--", "*", "/", "<<", ">>", "|", "&", "^", "%", "!", "~", ">>>>", ">>>",
    "<<<", "$>", "$>=", "$<", "$<=",
];

pub const SIZE: u64 = 64;

#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Trap,
    Interrupt,
    Syscall,
    PcAddress,
    If,
    Else,
    EndIf,
    Compare,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    UnsignedLessThan,
    UnsignedLessThanEq,
    UnsignedGreaterThan,
    UnsignedGreaterThanEq,
    LeftShift,
    LogicalRightShift,
    RightShift,
    LeftRotation,
    RightRotation,
    SignExtend,
    And,
    Or,
    Xor,
    Add,
    Subtract,
    Multiply,
    LongMultiply,
    Divide,
    LongDivide,
    Modulo,
    LongModulo,
    SignedDivide,
    SignedModulo,
    Not,
    Increment,
    Decrement,
    Equal,
    WeakEqual,
    Peek(usize),
    Poke(usize),
    PeekBits,
    PokeBits,
    PeekSize,
    PokeSize,
    PeekMany,
    PokeMany,
    PopCount,
    CountLeadingZeros,
    Ceiling,
    Floor,
    Round,
    SquareRoot,
    DoubleToInt,
    SignedToDouble,
    UnsignedToDouble,
    DoubleToFloat,
    FloatToDouble,
    FloatCompare,
    FloatNotEqual,
    FloatLessThan,
    FloatLessThanEq,
    FloatAdd,
    FloatSubtract,
    FloatMultiply,
    FloatDivide,
    FloatMultiplyAdd,
    NaN,
    FloatNegate,
    AddressStore,
    AddressRestore,
    Swap,
    Rotate,
    Pick,
    ReversePick,
    Pop,
    Duplicate,
    Number,
    Clear,
    Break,
    Repeat,
    GoTo,
    PrintStack,
    ToDo,
    NoOperation,
    SetJumpTarget,
    SetJumpTargetSet,
    SetDelay,
    Bits,
    Command,
    RegisterAlias,

    Print,      // Tool for cli hooks
    PrintDebug, // Tool for cli hooks
    Backtrace,  // Tool for cli hooks
    Constrain,
    ConstrainEqual,
    ConstraintPush,
    ConstraintPop,
    Terminate,
    Discard,

    // flag ops
    Zero,
    Carry,
    Borrow,
    Parity,
    Overflow,
    // SubOverflow,
    // i forget these
    S,
    Ds,
    JumpTarget,
    Js,
    R,

    Unknown,
}

impl Operations {
    pub fn from_string(s: &str) -> Self {
        match s {
            "TRAP" => Operations::Trap,
            "$" => Operations::Interrupt,
            "()" => Operations::Syscall,
            "$$" => Operations::PcAddress, // this is whack
            "?{" => Operations::If,
            "}{" => Operations::Else,
            "}" => Operations::EndIf,
            "==" => Operations::Compare,
            "<" => Operations::LessThan,
            "<=" => Operations::LessThanEq,
            ">" => Operations::GreaterThan,
            ">=" => Operations::GreaterThanEq,
            "$<" => Operations::UnsignedLessThan,
            "$<=" => Operations::UnsignedLessThanEq,
            "$>" => Operations::UnsignedGreaterThan,
            "$>=" => Operations::UnsignedGreaterThanEq,
            "<<" => Operations::LeftShift,
            "LSL" => Operations::LeftShift,
            "ASL" => Operations::LeftShift,
            ">>" => Operations::LogicalRightShift,
            "LSR" => Operations::LogicalRightShift,
            ">>>>" => Operations::RightShift,
            "ASR" => Operations::RightShift,
            "<<<" => Operations::LeftRotation,
            "ROL" => Operations::LeftRotation,
            ">>>" => Operations::RightRotation,
            "ROR" => Operations::RightRotation,
            "~" => Operations::SignExtend,
            "SIGN" => Operations::SignExtend,
            "&" => Operations::And,
            "|" => Operations::Or,
            "^" => Operations::Xor,
            "+" => Operations::Add,
            "-" => Operations::Subtract,
            "*" => Operations::Multiply,
            "L*" => Operations::LongMultiply,
            "/" => Operations::Divide,
            "L/" => Operations::LongDivide,
            "%" => Operations::Modulo,
            "L%" => Operations::LongModulo,
            "~/" => Operations::SignedDivide,
            "~%" => Operations::SignedModulo,
            "!" => Operations::Not,
            "++" => Operations::Increment,
            "--" => Operations::Decrement,
            "=" => Operations::Equal,
            ":=" => Operations::WeakEqual,
            "[1]" => Operations::Peek(1),
            "[2]" => Operations::Peek(2),
            "[3]" => Operations::Peek(3),
            "[4]" => Operations::Peek(4),
            "[8]" => Operations::Peek(8),
            "[16]" => Operations::Peek(16),
            "=[1]" => Operations::Poke(1),
            "=[2]" => Operations::Poke(2),
            "=[3]" => Operations::Poke(3),
            "=[4]" => Operations::Poke(4),
            "=[8]" => Operations::Poke(8),
            "=[16]" => Operations::Poke(16),
            "=[]" => Operations::PokeBits,
            "[]" => Operations::PeekBits,
            "=[n]" => Operations::PokeSize,
            "[n]" => Operations::PeekSize,
            "=[*]" => Operations::PokeMany,
            "[*]" => Operations::PeekMany,
            "POPCNT" => Operations::PopCount,
            "POPCOUNT" => Operations::PopCount,
            "CLZ" => Operations::CountLeadingZeros,
            "CEIL" => Operations::Ceiling,
            "FLOOR" => Operations::Floor,
            "ROUND" => Operations::Round,
            "SQRT" => Operations::SquareRoot,
            "D2I" => Operations::DoubleToInt,
            "I2D" => Operations::SignedToDouble,
            "S2D" => Operations::SignedToDouble,
            "U2D" => Operations::UnsignedToDouble,
            "F2D" => Operations::FloatToDouble,
            "D2F" => Operations::DoubleToFloat,
            "F+" => Operations::FloatAdd,
            "F-" => Operations::FloatSubtract,
            "F*" => Operations::FloatMultiply,
            "F/" => Operations::FloatDivide,
            "F*+" => Operations::FloatMultiplyAdd,
            "F==" => Operations::FloatCompare,
            "F!=" => Operations::FloatNotEqual,
            "F<" => Operations::FloatLessThan,
            "F<=" => Operations::FloatLessThanEq,
            "NAN" => Operations::NaN,
            "-F" => Operations::FloatNegate,
            "SWAP" => Operations::Swap,
            "ROT" => Operations::Rotate,
            "PICK" => Operations::Pick,
            "RPICK" => Operations::ReversePick,
            "POP" => Operations::Pop,
            "DUP" => Operations::Duplicate,
            "NUM" => Operations::Number,
            "CLEAR" => Operations::Clear,
            "BREAK" => Operations::Break,
            "REPEAT" => Operations::Repeat,
            "GOTO" => Operations::GoTo,
            "STACK" => Operations::PrintStack,
            "TODO" => Operations::ToDo,
            "SETJT" => Operations::SetJumpTarget,
            "SETJTS" => Operations::SetJumpTargetSet,
            "SETD" => Operations::SetDelay,
            "BITS" => Operations::Bits,
            "#!" => Operations::Command,
            "r=" => Operations::RegisterAlias,
            "" => Operations::NoOperation,

            // hax for use in the cli / plugin
            "." => Operations::Print,
            ".." => Operations::PrintDebug,
            "BT" => Operations::Backtrace,
            "_" => Operations::Constrain,
            "_=" => Operations::ConstrainEqual,
            "_+" => Operations::ConstraintPush,
            "_-" => Operations::ConstraintPop,
            "!!" => Operations::Terminate, // state.set_break()
            "!_" => Operations::Discard,   // state.set_inactive()

            "$z" => Operations::Zero,
            "$c" => Operations::Carry,
            "$b" => Operations::Borrow,
            "$p" => Operations::Parity,
            "$o" => Operations::Overflow,
            //"$so" => Operations::SubOverflow,
            "$s" => Operations::S,
            "$ds" => Operations::Ds,
            "$jt" => Operations::JumpTarget,
            "$js" => Operations::Js,
            "$r" => Operations::R,
            _ => Operations::Unknown,
        }
    }
}

/// size of the top stack item, literals are as wide as asm.bits (like r_esil_get_parm_size)
#[inline]
pub fn get_size(state: &mut State) -> u32 {
    match state.stack.last() {
        Some(StackItem::StackRegister(index)) => state.registers.indexes[*index].reg_info.size as u32,
        _ => state.memory.bits as u32,
    }
}

/// pop a value and the size of the register it came from, if any
#[inline]
pub fn pop_value_reg_size(state: &mut State) -> (Value, Option<usize>) {
    let size = match state.stack.last() {
        Some(StackItem::StackRegister(index)) => {
            Some(state.registers.indexes[*index].reg_info.size as usize)
        }
        _ => None,
    };
    // r2 ignores registers wider than 255 bits here
    let size = size.filter(|sz| *sz > 0 && *sz <= 255);
    (pop_value(state, false, false), size)
}

/// pop dst and src and set the flag state like r2's pushnums
#[inline]
pub fn pop_compare_args(state: &mut State) -> (Value, Value, usize) {
    let (dst, dst_sz) = pop_value_reg_size(state);
    let (src, src_sz) = pop_value_reg_size(state);
    let sz = dst_sz.or(src_sz).unwrap_or(64);
    state.esil.current = dst.to_owned() - src.to_owned();
    state.esil.previous = dst.to_owned();
    state.esil.last_sz = sz;
    (dst, src, sz)
}

/// prepare a value for a signed comparison of `sz` bits (like r2's signed_compare_gt)
#[inline]
pub fn signed_sized(value: Value, sz: usize) -> Value {
    match sz {
        1 => value & vc(1),
        8 | 16 | 32 => value.sext(&vc(sz as u64)),
        _ => value,
    }
}

/// division and modulo by zero push 0 like r2 does
#[inline]
pub fn div_by_zero_checked(state: &mut State, dst: Value, src: Value, op: fn(Value, Value) -> Value) -> Value {
    match &src {
        Value::Concrete(0, t) => Value::Concrete(0, dst.get_taint() | t),
        Value::Concrete(_, _) => op(dst, src),
        Value::Symbolic(_, t) => {
            let zero = Value::Concrete(0, dst.get_taint() | t);
            let is_zero = src.eq(&vc(0));
            let res = op(dst, src);
            state.solver.conditional(&is_zero, &zero, &res)
        }
    }
}

/// r2 float binops propagate the NaN of src first, then dst
#[inline]
pub fn float_nan_or(s: f64, d: f64, res: f64) -> f64 {
    if s.is_nan() {
        s
    } else if d.is_nan() {
        d
    } else {
        res
    }
}

pub fn half_to_f64(h: u16) -> f64 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = ((h >> 10) & 0x1f) as i32;
    let frac = (h & 0x3ff) as f64;
    match exp {
        0 => sign * frac * 2f64.powi(-24),
        0x1f if frac == 0.0 => sign * f64::INFINITY,
        0x1f => f64::NAN,
        _ => sign * (1.0 + frac / 1024.0) * 2f64.powi(exp - 15),
    }
}

pub fn f64_to_half(v: f64) -> u16 {
    let sign: u16 = if v.is_sign_negative() { 0x8000 } else { 0 };
    let a = v.abs();
    if v.is_nan() {
        return 0x7e00;
    } else if a >= 65520.0 {
        return sign | 0x7c00;
    } else if a < 2f64.powi(-14) {
        // subnormal, rounding up to 0x400 yields the smallest normal
        return sign | (a * 2f64.powi(24)).round_ties_even() as u16;
    }
    let mut e = a.log2().floor() as i32;
    let mut m = a / 2f64.powi(e);
    if m >= 2.0 {
        e += 1;
        m /= 2.0;
    } else if m < 1.0 {
        e -= 1;
        m *= 2.0;
    }
    let mut frac = ((m - 1.0) * 1024.0).round_ties_even() as u16;
    if frac == 1024 {
        frac = 0;
        e += 1;
    }
    if e > 15 {
        return sign | 0x7c00;
    }
    sign | (((e + 15) as u16) << 10) | frac
}

/// `value` if the current esil condition holds, `prev` otherwise
#[inline]
pub fn conditional_set(state: &State, value: Value, prev: Value) -> Value {
    match &state.condition {
        Some(cond) => state
            .solver
            .conditional(&Value::Symbolic(cond.to_owned(), 0), &value, &prev),
        None => value,
    }
}

/// write `value` to memory and set the flag state like r2's esil_poke_n
pub fn do_poke(state: &mut State, addr: Value, value: Value, n: usize) {
    let prev = if state.condition.is_some() {
        state.memory_read_value(&addr, n)
    } else {
        state.memory.read_sym(&addr, n, &mut state.solver)
    };

    if let Some(cond) = &state.condition.to_owned() {
        let cond_value = state
            .solver
            .conditional(&Value::Symbolic(cond.to_owned(), 0), &value, &prev);
        state.memory_write_value(&addr, &cond_value, n);
    } else {
        state.memory_write_value(&addr, &value, n);
    }

    state.esil.previous = prev;
    state.esil.current = value;
    state.esil.last_sz = 8 * n;
}

/// read memory and push it, peeks only update the last size like r2's esil_peek_n
pub fn do_peek(state: &mut State, addr: Value, n: usize) {
    let val = state.memory_read_value(&addr, n);
    push_value(state, val);
    state.esil.last_sz = 8 * n;
}

#[inline]
pub fn pop_bv(state: &mut State, n: u32) -> Value {
    let value = pop_value(state, false, false);
    Value::Symbolic(state.solver.to_bv(&value, n), value.get_taint())
}

//#[inline]
pub fn pop_value(state: &mut State, set_size: bool, sign_ext: bool) -> Value {
    let item = state.stack.pop().unwrap_or_default();

    let value = match item {
        StackItem::StackValue(val) => val,
        StackItem::StackRegister(index) => {
            if set_size {
                let reg = &state.registers.indexes[index];
                state.esil.last_sz = reg.reg_info.size as usize;
            }
            state.registers.get_value(index)
        }
    };

    match &value {
        Value::Concrete(_v, _t) => value,
        Value::Symbolic(ov, t) => {
            // check const and fits in u64
            if ov.is_const() && ov.get_width() <= 64 {
                Value::Concrete(ov.as_u64().unwrap(), *t)
            } else {
                let v = ov; //state.translate(&ov).unwrap();
                let szdiff = SIZE as i32 - v.get_width() as i32;
                if szdiff > 0 {
                    if sign_ext {
                        Value::Symbolic(v.sext(szdiff as u32), *t)
                    } else {
                        Value::Symbolic(v.uext(szdiff as u32), *t)
                    }
                } else {
                    value
                }
            }
        }
    }
}

#[inline]
pub fn pop_stack_value(
    state: &mut State,
    stack: &mut Vec<StackItem>,
    set_size: bool,
    sign_ext: bool,
) -> Value {
    let item = stack.pop().unwrap_or_default();

    let value = match item {
        StackItem::StackValue(val) => val,
        StackItem::StackRegister(index) => {
            if set_size {
                let reg = state.registers.indexes.get(index).unwrap();
                state.esil.last_sz = reg.reg_info.size as usize;
            }
            state.registers.get_value(index)
        }
    };

    match &value {
        Value::Concrete(_v, _t) => value,
        Value::Symbolic(ov, t) => {
            if ov.is_const() && ov.get_width() <= 64 {
                Value::Concrete(ov.as_u64().unwrap(), *t)
            } else {
                let v = ov; //state.translate(&ov).unwrap();
                let szdiff = SIZE as u32 - v.get_width();
                if szdiff > 0 {
                    if sign_ext {
                        Value::Symbolic(v.sext(szdiff), *t)
                    } else {
                        Value::Symbolic(v.uext(szdiff), *t)
                    }
                } else {
                    value
                }
            }
        }
    }
}

#[inline]
pub fn push_value(state: &mut State, value: Value) {
    state.stack.push(StackItem::StackValue(value));
}

#[inline]
pub fn pop_concrete(state: &mut State, set_size: bool, sign_ext: bool) -> u64 {
    let value = pop_value(state, set_size, sign_ext);

    match &value {
        Value::Concrete(val, _t) => *val,
        Value::Symbolic(_val, _t) => state.solver.evalcon_to_u64(&value).unwrap(),
    }
}

#[inline]
pub fn get_stack_taint(state: &mut State, n: usize) -> u64 {
    let mut taint = 0;
    for _ in 0..n {
        let arg = pop_value(state, false, false);
        taint |= arg.get_taint();
        push_value(state, arg);
    }
    taint
}

#[inline]
pub fn pop_double(state: &mut State) -> f64 {
    let value = pop_concrete(state, false, false);
    f64::from_bits(value)
}

#[inline]
pub fn do_equal(state: &mut State, reg: StackItem, value: Value, set_esil: bool) {
    if let StackItem::StackRegister(index) = reg {
        let register = state.registers.indexes.get(index).unwrap();
        let size = register.reg_info.size as usize;
        let prev = state.registers.get_value(index);

        // Zero-extend the value if it's smaller than the register size
        let extended_value = {
            let value_width = value.size() as usize;
            if value_width < size {
                // Zero-extend value to register size
                match &value {
                    Value::Symbolic(bv, taint) => {
                        Value::Symbolic(bv.uext((size - value_width) as u32), *taint)
                    },
                    Value::Concrete(val, taint) => {
                        // Mask concrete value to ensure it fits in the register size
                        let mask = if size >= 64 {
                            u64::MAX
                        } else {
                            (1u64 << size) - 1
                        };
                        Value::Concrete(val & mask, *taint)
                    }
                }
            } else {
                value.to_owned()
            }
        };

        if let Some(cond) = &state.condition {
            state.registers.set_value(
                index,
                state
                    .solver
                    .conditional(&Value::Symbolic(cond.to_owned(), 0), &extended_value, &prev),
            );
        } else {
            state.registers.set_value(index, extended_value);
        }

        if set_esil {
            state.esil.last_sz = size;
            state.esil.current = value;
            state.esil.previous = prev;
        }
    }
}

macro_rules! binary_operation {
    ($state:expr, $op:tt) => {
        let arg1 = pop_value($state, false, false);
        let arg2 = pop_value($state, false, false);
        push_value($state, arg1 $op arg2);
    };
}

// dst is on top of the stack and src below it, like r2's float ops
macro_rules! binary_float_operation {
    ($state:expr, |$d:ident, $s:ident| $res:expr) => {
        let t = get_stack_taint($state, 2);
        let $d = pop_double($state);
        let $s = pop_double($state);
        let value = Value::Concrete(f64::to_bits(float_nan_or($s, $d, $res)), t);
        push_value($state, value);
    };
}

macro_rules! float_compare {
    ($state:expr, |$d:ident, $s:ident| $res:expr) => {
        let t = get_stack_taint($state, 2);
        let $d = pop_double($state);
        let $s = pop_double($state);
        push_value($state, Value::Concrete($res as u64, t));
    };
}

#[inline]
pub fn genmask(bits: u64) -> u64 {
    if bits > 0 && bits < 63 {
        (2u64 << bits) - 1
    } else {
        0xffffffffffffffff
    }
}

pub fn do_operation(state: &mut State, operation: &Operations) {
    match operation {
        Operations::Trap => {}
        Operations::Interrupt => {}
        Operations::Syscall => {}
        Operations::PcAddress => {
            push_value(state, state.esil.prev_pc.clone());
        }
        Operations::If => {} // these are handled in processor
        Operations::Else => {}
        Operations::EndIf => {}
        Operations::Compare => {
            pop_compare_args(state);
        }
        // signed comparisons at the size of the register operands, 'src,dst,<' is dst < src
        Operations::LessThan => {
            let (dst, src, sz) = pop_compare_args(state);
            let res = signed_sized(dst, sz).slt(&signed_sized(src, sz));
            push_value(state, res);
        }
        Operations::LessThanEq => {
            let (dst, src, sz) = pop_compare_args(state);
            let res = signed_sized(dst, sz).slte(&signed_sized(src, sz));
            push_value(state, res);
        }
        Operations::GreaterThan => {
            let (dst, src, sz) = pop_compare_args(state);
            let res = signed_sized(dst, sz).sgt(&signed_sized(src, sz));
            push_value(state, res);
        }
        Operations::GreaterThanEq => {
            let (dst, src, sz) = pop_compare_args(state);
            let res = signed_sized(dst, sz).sgte(&signed_sized(src, sz));
            push_value(state, res);
        }
        Operations::UnsignedLessThan => {
            let arg1 = pop_value(state, true, true);
            let arg2 = pop_value(state, false, true);
            push_value(state, arg1.ult(&arg2));
        }
        Operations::UnsignedLessThanEq => {
            let arg1 = pop_value(state, true, true);
            let arg2 = pop_value(state, false, true);
            push_value(state, arg1.ulte(&arg2));
        }
        Operations::UnsignedGreaterThan => {
            let arg1 = pop_value(state, true, true);
            let arg2 = pop_value(state, false, true);
            push_value(state, arg1.ugt(&arg2));
        }
        Operations::UnsignedGreaterThanEq => {
            let arg1 = pop_value(state, true, true);
            let arg2 = pop_value(state, false, true);
            push_value(state, arg1.ugte(&arg2));
        }
        Operations::LeftShift => {
            binary_operation!(state, <<);
        }
        Operations::LogicalRightShift => {
            binary_operation!(state, >>);
        }
        Operations::RightShift => {
            let is_reg = matches!(state.stack.last(), Some(StackItem::StackRegister(_)));
            let sz = match get_size(state) {
                sz if is_reg || sz == 32 => sz,
                _ => 64,
            };
            let arg1 = pop_value(state, false, true);
            let arg2 = pop_value(state, false, true);
            push_value(state, arg1.asr(arg2, sz));
        }
        Operations::LeftRotation => {
            let sz = get_size(state);
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            push_value(state, arg1.rol(arg2, sz));
        }
        Operations::RightRotation => {
            let sz = get_size(state);
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            push_value(state, arg1.ror(arg2, sz));
        }
        Operations::SignExtend => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_concrete(state, false, false);

            let val = match arg1 {
                _ if arg2 > 63 => arg1,
                Value::Concrete(_, t) | Value::Symbolic(_, t) if arg2 == 0 => Value::Concrete(0, t),
                Value::Concrete(val1, t) => {
                    let shift = 64 - arg2;
                    Value::Concrete(((val1 << shift) as i64 >> shift) as u64, t)
                }
                Value::Symbolic(val1, t) => {
                    Value::Symbolic(val1.slice((arg2 - 1) as u32, 0).sext(64 - arg2 as u32), t)
                }
            };
            push_value(state, val);
        }
        Operations::And => {
            binary_operation!(state, &);
        }
        Operations::Or => {
            binary_operation!(state, |);
        }
        Operations::Xor => {
            binary_operation!(state, ^);
        }
        Operations::Add => {
            binary_operation!(state, +);
        }
        Operations::Subtract => {
            binary_operation!(state, -);
        }
        Operations::Multiply => {
            binary_operation!(state, *);
        }
        // here, unlike anywhere else, long means 128 bit
        // it should be long long long long multiply
        Operations::LongMultiply => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);

            match (arg1, arg2) {
                (Value::Concrete(val1, t1), Value::Concrete(val2, t2)) => {
                    let val = (val1 as u128) * (val2 as u128);
                    push_value(state, Value::Concrete((val >> 64) as u64, t1 | t2));
                    push_value(state, Value::Concrete(val as u64, t1 | t2));
                }
                (Value::Symbolic(val1, t1), Value::Concrete(val2, t2)) => {
                    let sval1 = val1.uext(64);
                    let sval2 = state.bvv(val2, 128);
                    let prod = sval1.mul(&sval2);
                    push_value(state, Value::Symbolic(prod.slice(127, 64), t1 | t2));
                    push_value(state, Value::Symbolic(prod.slice(63, 0), t1 | t2));
                }
                (Value::Concrete(val1, t1), Value::Symbolic(val2, t2)) => {
                    let sval2 = val2.uext(64);
                    let sval1 = state.bvv(val1, 128);
                    let prod = sval1.mul(&sval2);
                    push_value(state, Value::Symbolic(prod.slice(127, 64), t1 | t2));
                    push_value(state, Value::Symbolic(prod.slice(63, 0), t1 | t2));
                }
                (Value::Symbolic(val1, t1), Value::Symbolic(val2, t2)) => {
                    let sval1 = val1.uext(64);
                    let sval2 = val2.uext(64);
                    let prod = sval1.mul(&sval2);
                    push_value(state, Value::Symbolic(prod.slice(127, 64), t1 | t2));
                    push_value(state, Value::Symbolic(prod.slice(63, 0), t1 | t2));
                }
            }
        }
        Operations::LongDivide => {
            let arg1 = pop_bv(state, 128);
            let arg2 = pop_bv(state, 128);
            let arg3 = pop_bv(state, 128);

            push_value(state, ((arg2 << vc(64)) + arg1) / arg3);
        }
        Operations::LongModulo => {
            let arg1 = pop_bv(state, 128);
            let arg2 = pop_bv(state, 128);
            let arg3 = pop_bv(state, 128);

            //println!("{:?} {:?} {:?}", arg1, arg2, arg3);
            push_value(state, ((arg2 << vc(64)) + arg1) % arg3);
        }
        // division and modulo by zero push 0
        Operations::Divide => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            let res = div_by_zero_checked(state, arg1, arg2, |a, b| a / b);
            push_value(state, res);
        }
        Operations::Modulo => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            let res = div_by_zero_checked(state, arg1, arg2, |a, b| a % b);
            push_value(state, res);
        }
        Operations::SignedDivide => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            let res = div_by_zero_checked(state, arg1, arg2, Value::sdiv);
            push_value(state, res);
        }
        Operations::SignedModulo => {
            let arg1 = pop_value(state, false, false);
            let arg2 = pop_value(state, false, false);
            let res = div_by_zero_checked(state, arg1, arg2, Value::srem);
            push_value(state, res);
        }
        Operations::Not => {
            let arg1 = pop_value(state, false, false);
            push_value(state, !arg1);
        }
        Operations::Increment => {
            let arg1 = pop_value(state, false, false);
            push_value(state, arg1 + Value::Concrete(1, 0));
        }
        Operations::Decrement => {
            let arg1 = pop_value(state, false, false);
            push_value(state, arg1 - Value::Concrete(1, 0));
        }
        Operations::Equal => {
            let reg_arg = state.stack.pop().unwrap();
            let value = pop_value(state, false, false);
            do_equal(state, reg_arg, value, true);
        }
        Operations::WeakEqual => {
            let reg_arg = state.stack.pop().unwrap();
            let value = pop_value(state, false, false);
            do_equal(state, reg_arg, value, false);
        }
        Operations::Peek(n) => {
            let addr = pop_value(state, false, false);
            do_peek(state, addr, *n);
        }
        Operations::Poke(n) => {
            let addr = pop_value(state, false, false);
            let value = pop_value(state, false, false);
            do_poke(state, addr, value, *n);
        }
        Operations::PeekBits => {
            let n = (state.memory.bits / 8) as usize;
            let addr = pop_value(state, false, false);
            do_peek(state, addr, n);
        }
        Operations::PokeBits => {
            let n = (state.memory.bits / 8) as usize;
            let addr = pop_value(state, false, false);
            let value = pop_value(state, false, false);
            do_poke(state, addr, value, n);
        }
        Operations::PeekSize => {
            let n = pop_concrete(state, false, false) as usize;
            let addr = pop_value(state, false, false);
            do_peek(state, addr, n);
        }
        Operations::PokeSize => {
            let n = pop_concrete(state, false, false) as usize;
            let addr = pop_value(state, false, false);
            let value = pop_value(state, false, false);
            do_poke(state, addr, value, n);
        }
        Operations::PeekMany => {
            let mut addr = pop_value(state, false, false);
            let num = pop_concrete(state, false, false);

            for _ in 0..num {
                if let Some(StackItem::StackRegister(ind)) = state.stack.pop() {
                    let reg = state.registers.indexes[ind].clone();
                    let val = state.memory_read_value(&addr, reg.reg_info.size as usize / 8);
                    do_equal(state, StackItem::StackRegister(ind), val, false);
                    addr = addr + vc(reg.reg_info.size);
                }
            }
        }
        Operations::PokeMany => {
            let mut addr = pop_value(state, false, false);
            let num = pop_concrete(state, false, false);

            for _ in 0..num {
                if let Some(StackItem::StackRegister(ind)) = state.stack.pop() {
                    let reg = state.registers.indexes[ind].clone();
                    let val = state.registers.get_value(ind);
                    state.memory_write_value(&addr, &val, reg.reg_info.size as usize / 8);
                    addr = addr + vc(reg.reg_info.size);
                }
            }
        }
        // this is a hack to do op pokes ~efficiently
        Operations::AddressStore => {
            let addr = pop_value(state, false, false);
            state.esil.stored_address = Some(addr.to_owned());
            push_value(state, addr);
        }
        Operations::AddressRestore => {
            let addr = state.esil.stored_address.as_ref().unwrap().to_owned();
            push_value(state, addr);
            state.esil.stored_address = None;
        }
        Operations::PopCount => {
            let arg1 = pop_value(state, false, false);
            match arg1 {
                Value::Concrete(val, t) => {
                    let value = Value::Concrete(val.count_ones() as u64, t);
                    push_value(state, value);
                }
                Value::Symbolic(val, t) => {
                    let mut sym_val = state.bvv(0, 64);
                    for i in 0..val.get_width() {
                        sym_val = sym_val.add(&val.slice(i, i).uext(63));
                    }
                    push_value(state, Value::Symbolic(sym_val, t));
                }
            }
        }
        // 'w,x,CLZ' leading zeros of x in its low w bits, w if x is 0
        Operations::CountLeadingZeros => {
            let arg1 = pop_value(state, false, false);
            let width = pop_concrete(state, false, false).min(64);
            let mask = if width >= 64 { u64::MAX } else { (1 << width) - 1 };
            match arg1 & vc(mask) {
                Value::Concrete(val, t) => {
                    let clz = if val != 0 {
                        val.leading_zeros() as u64 - (64 - width)
                    } else {
                        width
                    };
                    push_value(state, Value::Concrete(clz, t));
                }
                Value::Symbolic(val, t) => {
                    // the highest set bit wins as it is checked last
                    let mut clz = state.bvv(width, 64);
                    for i in 0..width as u32 {
                        let bit_clz = state.bvv(width - 1 - i as u64, 64);
                        clz = val.slice(i, i).cond_bv(&bit_clz, &clz);
                    }
                    push_value(state, Value::Symbolic(clz, t));
                }
            }
        }
        Operations::Ceiling => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(f64::to_bits(arg1.ceil()), t));
        }
        Operations::Floor => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(f64::to_bits(arg1.floor()), t));
        }
        Operations::Round => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(f64::to_bits(arg1.round()), t));
        }
        Operations::SquareRoot => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(f64::to_bits(arg1.sqrt()), t));
        }
        Operations::DoubleToInt => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            let value = if arg1 > i64::MIN as f64 && arg1 < i64::MAX as f64 {
                arg1 as i64 as u64
            } else {
                0
            };
            push_value(state, Value::Concrete(value, t));
        }
        Operations::SignedToDouble => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_concrete(state, false, true);
            let value = Value::Concrete(f64::to_bits(arg1 as i64 as f64), t);
            push_value(state, value);
        }
        Operations::UnsignedToDouble => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_concrete(state, false, false);
            push_value(state, Value::Concrete(f64::to_bits(arg1 as f64), t));
        }
        // 'size,value,F2D' and 'size,value,D2F'
        Operations::FloatToDouble => {
            let val = pop_value(state, false, false);
            let size = pop_concrete(state, false, false);
            let t = val.get_taint();
            push_value(state, val);
            let raw = pop_concrete(state, false, false);

            let value = match size {
                32 => Value::Concrete(f64::to_bits(f32::from_bits(raw as u32) as f64), t),
                16 => Value::Concrete(f64::to_bits(half_to_f64(raw as u16)), t),
                _ => Value::Concrete(raw, t),
            };
            push_value(state, value);
        }
        Operations::DoubleToFloat => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            let size = pop_concrete(state, false, false);

            let value = match size {
                32 => f32::to_bits(arg1 as f32) as u64,
                16 => f64_to_half(arg1) as u64,
                _ => f64::to_bits(arg1),
            };
            push_value(state, Value::Concrete(value, t));
        }
        Operations::FloatAdd => {
            binary_float_operation!(state, |d, s| s + d);
        }
        Operations::FloatSubtract => {
            binary_float_operation!(state, |d, s| d - s);
        }
        Operations::FloatMultiply => {
            binary_float_operation!(state, |d, s| s * d);
        }
        Operations::FloatDivide => {
            binary_float_operation!(state, |d, s| d / s);
        }
        // 'a,n,m,F*+' is m * n + a
        Operations::FloatMultiplyAdd => {
            let t = get_stack_taint(state, 3);
            let m = pop_double(state);
            let n = pop_double(state);
            let a = pop_double(state);
            let res = if a.is_nan() {
                a
            } else {
                float_nan_or(m, n, m.mul_add(n, a))
            };
            push_value(state, Value::Concrete(f64::to_bits(res), t));
        }
        Operations::FloatCompare => {
            float_compare!(state, |d, s| s == d);
        }
        Operations::FloatNotEqual => {
            float_compare!(state, |d, s| s != d);
        }
        Operations::FloatLessThan => {
            float_compare!(state, |d, s| d < s);
        }
        Operations::FloatLessThanEq => {
            float_compare!(state, |d, s| d <= s);
        }
        Operations::NaN => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(arg1.is_nan() as u64, t));
        }
        Operations::FloatNegate => {
            let t = get_stack_taint(state, 1);
            let arg1 = pop_double(state);
            push_value(state, Value::Concrete(f64::to_bits(-arg1), t));
        }
        Operations::Swap => {
            let arg1 = state.stack.pop().unwrap();
            let arg2 = state.stack.pop().unwrap();
            state.stack.push(arg1);
            state.stack.push(arg2);
        }
        // rotate the third element to the top: [x,y,z] -> [y,z,x]
        Operations::Rotate => {
            let len = state.stack.len();
            if len >= 3 {
                let item = state.stack.remove(len - 3);
                state.stack.push(item);
            }
        }
        Operations::Pick => {
            let n = pop_concrete(state, false, false);
            let item = state.stack[state.stack.len() - n as usize].to_owned();
            state.stack.push(item);
        }
        Operations::ReversePick => {
            let n = pop_concrete(state, false, false);
            let item = state.stack[n as usize].to_owned();
            state.stack.push(item);
        }
        Operations::Pop => {
            state.stack.pop();
        }
        Operations::Duplicate => {
            let item = state.stack.pop().unwrap();
            state.stack.push(item.clone());
            state.stack.push(item);
        }
        Operations::Number => {
            let value = pop_value(state, false, false);
            push_value(state, value);
        }
        Operations::Clear => {
            state.stack.clear();
        }
        Operations::PrintStack => {
            for value in state.stack.iter().rev() {
                println!("{:?}", value);
            }
        }
        Operations::Break => {}
        Operations::Repeat => {}
        Operations::GoTo => {}
        Operations::NoOperation => {}
        // these may run under a symbolic condition, like register writes
        Operations::SetJumpTarget => {
            let target = pop_value(state, false, false);
            let prev = state.esil.jump_target.to_owned();
            state.esil.jump_target = conditional_set(state, target, prev);
            let prev = state.esil.jump_target_set.to_owned();
            state.esil.jump_target_set = conditional_set(state, vc(1), prev);
        }
        Operations::SetJumpTargetSet => {
            let value = pop_value(state, false, false);
            let prev = state.esil.jump_target_set.to_owned();
            state.esil.jump_target_set = conditional_set(state, value, prev);
        }
        Operations::SetDelay => {
            let value = pop_value(state, false, false);
            let prev = state.esil.delay.to_owned();
            state.esil.delay = conditional_set(state, value, prev);
        }
        Operations::Bits | Operations::Command => {
            state.stack.pop();
        }
        Operations::RegisterAlias => {
            state.stack.pop();
            state.stack.pop();
        }

        Operations::Print => {
            let value = pop_value(state, false, false);
            if let Some(cond) = &state.condition {
                state.solver.push();
                let condition = cond.clone();
                state.assert_bv(&condition);
            }
            let ip = state.registers.get_pc().as_u64().unwrap_or_default();
            if let Some(bv) = state.solver.eval_to_bv(&value) {
                let hex = state.solver.hex_solution(&bv).unwrap();
                if let Some(string) = state.evaluate_string_bv(&bv) {
                    println!("\n0x{:08x}    0x{} {:?}\n", ip, hex, string);
                } else {
                    println!("\n0x{:08x}    0x{}\n", ip, hex);
                }
            } else {
                println!("\n0x{:08x}    unsat\n", ip);
            }
            if state.condition.is_some() {
                state.solver.pop();
            }
        }
        Operations::PrintDebug => {
            let value = pop_value(state, false, false);
            let ip = state.registers.get_pc().as_u64().unwrap_or_default();
            println!("\n0x{:08x}    {:?}\n", ip, value);
        }
        Operations::Backtrace => {
            state.print_backtrace();
        }
        Operations::Constrain => {
            let value = pop_value(state, false, false);
            state.assert(&value);
        }
        // im gettin tired of writing x,y,-,!,_
        Operations::ConstrainEqual => {
            let val1 = pop_value(state, false, false);
            let val2 = pop_value(state, false, false);
            state.assert(&val1.eq(&val2));
        }
        Operations::ConstraintPush => {
            state.solver.push();
        }
        Operations::ConstraintPop => {
            state.solver.pop();
        }
        Operations::Terminate => {
            if let Some(cond) = &state.condition {
                let cond_value = Value::Symbolic(cond.to_owned(), 0);
                if state.solver.check_sat(&cond_value) {
                    state.solver.assert(&cond_value);
                    state.set_break();
                }
            } else {
                state.set_break();
            }
        }
        Operations::Discard => {
            state.set_inactive();
        }
        Operations::ToDo => {
            if state.strict {
                //unimplemented!();
                state.set_inactive();
            }
        }

        Operations::Zero => {
            let cur = &state.esil.current;
            let mask = Value::Concrete(genmask(state.esil.last_sz.saturating_sub(1) as u64), 0);
            let zf = !(cur.and(&mask));
            push_value(state, zf);
        }
        Operations::Carry => {
            let bits = pop_concrete(state, false, false);
            let mask = Value::Concrete(genmask(bits & 0x3f), 0);
            let cur = &state.esil.current;
            let old = &state.esil.previous;

            let cf = cur.and(&mask).ult(&old.and(&mask));
            push_value(state, cf);
        }
        // borrow from bit x is checked at bit x-1 (mod 64)
        Operations::Borrow => {
            let bits = pop_concrete(state, false, false);
            let mask = Value::Concrete(genmask((bits + 0x3f) & 0x3f), 0);
            let cur = &state.esil.current;
            let old = &state.esil.previous;

            let cf = old.and(&mask).ult(&cur.and(&mask));
            push_value(state, cf);
        }
        Operations::Parity => match &state.esil.current {
            Value::Concrete(val, t) => {
                let pf = Value::Concrete(((val & 0xff).count_ones() % 2 == 0) as u64, *t);
                push_value(state, pf);
            }
            Value::Symbolic(_val, _t) => {
                let c1 = Value::Concrete(0x0101010101010101, 0);
                let c2 = Value::Concrete(0x8040201008040201, 0);
                let c3 = Value::Concrete(0x1ff, 0);

                let cur = state.esil.current.to_owned();
                let lsb = cur & Value::Concrete(0xff, 0);
                let pf = !((((lsb * c1) & c2) % c3) & Value::Concrete(1, 0));
                //let pf = Value::Symbolic(val.redxor(), *t); this is 2x slower wtf
                push_value(state, pf);
            }
        },
        Operations::Overflow => {
            let bits = pop_concrete(state, false, false);
            let mask1 = Value::Concrete(genmask(bits & 0x3f), 0);
            let mask2 = Value::Concrete(genmask((bits + 0x3f) & 0x3f), 0);

            let cur = &state.esil.current;
            let old = &state.esil.previous;

            let c_in = cur.and(&mask1).ult(&old.and(&mask1));
            let c_out = cur.and(&mask2).ult(&old.and(&mask2));
            let of = c_in ^ c_out;
            push_value(state, of);
        }
        Operations::S => {
            let size = pop_concrete(state, false, false);
            let value = if size > 63 {
                vc(0)
            } else {
                (state.esil.current.to_owned() >> vc(size)) & vc(1)
            };
            push_value(state, value);
        }
        Operations::Ds => {
            push_value(state, state.esil.delay.to_owned());
        }
        Operations::JumpTarget => {
            push_value(state, state.esil.jump_target.to_owned());
        }
        Operations::Js => {
            push_value(state, state.esil.jump_target_set.to_owned());
        }
        Operations::R => {
            push_value(state, Value::Concrete(64 >> 3, 0));
        }
        Operations::Unknown => {
            if state.strict {
                // still panic here, we should support all ESIL
                panic!("Encountered an unknown word!");
            }
        }
    }
}
