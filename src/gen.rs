//! Raw interface generated from C headers.
//!
//! Please avoid using these directly, except for enums and constants.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::{convert::From, fmt, mem};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use status::Result;

pub use self::{
    ZydisAddressFormat as AddressFormat, ZydisAddressWidth as AddressWidth,
    ZydisCPUFlag as CPUFlag, ZydisCPUFlagAction as CPUFlagAction, ZydisCPUFlags as CPUFlags,
    ZydisDecodedInstruction as Instruction, ZydisDecodedOperand as Operand,
    ZydisDecoderMode as DecoderMode, ZydisDecoratorType as DecoratorType,
    ZydisDisplacementFormat as DisplacementFormat, ZydisFormatterContext as FormatterContext,
    ZydisFormatterStyle as FormatterStyle, ZydisImmediateFormat as ImmediateFormat,
    ZydisMachineMode as MachineMode, ZydisMnemonic as Mnemonic, ZydisRegister as Register,
    ZydisRegisterClass as RegisterClass,
};

impl From<ZyanStatus> for Status {
    fn from(x: ZyanStatus) -> Status {
        unsafe { mem::transmute(x) }
    }
}

impl From<Status> for ZyanStatus {
    fn from(x: Status) -> ZyanStatus {
        unsafe { mem::transmute(x) }
    }
}

impl ZydisDecodedInstruction {
    /// Calculates the absolute address for the given instruction operand,
    /// using the given `address` as the address for this instruction.
    pub fn calc_absolute_addr(&self, address: u64, operand: &Operand) -> Result<u64> {
        unsafe {
            let mut addr = 0u64;
            check!(
                ZydisCalcAbsoluteAddress(self, operand, address, &mut addr),
                addr
            )
        }
    }

    /// Returns a mask of CPU-flags that match the given `action`.
    pub fn get_flags(&self, action: CPUFlagAction) -> Result<CPUFlags> {
        unsafe {
            let mut flags = mem::uninitialized();
            check!(
                ZydisGetAccessedFlagsByAction(self, action, &mut flags),
                flags
            )
        }
    }

    /// Returns a mask of CPU-flags that are read (tested) by this instruction.
    pub fn get_flags_read(&self) -> Result<CPUFlags> {
        unsafe {
            let mut flags = mem::uninitialized();
            check!(ZydisGetAccessedFlagsRead(self, &mut flags), flags)
        }
    }

    /// Returns a mask of CPU-flags that are written (modified, undefined) by
    /// this instruction.
    pub fn get_flags_written(&self) -> Result<CPUFlags> {
        unsafe {
            let mut flags = mem::uninitialized();
            check!(ZydisGetAccessedFlagsWritten(self, &mut flags), flags)
        }
    }
}

impl ZyanString {
    pub fn new(buffer: *mut u8, capacity: usize) -> Result<Self> {
        unsafe {
            let mut string = mem::uninitialized();
            check!(ZyanStringInitCustomBuffer(
                &mut string,
                buffer as *mut i8,
                capacity
            ))?;
            Ok(string)
        }
    }

    /// Appends the given string `s` to this buffer.
    ///
    /// Warning: The actual Rust `&str`ings are encoded in UTF-8 and aren't
    /// converted to any other encoding. They're simply copied, byte by
    /// byte, to the buffer. Therefore, the buffer should be interpreted as
    /// UTF-8 when later being printed.
    pub fn append<S: AsRef<str> + ?Sized>(&mut self, s: &S) -> Result<()> {
        unsafe {
            let bytes = s.as_ref().as_bytes();
            let mut source = mem::uninitialized();
            check!(ZyanStringViewInitEx(
                &mut source,
                bytes.as_ptr() as *const i8,
                bytes.len()
            )).and_then(|_| check!(ZyanStringAppend(self, &source)))
        }
    }
}

impl fmt::Write for ZyanString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.append(s).map_err(|_| fmt::Error)
    }
}
