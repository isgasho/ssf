use super::context::Context;
use super::memory_buffer::MemoryBuffer;
use super::type_::*;
use super::utilities::*;
use super::value::*;
use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct Module {
    internal: LLVMModuleRef,
}

impl Module {
    pub fn add_function(&self, name: &str, function_type: Type) -> Value {
        unsafe { LLVMAddFunction(self.internal, c_string(name).as_ptr(), function_type.into()) }
            .into()
    }

    pub fn add_global(&self, name: &str, type_: Type) -> Value {
        unsafe { LLVMAddGlobal(self.internal, type_.into(), c_string(name).as_ptr()) }.into()
    }

    pub fn declare_function(&self, name: &str, return_type: Type, arguments: &[Type]) {
        self.add_function(name, self.context().function_type(return_type, arguments));
    }

    pub fn declare_intrinsics(&self) {
        self.declare_function(
            "malloc",
            self.context().generic_pointer_type(),
            &[self.context().i64_type()],
        );
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<Type> {
        let type_ = unsafe { LLVMGetTypeByName(self.internal, c_string(name).as_ptr()) };

        if type_.is_null() {
            None
        } else {
            Some(type_.into())
        }
    }

    fn context(&self) -> Context {
        unsafe { LLVMGetModuleContext(self.internal) }.into()
    }

    pub fn to_bitcode(&self) -> Vec<u8> {
        MemoryBuffer::new(unsafe { LLVMWriteBitcodeToMemoryBuffer(self.internal) })
            .as_bytes()
            .to_vec()
    }

    pub fn from_bitcode(bitcode: &[u8]) -> Self {
        MemoryBuffer::from(bitcode).get_bitcode_module()
    }

    pub fn verify(&self) {
        unsafe {
            LLVMVerifyModule(
                self.internal,
                LLVMVerifierFailureAction::LLVMAbortProcessAction,
                std::ptr::null_mut(),
            )
        };
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            unsafe { std::ffi::CString::from_raw(LLVMPrintModuleToString(self.internal)) }
                .to_str()
                .unwrap()
        )
    }
}

impl From<LLVMModuleRef> for Module {
    fn from(internal: LLVMModuleRef) -> Self {
        Self { internal }
    }
}