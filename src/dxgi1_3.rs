// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_3.h

ENUM!{ enum DXGI_FRAME_PRESENTATION_MODE {
    DXGI_FRAME_PRESENTATION_MODE_COMPOSED = 0,
    DXGI_FRAME_PRESENTATION_MODE_OVERLAY = 1,
    DXGI_FRAME_PRESENTATION_MODE_NONE = 2,
    DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE = 3,
}}

FLAGS!{ enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE = 0x1,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709 = 0x2,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC = 0x4,
}}

FLAGS!{ enum DXGI_OVERLAY_SUPPORT_FLAG {
    DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1,
    DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2,
}}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: ::UINT,
    pub PresentRefreshCount: ::UINT,
    pub SyncRefreshCount: ::UINT,
    pub SyncQPCTime: ::LARGE_INTEGER,
    pub SyncGPUTime: ::LARGE_INTEGER,
    pub CompositionMode: ::DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: ::UINT,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: ::FLOAT,
    pub _12: ::FLOAT,
    pub _21: ::FLOAT,
    pub _22: ::FLOAT,
    pub _31: ::FLOAT,
    pub _32: ::FLOAT,
}

RIDL!(
interface IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVtbl): IUnknown(IUnknownVtbl) {
    fn PresentBuffer(
        &mut self, BufferToPresent: ::UINT, SyncInterval: ::UINT, Flags: ::UINT
    ) -> ::HRESULT,
    fn SetSourceRect(&mut self, pRect: *const ::RECT) -> ::HRESULT,
    fn SetTargetRect(&mut self, pRect: *const ::RECT) -> ::HRESULT,
    fn SetDestSize(&mut self, Width: ::UINT, Height: ::UINT) -> ::HRESULT,
    fn GetSourceRect(&mut self, pRect: *mut ::RECT) -> ::HRESULT,
    fn GetTargetRect(&mut self, pRect: *mut ::RECT) -> ::HRESULT,
    fn GetDestSize(
        &mut self, pWidth: *mut ::UINT, pHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn SetColorSpace(
        &mut self, ColorSpace: ::DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
    ) -> ::HRESULT,
    fn GetColorSpace(&mut self) -> ::DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
});

RIDL!(
interface IDXGIDevice3(IDXGIDevice3Vtbl): IDXGIDevice2(IDXGIDevice2Vtbl) {
    fn Trim(&mut self) -> ()
});

RIDL!(
interface IDXGIFactory3(IDXGIFactory3Vtbl): IDXGIFactory2(IDXGIFactory2Vtbl) {
    fn GetCreationFlags(&mut self) -> ::UINT
});

RIDL!(
interface IDXGIFactoryMedia(IDXGIFactoryMediaVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSwapChainForCompositionSurfaceHandle(
        &mut self, pDevice: *mut ::IUnknown, hSurface: ::HANDLE,
        pDesc: *const ::DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut ::IDXGIOutput,
        ppSwapChain: *mut *mut ::IDXGISwapChain1
    ) -> ::HRESULT,
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(
        &mut self, pDevice: *mut ::IUnknown, hSurface: ::HANDLE,
        pDesc: *mut ::DXGI_DECODE_SWAP_CHAIN_DESC, pYuvDecodeBuffers: *mut ::IDXGIResource,
        pRestrictToOutput: *mut ::IDXGIOutput, ppSwapChain: *mut *mut ::IDXGIDecodeSwapChain
    ) -> ::HRESULT
});

RIDL!(
interface IDXGIOutput2(IDXGIOutput2Vtbl): IDXGIOutput1(IDXGIOutput1Vtbl) {
    fn SupportsOverlays(&mut self) -> ::BOOL
});

RIDL!(
interface IDXGIOutput3(IDXGIOutput3Vtbl): IDXGIOutput2(IDXGIOutput2Vtbl) {
    fn CheckOverlaySupport(
        &mut self, EnumFormat: ::DXGI_FORMAT, pConcernedDevice: *mut ::IUnknown,
        pFlags: *mut ::UINT
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChain2(IDXGISwapChain2Vtbl): IDXGISwapChain1(IDXGISwapChain1Vtbl) {
    fn SetSourceSize(&mut self, Width: ::UINT, Height: ::UINT) -> ::HRESULT,
    fn GetSourceSize(
        &mut self, pWidth: *mut ::UINT, pHeight: *mut ::UINT
    ) -> ::HRESULT,
    fn SetMaximumFrameLatency(&mut self, MaxLatency: ::UINT) -> ::HRESULT,
    fn GetMaximumFrameLatency(&mut self, pMaxLatency: *mut ::UINT) -> ::HRESULT,
    fn GetFrameLatencyWaitableObject(&mut self) -> ::HANDLE,
    fn SetMatrixTransform(
        &mut self, pMatrix: *const ::DXGI_MATRIX_3X2_F
    ) -> ::HRESULT,
    fn GetMatrixTransform(
        &mut self, pMatrix: *mut ::DXGI_MATRIX_3X2_F
    ) -> ::HRESULT
});

RIDL!(
interface IDXGISwapChainMedia(IDXGISwapChainMediaVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrameStatisticsMedia(
        &mut self, pStats: *mut ::DXGI_FRAME_STATISTICS_MEDIA
    ) -> ::HRESULT,
    fn SetPresentDuration(&mut self, Duration: ::UINT) -> ::HRESULT,
    fn CheckPresentDurationSupport(
        &mut self, DesiredPresentDuration: ::UINT, pClosestSmallerPresentDuration: *mut ::UINT,
        pClosestLargerPresentDuration: *mut ::UINT
    ) -> ::HRESULT
});

pub const DXGI_CREATE_FACTORY_DEBUG: ::UINT = 0x1;
