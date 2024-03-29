/* automatically generated by rust-bindgen 0.68.1 */

extern "C" {
    #[doc = " Query how much memory is required for the Vulkan backend's scratch buffer.\n\n @returns\n The size (in bytes) of the required scratch memory buffer for the VK backend."]
    #[link_name = "ffxFsr2GetScratchMemorySizeVK"]
    pub fn GetScratchMemorySizeVK(
        physicalDevice: VkPhysicalDevice,
        enumerateDeviceExtensionProperties: PFN_vkEnumerateDeviceExtensionProperties,
    ) -> usize;
}
extern "C" {
    #[doc = " Populate an interface with pointers for the VK backend.\n\n @param [out] fsr2Interface              A pointer to a <c><i>FfxFsr2Interface</i></c> structure to populate with pointers.\n @param [in] device                      A Vulkan device.\n @param [in] scratchBuffer               A pointer to a buffer of memory which can be used by the DirectX(R)12 backend.\n @param [in] scratchBufferSize           The size (in bytes) of the buffer pointed to by <c><i>scratchBuffer</i></c>.\n @param [in] instance                    A Vulkan instance. Provide a nullptr if you don't need to load function pointers from the instance directly.\n @param [in] physicalDevice              The Vulkan physical device that FSR 2.0 will be executed on.\n @param [in] getInstanceProcAddr         A function pointer to vkGetInstanceProcAddr which is used to obtain all the Vulkan instance functions.\n @param [in] getDeviceProcAddr           A function pointer to vkGetDeviceProcAddr which is used to obtain all the Vulkan device functions.\n\n @retval\n FFX_OK                                  The operation completed successfully.\n @retval\n FFX_ERROR_CODE_INVALID_POINTER          The <c><i>interface</i></c> pointer was <c><i>NULL</i></c>.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxFsr2GetInterfaceVK"]
    pub fn GetInterfaceVK(
        outInterface: *mut Interface,
        scratchBuffer: *mut ::std::os::raw::c_void,
        scratchBufferSize: usize,
        instance: VkInstance,
        physicalDevice: VkPhysicalDevice,
        getInstanceProcAddr: PFN_vkGetInstanceProcAddr,
        getDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    ) -> ErrorCode;
}
extern "C" {
    #[doc = " Create a <c><i>FfxFsr2Device</i></c> from a <c><i>VkDevice</i></c>.\n\n @param [in] device                      A pointer to the Vulkan logical device.\n\n @returns\n An abstract FidelityFX device.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetDeviceVK"]
    pub fn GetDeviceVK(device: VkDevice) -> Device;
}
extern "C" {
    #[doc = " Create a <c><i>FfxCommandList</i></c> from a <c><i>VkCommandBuffer</i></c>.\n\n @param [in] cmdBuf                      A pointer to the Vulkan command buffer.\n\n @returns\n An abstract FidelityFX command list.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetCommandListVK"]
    pub fn GetCommandListVK(cmdBuf: VkCommandBuffer) -> CommandList;
}
extern "C" {
    #[doc = " Create a <c><i>FfxResource</i></c> from a <c><i>VkImage</i></c>.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] imgVk                       A Vulkan image resource.\n @param [in] imageView                   An image view of the given image resource.\n @param [in] width                       The width of the image resource.\n @param [in] height                      The height of the image resource.\n @param [in] imgFormat                   The format of the image resource.\n @param [in] name                        (optional) A name string to identify the resource in debug mode.\n @param [in] state                       The state the resource is currently in.\n\n @returns\n An abstract FidelityFX resources.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetTextureResourceVK"]
    pub fn GetTextureResourceVK(
        context: *mut Context,
        imgVk: VkImage,
        imageView: VkImageView,
        width: u32,
        height: u32,
        imgFormat: VkFormat,
        name: *const widechar,
        state: ResourceStates,
    ) -> Resource;
}
extern "C" {
    #[doc = " Create a <c><i>FfxResource</i></c> from a <c><i>VkBuffer</i></c>.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] bufVk                       A Vulkan buffer resource.\n @param [in] size                        The size of the buffer resource.\n @param [in] name                        (optional) A name string to identify the resource in debug mode.\n @param [in] state                       The state the resource is currently in.\n\n @returns\n An abstract FidelityFX resources.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetBufferResourceVK"]
    pub fn GetBufferResourceVK(
        context: *mut Context,
        bufVk: VkBuffer,
        size: u32,
        name: *const widechar,
        state: ResourceStates,
    ) -> Resource;
}
extern "C" {
    #[doc = " Convert a <c><i>FfxResource</i></c> value to a <c><i>VkImage</i></c>.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] resId                       A resourceID.\n\n @returns\n A <c><i>VkImage</i></c>.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetVkImage"]
    pub fn GetVkImage(context: *mut Context, resId: u32) -> VkImage;
}
extern "C" {
    #[doc = " Convert a <c><i>FfxResource</i></c> value to a <c><i>VkImageView</i></c>.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] resId                       A resourceID.\n\n @returns\n A <c><i>VkImage</i></c>.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetVkImageView"]
    pub fn GetVkImageView(context: *mut Context, resId: u32) -> VkImageView;
}
extern "C" {
    #[doc = " Convert a <c><i>FfxResource</i></c> value to a <c><i>VkImageLayout</i></c>.\n\n @param [in] context                     A pointer to a <c><i>FfxFsr2Context</i></c> structure.\n @param [in] resId                       A resourceID.\n\n @returns\n A <c><i>VkImage</i></c>.\n\n @ingroup FSR2 VK"]
    #[link_name = "ffxGetVkImageLayout"]
    pub fn GetVkImageLayout(context: *mut Context, resId: u32) -> VkImageLayout;
}
