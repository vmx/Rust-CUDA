/* automatically generated by rust-bindgen 0.59.2 */

#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct float2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct double2 {
    pub x: f64,
    pub y: f64,
}
pub type cuFloatComplex = float2;
pub type cuDoubleComplex = double2;
pub type cuComplex = cuFloatComplex;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasStatus_t {
    CUBLAS_STATUS_SUCCESS = 0,
    CUBLAS_STATUS_NOT_INITIALIZED = 1,
    CUBLAS_STATUS_ALLOC_FAILED = 3,
    CUBLAS_STATUS_INVALID_VALUE = 7,
    CUBLAS_STATUS_ARCH_MISMATCH = 8,
    CUBLAS_STATUS_MAPPING_ERROR = 11,
    CUBLAS_STATUS_EXECUTION_FAILED = 13,
    CUBLAS_STATUS_INTERNAL_ERROR = 14,
    CUBLAS_STATUS_NOT_SUPPORTED = 15,
    CUBLAS_STATUS_LICENSE_ERROR = 16,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasFillMode_t {
    CUBLAS_FILL_MODE_LOWER = 0,
    CUBLAS_FILL_MODE_UPPER = 1,
    CUBLAS_FILL_MODE_FULL = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasDiagType_t {
    CUBLAS_DIAG_NON_UNIT = 0,
    CUBLAS_DIAG_UNIT = 1,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasSideMode_t {
    CUBLAS_SIDE_LEFT = 0,
    CUBLAS_SIDE_RIGHT = 1,
}
impl cublasOperation_t {
    pub const CUBLAS_OP_HERMITAN: cublasOperation_t = cublasOperation_t::CUBLAS_OP_C;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasOperation_t {
    CUBLAS_OP_N = 0,
    CUBLAS_OP_T = 1,
    CUBLAS_OP_C = 2,
    CUBLAS_OP_CONJG = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cublasXtContext {
    _unused: [u8; 0],
}
pub type cublasXtHandle_t = *mut cublasXtContext;
extern "C" {
    pub fn cublasXtCreate(handle: *mut cublasXtHandle_t) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDestroy(handle: cublasXtHandle_t) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtGetNumBoards(
        nbDevices: ::std::os::raw::c_int,
        deviceId: *mut ::std::os::raw::c_int,
        nbBoards: *mut ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtMaxBoards(nbGpuBoards: *mut ::std::os::raw::c_int) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDeviceSelect(
        handle: cublasXtHandle_t,
        nbDevices: ::std::os::raw::c_int,
        deviceId: *mut ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSetBlockDim(
        handle: cublasXtHandle_t,
        blockDim: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtGetBlockDim(
        handle: cublasXtHandle_t,
        blockDim: *mut ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasXtPinnedMemMode_t {
    CUBLASXT_PINNING_DISABLED = 0,
    CUBLASXT_PINNING_ENABLED = 1,
}
extern "C" {
    pub fn cublasXtGetPinningMemMode(
        handle: cublasXtHandle_t,
        mode: *mut cublasXtPinnedMemMode_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSetPinningMemMode(
        handle: cublasXtHandle_t,
        mode: cublasXtPinnedMemMode_t,
    ) -> cublasStatus_t;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasXtOpType_t {
    CUBLASXT_FLOAT = 0,
    CUBLASXT_DOUBLE = 1,
    CUBLASXT_COMPLEX = 2,
    CUBLASXT_DOUBLECOMPLEX = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cublasXtBlasOp_t {
    CUBLASXT_GEMM = 0,
    CUBLASXT_SYRK = 1,
    CUBLASXT_HERK = 2,
    CUBLASXT_SYMM = 3,
    CUBLASXT_HEMM = 4,
    CUBLASXT_TRSM = 5,
    CUBLASXT_SYR2K = 6,
    CUBLASXT_HER2K = 7,
    CUBLASXT_SPMM = 8,
    CUBLASXT_SYRKX = 9,
    CUBLASXT_HERKX = 10,
    CUBLASXT_TRMM = 11,
    CUBLASXT_ROUTINE_MAX = 12,
}
extern "C" {
    pub fn cublasXtSetCpuRoutine(
        handle: cublasXtHandle_t,
        blasOp: cublasXtBlasOp_t,
        type_: cublasXtOpType_t,
        blasFunctor: *mut ::std::os::raw::c_void,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSetCpuRatio(
        handle: cublasXtHandle_t,
        blasOp: cublasXtBlasOp_t,
        type_: cublasXtOpType_t,
        ratio: f32,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSgemm(
        handle: cublasXtHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: usize,
        n: usize,
        k: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *const f32,
        ldb: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDgemm(
        handle: cublasXtHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: usize,
        n: usize,
        k: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *const f64,
        ldb: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCgemm(
        handle: cublasXtHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: usize,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZgemm(
        handle: cublasXtHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: usize,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSsyrk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDsyrk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCsyrk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZsyrk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCherk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f32,
        A: *const cuComplex,
        lda: usize,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZherk(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f64,
        A: *const cuDoubleComplex,
        lda: usize,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSsyr2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *const f32,
        ldb: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDsyr2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *const f64,
        ldb: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCsyr2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZsyr2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCherkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZherkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtStrsm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *mut f32,
        ldb: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDtrsm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *mut f64,
        ldb: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCtrsm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *mut cuComplex,
        ldb: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZtrsm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *mut cuDoubleComplex,
        ldb: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSsymm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *const f32,
        ldb: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDsymm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *const f64,
        ldb: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCsymm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZsymm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtChemm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZhemm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSsyrkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *const f32,
        ldb: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDsyrkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *const f64,
        ldb: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCsyrkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZsyrkx(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCher2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        beta: *const f32,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZher2k(
        handle: cublasXtHandle_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        n: usize,
        k: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const f64,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtSspmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const f32,
        AP: *const f32,
        B: *const f32,
        ldb: usize,
        beta: *const f32,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDspmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const f64,
        AP: *const f64,
        B: *const f64,
        ldb: usize,
        beta: *const f64,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCspmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuComplex,
        AP: *const cuComplex,
        B: *const cuComplex,
        ldb: usize,
        beta: *const cuComplex,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZspmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        m: usize,
        n: usize,
        alpha: *const cuDoubleComplex,
        AP: *const cuDoubleComplex,
        B: *const cuDoubleComplex,
        ldb: usize,
        beta: *const cuDoubleComplex,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtStrmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const f32,
        A: *const f32,
        lda: usize,
        B: *const f32,
        ldb: usize,
        C: *mut f32,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtDtrmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const f64,
        A: *const f64,
        lda: usize,
        B: *const f64,
        ldb: usize,
        C: *mut f64,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtCtrmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const cuComplex,
        A: *const cuComplex,
        lda: usize,
        B: *const cuComplex,
        ldb: usize,
        C: *mut cuComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasXtZtrmm(
        handle: cublasXtHandle_t,
        side: cublasSideMode_t,
        uplo: cublasFillMode_t,
        trans: cublasOperation_t,
        diag: cublasDiagType_t,
        m: usize,
        n: usize,
        alpha: *const cuDoubleComplex,
        A: *const cuDoubleComplex,
        lda: usize,
        B: *const cuDoubleComplex,
        ldb: usize,
        C: *mut cuDoubleComplex,
        ldc: usize,
    ) -> cublasStatus_t;
}
