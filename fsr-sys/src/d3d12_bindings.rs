/* automatically generated by rust-bindgen 0.68.1 */

extern "C" {
    #[doc = " Query how much memory is required for the DirectX 12 backend's scratch buffer.\n\n @returns\n The size (in bytes) of the required scratch memory buffer for the DX12 backend."]
    #[link_name = "ffxFsr2GetScratchMemorySizeDX12"]
    pub fn GetScratchMemorySizeDX12() -> usize;
}
extern "C" {
    #[doc = " Populate an interface with pointers for the DX12 backend.\n\n @param [out] fsr2Interface              A pointer to a <c><i>FfxFsr2Interface</i></c> structure to populate with pointers.\n @param [in] device                      A pointer to the DirectX12 device.\n @param [in] scratchBuffer               A pointer to a buffer of memory which can be used by the DirectX(R)12 backend.\n @param [in] scratchBufferSize           The size (in bytes) of the buffer pointed to by <c><i>scratchBuffer</i></c>.\n\n @retval\n FFX_OK                                  The operation completed successfully.\n @retval\n FFX_ERROR_CODE_INVALID_POINTER          The <c><i>interface</i></c> pointer was <c><i>NULL</i></c>.\n\n @ingroup FSR2 DX12"]
    #[link_name = "ffxFsr2GetInterfaceDX12"]
    pub fn GetInterfaceDX12(
        fsr2Interface: *mut Interface,
        device: *mut ID3D12Device,
        scratchBuffer: *mut ::std::os::raw::c_void,
        scratchBufferSize: usize,
    ) -> ErrorCode;
}
extern "C" {
    #[doc = " Create a <c><i>FfxFsr2Device</i></c> from a <c><i>ID3D12Device</i></c>.\n\n @param [in] device                      A pointer to the DirectX12 device.\n\n @returns\n An abstract FidelityFX device.\n\n @ingroup FSR2 DX12"]
    #[link_name = "ffxGetDeviceDX12"]
    pub fn GetDeviceDX12(device: *mut ID3D12Device) -> Device;
}
extern "C" {
    #[doc = " Create a <c><i>FfxCommandList</i></c> from a <c><i>ID3D12CommandList</i></c>.\n\n @param [in] cmdList                     A pointer to the DirectX12 command list.\n\n @returns\n An abstract FidelityFX command list.\n\n @ingroup FSR2 DX12"]
    #[link_name = "ffxGetCommandListDX12"]
    pub fn GetCommandListDX12(cmdList: *mut ID3D12CommandList) -> CommandList;
}
extern "C" {
    #[doc = " Create a <c><i>FfxResource</i></c> from a <c><i>ID3D12Resource</i></c>.\n\n @param [in] fsr2Interface               A pointer to a <c><i>FfxFsr2Interface</i></c> structure.\n @param [in] resDx12                     A pointer to the DirectX12 resource.\n @param [in] name                        (optional) A name string to identify the resource in debug mode.\n @param [in] state                       The state the resource is currently in.\n @param [in] shaderComponentMapping      The shader component mapping.\n\n @returns\n An abstract FidelityFX resources.\n\n @ingroup FSR2 DX12"]
    #[link_name = "ffxGetResourceDX12"]
    pub fn GetResourceDX12(
        context: *mut Context,
        resDx12: *mut ID3D12Resource,
        name: *const widechar,
        state: ResourceStates,
        shaderComponentMapping: UINT,
    ) -> Resource;
}
extern "C" {
    #[doc = " Retrieve a <c><i>ID3D12Resource</i></c> pointer associated with a RESOURCE_IDENTIFIER.\n Used for debug purposes when blitting internal surfaces.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] resId                       A resourceID.\n\n @returns\n A <c><i>ID3D12Resource</i> pointer</c>.\n\n @ingroup FSR2 DX12"]
    #[link_name = "ffxGetDX12ResourcePtr"]
    pub fn GetDX12ResourcePtr(context: *mut Context, resId: u32) -> *mut ID3D12Resource;
}
